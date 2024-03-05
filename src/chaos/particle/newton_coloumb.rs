use crate::chaos::data::*;
use crate::chaos::functions::{empty_into_iter, Integrator, OdeSolverTrait};
use crate::chaos::Particle;
use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::vec::IntoIter;
use std::vec::Vec;

pub trait Force {
    type Cause;
    type Params;
    fn zero() -> Self;
    fn single(p: &mut Self::Cause, params: &Self::Params);
    fn pairwise(p_i: &mut Self::Cause, p_j: &mut Self::Cause, params: &Self::Params) -> ChaosFloat;
    fn collision(p_i: &mut Self::Cause, other: &mut Option<Self::Cause>, params: &Self::Params);
}

pub trait NewtonState {
    type CartesianState;
    fn position(&self) -> Self::CartesianState;
    fn velocity(&self) -> Self::CartesianState;
    fn set_velocity(&mut self, vel: Self::CartesianState);
    fn scalar_mul_mut(cartesian_state: &mut Self::CartesianState, scalar: ChaosFloat);
    fn add_mut(cartesian_state: &mut Self::CartesianState, other: &Self::CartesianState);
    fn sub_mut(cartesian_state: &mut Self::CartesianState, other: &Self::CartesianState);
    fn cartesian_zero() -> Self::CartesianState;
    fn cartesian_norm(cartesian_state: &Self::CartesianState) -> ChaosFloat;
    fn from_position_and_velocity(
        position: Self::CartesianState,
        velocity: Self::CartesianState,
    ) -> Self;
    fn dot_product(s_i: &Self::CartesianState, s_j: &Self::CartesianState) -> ChaosFloat;
}

pub trait NewtonParams {
    fn mid_scale_factor(&self) -> ChaosFloat;
    fn large_scale_factor(&self) -> ChaosFloat;
}

pub trait CollisionParams {
    fn collision_factor(&self) -> ChaosFloat;
    fn perform_collision_detection(&self) -> bool;
}

pub trait IntegrationParams {
    fn integration_steps(&self) -> usize {
        10
    }
}

impl NewtonState for State4 {
    type CartesianState = State2;
    fn position(&self) -> State2 {
        self.fixed_rows::<2>(0).into()
    }
    fn velocity(&self) -> State2 {
        self.fixed_rows::<2>(2).into()
    }
    fn set_velocity(&mut self, vel: Self::CartesianState) {
        for i in 0..2 {
            self[2 + i] = vel[i];
        }
    }
    fn add_mut(cartesian_state: &mut Self::CartesianState, other: &Self::CartesianState) {
        cartesian_state.add_assign(other);
    }
    fn sub_mut(cartesian_state: &mut Self::CartesianState, other: &Self::CartesianState) {
        cartesian_state.sub_assign(other);
    }
    fn cartesian_zero() -> Self::CartesianState {
        Self::CartesianState::zeros()
    }
    fn scalar_mul_mut(cartesian_state: &mut Self::CartesianState, scalar: ChaosFloat) {
        cartesian_state.scale_mut(scalar);
    }
    fn cartesian_norm(cartesian_state: &Self::CartesianState) -> ChaosFloat {
        cartesian_state.norm()
    }
    fn from_position_and_velocity(
        position: Self::CartesianState,
        velocity: Self::CartesianState,
    ) -> Self {
        Self::from_row_slice(&[position[0], position[1], velocity[0], velocity[1]])
    }
    fn dot_product(s_i: &Self::CartesianState, s_j: &Self::CartesianState) -> ChaosFloat {
        s_i.dot(s_j)
    }
}
impl NewtonState for State6 {
    type CartesianState = State3;
    fn position(&self) -> State3 {
        self.fixed_rows::<3>(0).into()
    }
    fn velocity(&self) -> State3 {
        self.fixed_rows::<3>(3).into()
    }
    fn set_velocity(&mut self, vel: Self::CartesianState) {
        for i in 0..3 {
            self[3 + i] = vel[i];
        }
    }
    fn add_mut(cartesian_state: &mut Self::CartesianState, other: &Self::CartesianState) {
        cartesian_state.add_assign(other);
    }
    fn sub_mut(cartesian_state: &mut Self::CartesianState, other: &Self::CartesianState) {
        cartesian_state.sub_assign(other);
    }
    fn cartesian_zero() -> Self::CartesianState {
        Self::CartesianState::zeros()
    }
    fn scalar_mul_mut(cartesian_state: &mut Self::CartesianState, scalar: ChaosFloat) {
        cartesian_state.scale_mut(scalar);
    }
    fn cartesian_norm(cartesian_state: &Self::CartesianState) -> ChaosFloat {
        cartesian_state.norm()
    }
    fn from_position_and_velocity(
        position: Self::CartesianState,
        velocity: Self::CartesianState,
    ) -> Self {
        Self::from_row_slice(&[
            position[0],
            position[1],
            position[2],
            velocity[0],
            velocity[1],
            velocity[2],
        ])
    }
    fn dot_product(s_i: &Self::CartesianState, s_j: &Self::CartesianState) -> ChaosFloat {
        s_i.dot(s_j)
    }
}

