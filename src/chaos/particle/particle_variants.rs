use crate::chaos::data::*;
use crate::chaos::functions::Integrator;
use crate::chaos::functions::ParticleXY as ParticleXYConf;
use crate::chaos::functions::ParticleXYZ as ParticleXYZConf;
use crate::chaos::labels::*;
use ode_solvers::dop_shared::IntegrationError as Error;
use ode_solvers::Rk4;
use ode_solvers::System;
use rand::{thread_rng, Rng};
use std::vec::IntoIter;

use super::newton_coloumb::*;
pub type ParticleXY = Particle<State4, NewtonColoumb<State2, State4, ParticleXYConf>>;
pub type ParticleXYZ = Particle<State6, NewtonColoumb<State3, State6, ParticleXYZConf>>;
pub type ParticleXYSystemSolver = ParticleOdeSolver<State4, FastIntegratorXY, ParticleXYConf>;
pub type ParticleXYZSystemSolver = ParticleOdeSolver<State6, FastIntegratorXYZ, ParticleXYZConf>;

#[derive(Clone, Debug)]
pub struct Particle<V, F> {
    pub short: ChaosFloat, // aka strong
    pub mid: ChaosFloat,   // aka charge
    pub mass: ChaosFloat,  // aka mass
    pub state: V,          // position and velocity
    pub force: F,          // mass * acceleration = force
    pub radius: ChaosFloat,
    pub has_collided: bool,
}

impl<V, F> Particle<V, F>
where
    F: Force,
{
    pub fn new(short: ChaosFloat, mid: ChaosFloat, mass: ChaosFloat, state: V) -> Self {
        let mass = mass.abs().max(f64::EPSILON);
        let radius = Self::calc_radius(mass);
        Self {
            short,
            mid,
            mass,
            state,
            force: Force::zero(),
            radius,
            has_collided: false,
        }
    }
    pub fn get_state(&self) -> &V {
        &self.state
    }
    fn calc_radius(mass: f64) -> f64 {
        mass.sqrt() // powf(0.8) like a star
    }

    pub fn add_scalar_values(&mut self, other_particle: &Self) {
        self.short += other_particle.short;
        self.mid += other_particle.mid;
        self.mass += other_particle.mass;
        self.radius = Self::calc_radius(self.mass);
    }
}

impl<V, F> FromStateVec for Particle<V, F>
where
    V: FromStateVec,
    F: Force,
{
    fn from(state: InitState) -> Self {
        let (static_part, dynamic_part) = state.split_at(3);
        Particle::new(
            static_part[0],
            static_part[1],
            static_part[2],
            V::from(dynamic_part.to_owned()),
        )
    }
}

impl StateIndex for ParticleXY {
    fn ind(&self, i: usize) -> ChaosFloat {
        match i {
            0..=3 => self.state[i],
            4 => self.short,
            5 => self.mid,
            6 => self.mass,
            7 => self.radius,
            _ => {
                // TODO panic ?
                0.0
            }
        }
    }
}

impl StateIndex for ParticleXYZ {
    fn ind(&self, i: usize) -> ChaosFloat {
        match i {
            0..=5 => self.state[i],
            6 => self.short,
            7 => self.mid,
            8 => self.mass,
            9 => self.radius,
            _ => {
                // TODO panic ?
                0.0
            }
        }
    }
}

impl<V, F> ValidStateCheck for Particle<V, F>
where
    V: ValidStateCheck,
{
    fn is_valid(&self) -> bool {
        self.state.is_valid()
    }
}

impl System<State4> for ParticleXY {
    fn system(&self, _t: Time, y: &State4, dy: &mut State4) {
        let acceleration = self.force.get_state() / self.mass;
        *dy = State4::from_column_slice(&[y[2], y[3], acceleration[0], acceleration[1]]);
        // pos / dt = v
        // v / dt = a = F / m
    }
}