#[derive(Clone, Debug)]
pub struct NewtonColoumb<
    A,
    V: NewtonState<CartesianState = A>,
    Param: NewtonParams + CollisionParams,
> {
    state: A,
    phantom_state: PhantomData<V>,
    phantom_conf: PhantomData<Param>,
}
impl<A: Clone, V: NewtonState<CartesianState = A>, Param: NewtonParams + CollisionParams>
    NewtonColoumb<A, V, Param>
{
    pub fn new(v: A) -> Self {
        Self {
            state: v,
            phantom_conf: PhantomData,
            phantom_state: PhantomData,
        }
    }
    pub fn get_state(&self) -> &A {
        &self.state
    }

    fn scalar_force_mid_long(
        distance: &f64,
        (mid, mass): (f64, f64),
        (mid_i, mass_i): (f64, f64),
        (mid_j, mass_j): (f64, f64),
    ) -> f64 {
        // positive means attraction
        // mid applies on mid-range distances - like Coloumb force on charges where same charges repell
        let mid = -mid * mid_i * mid_j / distance.powi(3); // mid-range
                                                           // mass applies over great distances - like gravity with only positive masses
        let mass = mass * mass_i * mass_j / distance.powi(2); // mass-distance
        mid + mass
    }

    fn collision_elastic(p_i: &mut Particle<V, Self>, p_j: &mut Particle<V, Self>, s: ChaosFloat) {
        // see physics stackexchange: Elastic collision 3d equation
        let n = {
            let mut n = Self::distance_i_to_j(p_i, p_j);
            let distance = V::cartesian_norm(&n);
            if distance == 0.0 {
                return;
            }
            V::scalar_mul_mut(&mut n, 1.0 / distance);
            n
        };
        let (v_i, v_j) = (p_i.get_state().velocity(), p_j.get_state().velocity());
        let vel_delta = {
            let mut vel_delta = v_i.clone();
            V::sub_mut(&mut vel_delta, &v_j);
            vel_delta
        };
        let impulse_magnitude = {
            let v_imp = V::dot_product(&n, &vel_delta);
            let m_eff = 1.0 / (1.0 / p_i.mass + 1.0 / p_j.mass); // mass > 0
            (1.0 + s.abs()) * m_eff * v_imp
        };
        let (mut vel_delta_i, mut vel_delta_j) = (n.clone(), n.clone());
        V::scalar_mul_mut(&mut vel_delta_i, -impulse_magnitude / p_i.mass);
        V::scalar_mul_mut(&mut vel_delta_j, impulse_magnitude / p_j.mass);
        V::add_mut(&mut vel_delta_i, &v_i);
        V::add_mut(&mut vel_delta_j, &v_j);
        p_i.state.set_velocity(vel_delta_i);
        p_j.state.set_velocity(vel_delta_j);
    }
    fn collision_inelastic(p_i: &mut Particle<V, Self>, p_j: &Particle<V, Self>) {
        // only p_i gets changed - it becomes the resulting particle
        let mut velocity = p_i.state.velocity();
        let m_i = p_i.mass;
        let m_j = p_j.mass;
        let total_mass = m_i + m_j;
        let m_i = m_i / total_mass;
        let m_j = m_j / total_mass;
        // weighted sum of velocities
        V::scalar_mul_mut(&mut velocity, m_i);
        let mut other_velocity = p_j.get_state().velocity();
        V::scalar_mul_mut(&mut other_velocity, m_j);
        V::add_mut(&mut velocity, &other_velocity);
        // weighted sum of positions
        let mut position = p_i.state.position();
        V::scalar_mul_mut(&mut position, m_i);
        let mut other_position = p_j.get_state().position();
        V::scalar_mul_mut(&mut other_position, m_j);
        V::add_mut(&mut position, &other_position);
        p_i.state = V::from_position_and_velocity(position, velocity);
        p_i.add_scalar_values(p_j);
        V::add_mut(&mut p_i.force.state, &p_j.force.state);
    }
    fn distance_i_to_j(p_i: &Particle<V, Self>, p_j: &Particle<V, Self>) -> A {
        let (s_i, s_j) = (p_i.get_state(), p_j.get_state());
        let mut pos_delta = s_j.position();
        V::sub_mut(&mut pos_delta, &s_i.position());
        pos_delta
    }
}