impl System<State6> for ParticleXYZ {
    fn system(&self, _t: Time, y: &State6, dy: &mut State6) {
        let acceleration = self.force.get_state() / self.mass;
        *dy = State6::from_column_slice(&[
            y[3],
            y[4],
            y[5],
            acceleration[0],
            acceleration[1],
            acceleration[2],
        ]);
    }
}

impl NewtonParams for ParticleXYConf {
    fn mid_scale_factor(&self) -> ChaosFloat {
        self.m
    }
    fn large_scale_factor(&self) -> ChaosFloat {
        self.l
    }
}
impl CollisionParams for ParticleXYConf {
    fn collision_factor(&self) -> ChaosFloat {
        self.s
    }
    fn perform_collision_detection(&self) -> bool {
        self.s != 0.0
    }
}

impl NewtonParams for ParticleXYZConf {
    fn mid_scale_factor(&self) -> ChaosFloat {
        self.m
    }
    fn large_scale_factor(&self) -> ChaosFloat {
        self.l
    }
}
impl CollisionParams for ParticleXYZConf {
    fn collision_factor(&self) -> ChaosFloat {
        self.s
    }
    fn perform_collision_detection(&self) -> bool {
        self.s != 0.0
    }
}

impl IntegrationParams for ParticleXYConf {}
impl IntegrationParams for ParticleXYZConf {}
const G: ChaosFloat = 6.673; // actually 6.673 * 10**(-11)
const K: ChaosFloat = 8.988; // actually k: 8.988 * 10**(9)
const S: ChaosFloat = 1.0;

impl Default for ParticleXYConf {
    fn default() -> Self {
        Self { s: S, m: K, l: G }
    }
}
impl ChaosDescription for ParticleXYConf {
    fn description(&self) -> String {
        LABEL_PARTICLE_DESCRIPTION.into()
    }
    fn reference(&self) -> &'static str {
        LINK_PARTICLE
    }
}
impl ChaosFormula for ParticleXYConf {
    fn formula(&self) -> &[&'static str] {
        &FORMULA_PARTICLE
    }
}

impl Default for ParticleXYZConf {
    fn default() -> Self {
        Self { s: S, m: K, l: G }
    }
}
impl ChaosDescription for ParticleXYZConf {
    fn description(&self) -> String {
        LABEL_PARTICLE_DESCRIPTION.into()
    }
    fn reference(&self) -> &'static str {
        LINK_PARTICLE
    }
}
impl ChaosFormula for ParticleXYZConf {
    fn formula(&self) -> &[&'static str] {
        &FORMULA_PARTICLE
    }
}

macro_rules! particle_integrator_Rk4 {
    ($particle: ident, $state: ident, $integrator: ident) => {
        #[derive(Clone, Debug)]
        pub struct $integrator {
            step_size: f64,
            max_step: f64,
            is_random: bool,
        }
        impl Default for $integrator {
            fn default() -> Self {
                Self {
                    step_size: 0.01,
                    max_step: 0.11,
                    is_random: true,
                }
            }
        }
        impl Integrator for $integrator {
            type Input = $particle;
            type Output = $state;
            fn set_steps(&mut self, num_steps: usize) {
                self.max_step = self.step_size * (num_steps as f64);
            }
            fn set_randomness(&mut self, randomness: bool) {
                self.is_random = randomness;
            }
            fn integration_time(&self) -> Time {
                if self.is_random {
                    thread_rng().gen_range(self.step_size..self.max_step)
                } else {
                    self.max_step
                }
            }
            fn integrate(&self, p: &$particle) -> Result<IntoIter<$state>, Error> {
                let y0 = p.get_state();
                let mut stepper = Rk4::new(
                    p.clone(),
                    0.0,
                    y0.to_owned(),
                    self.integration_time(),
                    self.step_size,
                );
                let _ = stepper.integrate(); // Rk4 res is always Ok
                let next_particle_states = stepper.y_out().to_owned().into_iter();
                Ok(next_particle_states)
            }
        }
    };
}

particle_integrator_Rk4!(ParticleXY, State4, FastIntegratorXY);
particle_integrator_Rk4!(ParticleXYZ, State6, FastIntegratorXYZ);