impl<A: Clone, V, Param> Force for NewtonColoumb<A, V, Param>
where
    V: NewtonState<CartesianState = A>,
    Param: NewtonParams + CollisionParams,
{
    type Cause = Particle<V, Self>;
    type Params = Param;
    fn zero() -> Self {
        Self::new(V::cartesian_zero())
    }
    fn single(_p: &mut Self::Cause, _params: &Param) {}
    fn pairwise(p_i: &mut Self::Cause, p_j: &mut Self::Cause, params: &Param) -> ChaosFloat {
        let mut pos_delta = Self::distance_i_to_j(p_i, p_j);
        let distance = V::cartesian_norm(&pos_delta);
        let params_mid_long = (params.mid_scale_factor(), params.large_scale_factor());
        let p_i_vals = (p_i.mid, p_i.mass);
        let p_j_vals = (p_j.mid, p_j.mass);
        let scalar = Self::scalar_force_mid_long(&distance, params_mid_long, p_i_vals, p_j_vals);
        V::scalar_mul_mut(&mut pos_delta, scalar);
        V::sub_mut(&mut p_j.force.state, &pos_delta);
        V::add_mut(&mut p_i.force.state, &pos_delta);
        distance
    }
    fn collision(
        p_i: &mut Self::Cause,
        other_particle: &mut Option<Self::Cause>,
        params: &Self::Params,
    ) {
        if let Some(p_j) = other_particle {
            let small_scale_scalar = params.collision_factor() * p_i.short * p_j.short;
            if small_scale_scalar > 0.0 {
                Self::collision_inelastic(p_i, p_j);
                *other_particle = None;
            } else if small_scale_scalar < 0.0 {
                Self::collision_elastic(p_i, p_j, small_scale_scalar);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParticleOdeSolver<V, I, S> {
    params: S,
    iterators: Vec<IntoIter<V>>,
    integrator: I,
    temp_collision_indices: BTreeMap<usize, BTreeSet<usize>>,
    integration_counter: usize,
}

impl<V, I, S> ParticleOdeSolver<V, I, S>
where
    S: IntegrationParams,
    I: Integrator<Output = V> + Default,
{
    pub fn new(params: S) -> Self {
        let integration_steps = params.integration_steps();
        let mut integrator = I::default();
        integrator.set_steps(integration_steps);
        // TODO set randomness from params as well
        Self {
            params,
            iterators: Vec::new(),
            integrator,
            temp_collision_indices: Default::default(),
            integration_counter: integration_steps + 1,
        }
    }
}
impl<V, I, S> ParticleOdeSolver<V, I, S>
where
    V: ValidStateCheck,
{
    fn set_states_from_integration_results<F>(&mut self, states: &mut [Option<Particle<V, F>>]) {
        // assumes that iterators have available results!
        states
            .iter_mut()
            .zip(self.iterators.iter_mut())
            .for_each(|(p, iter)| {
                if let Some(p_i) = p {
                    if let Some(next_state) = iter.next() {
                        if next_state.is_valid() {
                            p_i.state = next_state;
                        } else {
                            *p = None;
                            *iter = empty_into_iter();
                        }
                    }
                }
            })
    }
}

impl<V, I, S: CollisionParams> ParticleOdeSolver<V, I, S> {
    fn collision_detection<F>(distance: &f64, p_i: &Particle<V, F>, p_j: &Particle<V, F>) -> bool {
        *distance < p_i.radius + p_j.radius
    }
    fn init_force_calculation<F>(&mut self, particles: &mut [Option<Particle<V, F>>])
    where
        F: Force<Cause = Particle<V, F>, Params = S>,
    {
        // init all forces to zero (collision markers are empty)
        particles.iter_mut().for_each(|particle| {
            if let Some(p_i) = particle {
                p_i.has_collided = false;
                p_i.force = F::zero();
                F::single(p_i, &self.params);
            }
        })
    }
    fn calc_pairwise_forces_and_check_collisions<F>(
        &mut self,
        particles: &mut [Option<Particle<V, F>>],
    ) where
        F: Force<Cause = Particle<V, F>, Params = S>,
    {
        // calculate pairwise forces for all particles and set collision markers (internal temp_collision_indices)
        // only calculate i < j and use anti-symmetry of newtonian forces - insert collisions with i and the index of j in the successors (first successor = index 0)
        let n = particles.len();
        if n < 2 {
            return;
        }
        for i in 0..(n - 1) {
            let (particle_i, particles_j) = particles.split_at_mut(i + 1);
            let particle_i = particle_i
                .last_mut()
                .expect("Particle i exists within range at the lower split.");
            if let Some(p_i) = particle_i {
                self.calc_pairwise_force_and_check_collision(i, p_i, particles_j);
            }
        }
    }
    fn add_to_temp_collision_indices(&mut self, i: usize, succ_j: usize) {
        self.temp_collision_indices
            .entry(i)
            .and_modify(|collided_succ_indices| {
                collided_succ_indices.insert(succ_j);
            })
            .or_insert(BTreeSet::from([succ_j]));
    }
    fn calc_pairwise_force_and_check_collision<F>(
        &mut self,
        i: usize,
        p_i: &mut Particle<V, F>,
        successors: &mut [Option<Particle<V, F>>],
    ) where
        F: Force<Cause = Particle<V, F>, Params = S>,
    {
        successors
            .iter_mut()
            .enumerate()
            .for_each(|(succ_j, other_particle)| {
                if let Some(p_j) = other_particle {
                    let distance = F::pairwise(p_i, p_j, &self.params);
                    if self.params.perform_collision_detection()
                        && Self::collision_detection(&distance, p_i, p_j)
                    {
                        self.add_to_temp_collision_indices(i, succ_j);
                        p_i.has_collided = true;
                        p_j.has_collided = true;
                    }
                }
            });
    }

    fn handle_collisions<F>(&mut self, particles: &mut [Option<Particle<V, F>>])
    where
        F: Force<Cause = Particle<V, F>, Params = S>,
    {
        if !self.params.perform_collision_detection() {
            return;
        }
        // assumes that the internal temp_collision_indices are set in accordance with particles
        while let Some((i, mut succ_indices)) = self.temp_collision_indices.pop_first() {
            let (particle_i, succ_particles) = particles.split_at_mut(i + 1);
            let particle_i = particle_i
                .last_mut()
                .expect("i was only added if p_i is some and length of particles did not change!");
            let merged_with_succ_i =
                self.handle_successor_collisions(particle_i, &mut succ_indices, succ_particles);
            // succ_indices now contains the remaining indices
            if let Some(merged_succ_i_index) = merged_with_succ_i {
                // particle_i became None - add remaining successors to the successor particle p_j that p_i merged with
                let j = i + 1 + merged_succ_i_index; // we splitted at i + 1 and have succ_j successors (starting at index 0)
                if !succ_indices.is_empty() {
                    for succ_i in succ_indices.iter() {
                        /* Example
                        (p_0,p_1,p_2,p_3,p_4) - p_1 collides with p_2,p_3 and p_4
                        indices for p_1: 0,1,2
                        p_2 consumes p_1 -> add p_3 and p_4 to p_2
                        indices for p_2: 0,1
                        */
                        let successor_index_for_j = succ_i - merged_succ_i_index - 1;
                        self.add_to_temp_collision_indices(j, successor_index_for_j);
                    }
                }
            };
        }
    }

    fn handle_successor_collisions<F>(
        &mut self,
        particle_i: &mut Option<Particle<V, F>>,
        succ_indices: &mut BTreeSet<usize>,
        succ_particles: &mut [Option<Particle<V, F>>],
    ) -> Option<usize>
    where
        F: Force<Cause = Particle<V, F>, Params = S>,
    {
        while let Some(succ_j) = succ_indices.pop_first() {
            let p_j = succ_particles
                .get_mut(succ_j)
                .expect("succ_j was set as valid index for successors of i (which has still the same length)")
                .as_mut()
                .expect("succ_j was only added if p_j is some and length of particles did not change!");
            F::collision(p_j, particle_i, &self.params);
            if particle_i.is_none() {
                // particle_i becomes None in an elastic collision - this ends the iteration for i and moves the partners to the particle that consumed p_i
                return Some(succ_j);
            }
        }
        None
    }
}

impl<V, F, I, S> ParticleOdeSolver<V, I, S>
where
    I: Integrator<Input = Particle<V, F>, Output = V> + Default,
    S: IntegrationParams + Clone,
{
    fn integrate(&mut self, particles: &mut [Option<Particle<V, F>>]) {
        particles
            .iter_mut()
            .zip(self.iterators.iter_mut())
            .for_each(|(particle, iter)| {
                if let Some(p) = particle {
                    *iter = match self.integrator.integrate(p) {
                        Ok(new_iter) => new_iter,
                        Err(_) => {
                            *particle = None;
                            empty_into_iter()
                        }
                    }
                }
            })
    }
}

impl<V, F, I, S> OdeSolverTrait for ParticleOdeSolver<V, I, S>
where
    V: StateIndex + FromStateVec + ValidStateCheck + Clone,
    F: Force<Cause = Particle<V, F>, Params = S> + Clone,
    I: Integrator<Input = Particle<V, F>, Output = V> + Default,
    S: IntegrationParams + CollisionParams + Clone,
{
    type State = Particle<V, F>;
    fn execute(&mut self, states: &mut [Option<Particle<V, F>>], num_executions: usize) {
        for _ in 0..num_executions {
            // check if next states come from iterators
            if self.integration_counter <= self.params.integration_steps() {
                self.integration_counter += 1;
                self.set_states_from_integration_results(states);
            } else {
                self.integration_counter = 0;
                self.init_force_calculation(states);
                self.calc_pairwise_forces_and_check_collisions(states);
                self.handle_collisions(states);
                self.integrate(states);
            }
        }
    }
    fn initial_states(&mut self, particles: &mut [Option<Particle<V, F>>]) {
        let n = particles.len();
        self.iterators = vec![empty_into_iter(); n];
        self.integrator.set_steps(self.params.integration_steps());
        self.integrator.set_randomness(false); // TODO parameter
    }
    fn reinit_states(
        &mut self,
        _particles: &mut [Option<Particle<V, F>>],
        new_indices: Vec<usize>,
    ) {
        // we assume the particles vector is the same here!
        // new states are only considered after current integration results were used
        for i in new_indices {
            self.iterators[i] = empty_into_iter();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chaos::data::{ChaosData, Fixed, InitialDistributionVariant, Linspace};
    use crate::chaos::functions::ParticleXY as ParticleXYConf;
    use crate::chaos::particle::particle_variants::*;
    fn init_particles_from_pos(
        positions: Vec<State2>,
        short: f64,
        mid: f64,
        mass: f64,
    ) -> Vec<Option<ParticleXY>> {
        let vel = State2::zeros();
        positions
            .into_iter()
            .map(|pos| {
                Some(ParticleXY::new(
                    short,
                    mid,
                    mass,
                    State4::from_position_and_velocity(pos, vel),
                ))
            })
            .collect()
    }
    fn set_particles_velocity(particles: &mut [Option<ParticleXY>], velocities: Vec<State2>) {
        particles.iter_mut().zip(velocities).for_each(|(p, vel)| {
            if let Some(p) = p.as_mut() {
                p.state.set_velocity(vel);
            }
        })
    }
    fn init_solver(
        particles: &mut [Option<ParticleXY>],
        s: f64,
        m: f64,
        l: f64,
    ) -> ParticleXYSystemSolver {
        let params = ParticleXYConf { s, m, l };
        let mut solver = ParticleXYSystemSolver::new(params);
        solver.initial_states(particles);
        solver.execute(particles, 1);
        solver
    }
    #[test]
    fn test_particle_xy_instantiation_and_total_collision() {
        let num_points = 3;
        let distr = vec![
            InitialDistributionVariant::Fixed(Fixed { value: 1.0 }), // parity 1 means collision
            InitialDistributionVariant::Fixed(Fixed { value: 0.0 }), // no charge
            InitialDistributionVariant::Fixed(Fixed { value: 10.0 }), // big mass => big radius => all collide
            InitialDistributionVariant::Linspace(Linspace {
                low: 0.0,
                high: 1.0,
            }), // 3 x-values 0.0, 0.5, 1.0
            InitialDistributionVariant::Fixed(Fixed { value: 0.0 }),  // alligned at y-Axis
            InitialDistributionVariant::Fixed(Fixed { value: 0.0 }),  //
            InitialDistributionVariant::Fixed(Fixed { value: 0.0 }),  // no velocity
        ];
        let mut chaos_data = ChaosData::<ParticleXY>::new(num_points, &distr);
        let particles = chaos_data.data_mut();
        let mut solver = init_solver(particles, 1.0, 1.0, 1.0);
        let mut num_particles = 0;
        particles.iter().enumerate().for_each(|(i, p)| {
            if let Some(p) = p {
                num_particles += 1;
                assert_eq!(
                    solver.params.integration_steps() + 1,
                    solver.iterators[i].len(),
                    "Integration should yield the specified number of steps!"
                );
                assert_eq!(p.mass, 30.0, "Mass must be the sum of the three particles!");
            }
        });
        assert_eq!(
            num_particles, 1,
            "Nearby particles must have collided into one!"
        );
        solver.execute(particles, 1);
    }
    #[test]
    fn test_particle_xy_collision_inelastic() {
        let mass = 1.0; // let radius = mass.powf(0.8); // 1.0
        let pos_0 = State2::new(-5.0, 0.0);
        let pos_1 = State2::new(0.0, 0.0);
        let pos_2 = State2::new(1.0, 0.0);
        let pos_3 = State2::new(2.0, 0.0);
        let pos_4 = State2::new(-1.0, 0.0);
        let positions = vec![pos_0, pos_1, pos_2, pos_3, pos_4];
        let mut particles: Vec<Option<ParticleXY>> =
            init_particles_from_pos(positions, 1.0, 0.0, mass);
        let mut solver = init_solver(&mut particles, 1.0, 0.0, 0.0);
        let valid_p: Vec<(usize, bool)> = particles
            .iter()
            .enumerate()
            .filter_map(|(i, p)| p.as_ref().map(|p| (i, p.has_collided)))
            .collect();
        assert_eq!(
            valid_p,
            vec![(0, false), (4, true)],
            "Particles must have collided!"
        );
        solver.execute(&mut particles, 1);
    }
    #[test]
    fn test_particle_xy_collision_elastic() {
        // important change is params.s
        let mass = 1.0; // let radius = mass.powf(0.8); // 1.0
        let pos_0 = State2::new(-5.0, 0.0);
        let pos_1 = State2::new(0.0, 0.0);
        let pos_2 = State2::new(1.0, 0.0);
        let positions = vec![pos_0, pos_1, pos_2];
        let mut particles: Vec<Option<ParticleXY>> =
            init_particles_from_pos(positions, 1.0, 0.0, mass);
        let vel_x_1 = 1.0;
        let vel_x_2 = -1.0;
        set_particles_velocity(
            &mut particles,
            vec![
                State2::zeros(),
                State2::new(vel_x_1, 0.0),
                State2::new(vel_x_2, 0.0),
            ],
        );
        let mut solver = init_solver(&mut particles, -1.0, 0.0, 0.0);
        let valid_p: Vec<(usize, bool)> = particles
            .iter()
            .enumerate()
            .filter_map(|(i, p)| p.as_ref().map(|p| (i, p.has_collided)))
            .collect();
        assert_eq!(
            valid_p,
            vec![(0, false), (1, true), (2, true)],
            "Particles 1 and 2 must have collided!"
        );
        let new_vel_1 = particles[1].as_ref().unwrap().get_state().velocity();
        let new_vel_2 = particles[2].as_ref().unwrap().get_state().velocity();
        assert_eq!(new_vel_1[0], vel_x_2);
        assert_eq!(new_vel_2[0], vel_x_1);
        solver.execute(&mut particles, 1);
    }
    #[test]
    fn test_particle_xy_force_attractive() {
        let (x_0, x_1) = (-3.0, 3.0);
        let pos_0 = State2::new(x_0, 0.0);
        let pos_1 = State2::new(x_1, 0.0);
        let mut particles: Vec<Option<ParticleXY>> =
            init_particles_from_pos(vec![pos_0, pos_1], 0.0, -1.0, 1.0);
        let mut solver = init_solver(&mut particles, 0.0, 0.0, 1.0);
        solver.execute(&mut particles, 2);
        let (p_0, p_1) = (
            particles[0]
                .as_ref()
                .expect("Particle should still exist!")
                .clone(),
            particles[1]
                .as_ref()
                .expect("Particle should still exist!")
                .clone(),
        );
        assert_eq!(
            p_0.force.state, -p_1.force.state,
            "Forces must be set pairwise!"
        );
        assert!(
            p_0.force.state[0] > 0.0,
            "Left particle is attracted to the right!"
        );
        assert!(p_0.state[2] > 0.0, "Left particle is moving to the right!");
        assert!(
            p_0.state[0] - x_0 > 0.0,
            "Left particle has moved to the right!"
        );
        assert_eq!(
            p_0.state[0] - x_0,
            x_1 - p_1.state[0],
            "Particles moved anti-symmetric."
        );
    }
}
