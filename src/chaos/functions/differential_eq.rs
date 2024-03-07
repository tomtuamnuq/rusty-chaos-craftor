use ode_solvers::dop853::Dop853;
use ode_solvers::dop_shared::IntegrationError as Error;
use ode_solvers::Dopri5;
use ode_solvers::Rk4;
use ode_solvers::System;
use rand::{thread_rng, Rng};
use std::vec::IntoIter;
use std::vec::Vec;

use super::chaotic_function_configs::*;
use crate::chaos::data::*;
use crate::chaos::labels::{ChaosDescription, ChaosFormula};

pub fn empty_into_iter<P>() -> IntoIter<P> {
    let v: Vec<P> = Vec::new();
    v.into_iter()
}

pub trait Integrator {
    type Input;
    type Output;
    fn integrate(&self, y0: &Self::Input) -> Result<IntoIter<Self::Output>, Error>;
    fn integration_time(&self) -> Time {
        thread_rng().gen_range(1.0..25.0)
    }
    fn set_steps(&mut self, _steps: usize) {}
    fn set_randomness(&mut self, _randomness: bool) {}
}

pub trait OdeSolverTrait {
    type State;
    fn execute(&mut self, states: &mut [Option<Self::State>], num_executions: usize);
    fn reinit_states(
        &mut self,
        all_states: &mut [Option<Self::State>],
        new_state_indices: Vec<usize>,
    );
    fn initial_states(&mut self, states: &mut [Option<Self::State>]);
}

#[derive(Clone)]
pub struct OdeSolver<V, T>
where
    T: Clone + System<Time, V> + Integrator<Input = V, Output = V>,
{
    system: T,
    iterators: Vec<IntoIter<V>>,
}

impl<V: ValidStateCheck, T: Clone + System<Time, V> + Integrator<Input = V, Output = V>> OdeSolver<V, T> {
    pub fn new(system: T) -> Self {
        Self {
            system,
            iterators: Vec::new(),
        }
    }

    fn remove_v_and_iter(state: &mut Option<V>) -> IntoIter<V> {
        *state = None;
        empty_into_iter()
    }

    fn execute_state(
        state: &mut Option<V>,
        iter: &mut IntoIter<V>,
        system: &T,
        num_executions: usize,
    ) {
        if let Some(y) = state {
            for _ in 0..num_executions {
                let next_state = iter.next();
                match next_state {
                    Some(state) => *y = state,
                    None => match system.integrate(y) {
                        Ok(new_iter) => {
                            *iter = new_iter;
                            if let Some(y_new) = iter.next() {
                                *y = y_new;
                            }
                        }
                        Err(_e) => {
                            *iter = Self::remove_v_and_iter(state);
                            return;
                        }
                    },
                }
                if !y.is_valid() {
                    *iter = Self::remove_v_and_iter(state);
                    return;
                }
            }
        }
    }
}

impl<V: ValidStateCheck, T: Clone + System<Time, V> + Integrator<Input = V, Output = V>> OdeSolverTrait
    for OdeSolver<V, T>
{
    type State = V;
    fn execute(&mut self, states: &mut [Option<V>], num_executions: usize) {
        states
            .iter_mut()
            .zip(self.iterators.iter_mut())
            .for_each(|(state, iter)| {
                Self::execute_state(state, iter, &self.system, num_executions);
            });
    }

    fn reinit_states(&mut self, all_states: &mut [Option<V>], new_indices: Vec<usize>) {
        for i in new_indices {
            let state = &mut all_states[i];
            let y0 = state
                .as_mut()
                .expect("States to reinit for the solver must all be Some");
            self.iterators[i] = match self.system.integrate(y0) {
                Ok(iter) => iter,
                Err(_e) => Self::remove_v_and_iter(state),
            };
        }
    }

    fn initial_states(&mut self, states: &mut [Option<V>]) {
        self.iterators = states
            .iter_mut()
            .map(|state| {
                if let Some(y0) = state {
                    match self.system.integrate(y0) {
                        Ok(iter) => iter,
                        Err(_e) => Self::remove_v_and_iter(state),
                    }
                } else {
                    empty_into_iter()
                }
            })
            .collect();
    }
}

macro_rules! implement_integrator_Rk4 {
    ($($system: ident, $state: ident),*) => {
        $(
            impl Integrator for $system {
                type Input = $state;
                type Output = $state;
                fn integrate(&self, y0: &$state) -> Result<IntoIter<$state>, Error> {
                    // distribute integration workload uniformly
                    let mut stepper = Rk4::new(
                        self.clone(),
                        0.0,
                        y0.to_owned(),
                        self.integration_time(),
                        1e-1,
                    );
                    let _ = stepper.integrate(); // Rk4 res is always Ok
                    let mut iter = stepper.y_out().to_owned().into_iter();
                    iter.next();
                    Ok(iter)
                }
            }
        )*
    };
}
macro_rules! implement_integrator_Dop853 {
    ($($system: ident, $state: ident),*) => {
        $(
            impl Integrator for $system {
                type Input = $state;
                type Output = $state;
                fn integrate(&self, y0: &$state) -> Result<IntoIter<$state>, Error> {
                    let mut stepper = Dop853::new(
                        self.clone(),
                        0.0,
                        self.integration_time(),
                        1e-1,
                        y0.to_owned(),
                        1e-2,
                        1e-2,
                    );
                    match stepper.integrate() {
                        Ok(_) => {
                            let mut iter = stepper.y_out().to_owned().into_iter();
                            iter.next();
                            Ok(iter)
                        }
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
            }
        )*
    };
}

implement_integrator_Rk4! {
    Lorenz, State3,
    Rossler, State3,
    Aizawa, State3,
    ChuasCircuit, State3,
    GenesioTesi, State3,
    BurkeShaw, State3,
    Rikitake, State3
}
implement_integrator_Dop853! {
    Brusselator, State2,
    VanDerPol, State2,
    QuadrupTwoOrbit, State2,
    Chen, State3,
    RabinovichFabrikant, State3,
    Halvorsen, State3,
    ThreeSpeciesLotkaVolterra, State3,
    HindmarshRose, State3
}
// On the Prediction of Chaotic Time Series using Neural Networks
// TODO Mackey Glass as 1D example ?
// http://dx.doi.org/10.51537/chaos.1116084

impl System<Time, State2> for Brusselator {
    fn system(&self, _t: Time, y: &State2, dy: &mut State2) {
        dy[0] = 1.0 - (self.b + 1.0) * y[0] + self.a * y[0] * y[0] * y[1];
        dy[1] = self.b * y[0] - self.a * y[0] * y[0] * y[1];
    } // eq. 14 and 15 in Dynamics of Brusselator
}
impl Default for Brusselator {
    fn default() -> Self {
        Self { a: 1.0, b: 3.0 }
    }
}
impl ChaosDescription for Brusselator {
    fn description(&self) -> String {
        "Brusselator is a theoretical model for a autocatalytic reaction, used to study reaction-diffusion systems and their stability. The state (x,y) represents concentrations of intermediate compounds in the reaction. Parameter 'a' is the rate of the reaction where the reactant is converted to the product, and 'b' is the rate at which the reactant is fed into the system.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Brusselator"
    }
}
impl ChaosFormula for Brusselator {
    fn formula(&self) -> &[&'static str] {
        &["dx= 1 - (b + 1) x + a x² y", "dy= b x - a x² y"]
    }
}

impl System<Time, State2> for VanDerPol {
    fn system(&self, _t: Time, y: &State2, dy: &mut State2) {
        dy[0] = self.mu * (y[0] - y[0].powi(3) / 3.0 - y[1]);
        dy[1] = y[0] / self.mu;
    } // Van der Pol Oscillator on Wikipedia - no defaults
}
impl Default for VanDerPol {
    fn default() -> Self {
        Self { mu: 1.0 }
    }
}
impl ChaosDescription for VanDerPol {
    fn description(&self) -> String {
        "The non-conservative Van der Pol system is an oscillator with non-linear damping. Parameter 'μ' indicates the non-linearity and strength of damping. It was used to study biological phenomena such as heartbeats and electrical circuits.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Van_der_Pol_oscillator"
    }
}
impl ChaosFormula for VanDerPol {
    fn formula(&self) -> &[&'static str] {
        &["dx= μ (x - x³ / 3 - y)", "dy= x / μ"]
    }
}

impl System<Time, State2> for QuadrupTwoOrbit {
    fn system(&self, _t: Time, y: &State2, dy: &mut State2) {
        dy[0] = y[1]
            - y[0].signum()
                * (self.b * y[0] - self.c).abs().ln().sin()
                * (self.c * y[0] - self.b).powi(2).atan();
        dy[1] = self.a - y[0];
    } // Quadrup Two Orbit Fractal jamesh.id.au (with defaults)
}
impl Default for QuadrupTwoOrbit {
    fn default() -> Self {
        Self {
            a: 34.0,
            b: 1.0,
            c: 5.0,
        }
    }
}
impl ChaosDescription for QuadrupTwoOrbit {
    fn description(&self) -> String {
        "A 2D map that produces fractal like visuals.".into()
    }
    fn reference(&self) -> &'static str {
        "https://www.jamesh.id.au/fractals/orbit/quadruptwo"
    }
}
impl ChaosFormula for QuadrupTwoOrbit {
    fn formula(&self) -> &[&'static str] {
        &[
            "dx= y - sign(x) sin(ln|b x - c|) atan((c x - b)²)",
            "dy= a - x",
        ]
    }
}

impl System<Time, State3> for Lorenz {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = self.sigma * (y[1] - y[0]);
        dy[1] = y[0] * (self.rho - y[2]) - y[1];
        dy[2] = y[0] * y[1] - self.beta * y[2];
    } // Lorenz system Wikipedia (with defaults)
}
impl Default for Lorenz {
    fn default() -> Self {
        Self {
            sigma: 10.,
            beta: 8. / 3.,
            rho: 28.,
        }
    }
}
impl ChaosDescription for Lorenz {
    fn description(&self) -> String {
        "The well-known Lorenz attractor with the typical shape.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Lorenz_system"
    }
}
impl ChaosFormula for Lorenz {
    fn formula(&self) -> &[&'static str] {
        &["dx= σ (y - x)", "dy= x (ρ - z) - y", "dz= x y - β z"]
    }
}

impl System<Time, State3> for Rossler {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = -y[1] - y[2];
        dy[1] = y[0] + self.a * y[1];
        dy[2] = self.b + y[2] * (y[0] - self.c);
    } // Rössler attractor on Wikipedia (with default values)
}
impl Default for Rossler {
    fn default() -> Self {
        Self {
            a: 0.1,
            b: 0.1,
            c: 14.0,
        }
    }
}
impl ChaosDescription for Rossler {
    fn description(&self) -> String {
        "The Rössler system (after German biochemist Otto Rössler) describes a continuous-time dynamical system exhibiting chaotic behavior. Certain parameter combinations lead to a variety of attractors. A common set is a=0.2, b=0.2, c=5.7.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/R%C3%B6ssler_attractor"
    }
}
impl ChaosFormula for Rossler {
    fn formula(&self) -> &[&'static str] {
        &["dx= -y - z", "dy= x + a y", "dz= b + z (x - c)"]
    }
}

impl System<Time, State3> for Chen {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = self.a * (y[1] - y[0]);
        dy[1] = (self.c - self.a) * y[0] - y[0] * y[2] + self.c * y[1];
        dy[2] = y[0] * y[1] - self.b * y[2];
    } // Multiscroll attractor on Wikipedia (with defaults)- initial value y0 = (-0.1, 0.5, -0.6)
}
impl Default for Chen {
    fn default() -> Self {
        Self {
            a: 40.0,
            b: 3.0,
            c: 28.0,
        }
    }
}
impl ChaosDescription for Chen {
    fn description(&self) -> String {
        "The Chen system, also known as Multiscroll attractor, exhibits chaotic, fractal-like behavior. It is known for generating multiple scrolls in a single attractor. Common initial values are (x=-0.1, y=0.5, z= -0.6).".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Multiscroll_attractor"
    }
}
impl ChaosFormula for Chen {
    fn formula(&self) -> &[&'static str] {
        &[
            "dx= a (y - x)",
            "dy= (c - a) x - x z + c y",
            "dz= x y - b z",
        ]
    }
}

impl System<Time, State3> for Aizawa {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = y[0] * (y[2] - self.b) - self.d * y[1];
        dy[1] = self.d * y[0] + y[1] * (y[2] - self.b);
        dy[2] = self.c + self.a * y[2]
            - (y[2].powi(3) / 3.0)
            - y[0].powi(2)
            - y[1].powi(2) * (1.0 + self.d * y[2])
            + self.f * y[2] * y[0].powi(3);
    } // algosome.com aizawa attractor chaos (with default values)
}
impl Default for Aizawa {
    fn default() -> Self {
        Self {
            a: 0.95,
            b: 0.7,
            c: 0.6,
            d: 3.5,
            e: 0.25,
            f: 0.1,
        }
    }
}
impl ChaosDescription for Aizawa {
    fn description(&self) -> String {
        "The Aizawa system produces complex, fractal-like patterns. It is known for its distinctive shape, which resembles a sphere with a tube-like structure penetrating one of its axes.".into()
    }
    fn reference(&self) -> &'static str {
        "https://www.algosome.com/articles/aizawa-attractor-chaos.html"
    }
}
impl ChaosFormula for Aizawa {
    fn formula(&self) -> &[&'static str] {
        &[
            "dx= x (z - b) - d y",
            "dy= d x + y (z - b)",
            "dz= c + a z - z³ / 3 - x² - y² (1 + d z)",
        ]
    }
}

impl System<Time, State3> for ChuasCircuit {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = self.alpha * (y[1] - self.g(y[0]));
        dy[1] = y[0] - y[1] + y[2]; // actually x - y + Rz
        dy[2] = -self.beta * y[1];
    } // Scholarpedia Chua Circuit Equations
}
impl ChuasCircuit {
    fn g(&self, x: ChaosFloat) -> ChaosFloat {
        x.powi(3) / 16.0 - x / 6.0
    } // Generalization
}
impl Default for ChuasCircuit {
    fn default() -> Self {
        Self {
            alpha: 10.91865,
            beta: 14.0,
        }
    }
}
impl ChaosDescription for ChuasCircuit {
    fn description(&self) -> String {
        "The simple electronic Chua circuit exhibits chaotic behavior. The set of differential equations models the voltage across capacitors and the current through an inductor in the circuit. The piecewise-linear function g(x) represents the nonlinearity due to the Chua diode.".into()
    }
    fn reference(&self) -> &'static str {
        "http://www.scholarpedia.org/article/Chua_circuit"
    }
}
impl ChaosFormula for ChuasCircuit {
    fn formula(&self) -> &[&'static str] {
        &[
            "g(x)= x³ / 16 - x / 6",
            "dx= α (y - g(x))",
            "dy= x - y + z",
            "dz= -β y",
        ]
    }
}

impl System<Time, State3> for RabinovichFabrikant {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = y[1] * (y[2] - 1.0 + y[0].powi(2)) + self.gamma * y[0];
        dy[1] = y[0] * (3.0 * y[2] + 1.0 - y[0].powi(2)) + self.gamma * y[1];
        dy[2] = -2.0 * y[2] * (self.alpha + y[0] * y[1]);
    } // Rabinovich-Fabrikant equations Wikipedia (with default values) - y0 = (0.1, -0.1, 0.1)
}
impl Default for RabinovichFabrikant {
    fn default() -> Self {
        Self {
            alpha: 0.05,
            gamma: 0.1,
        }
    }
}
impl ChaosDescription for RabinovichFabrikant {
    fn description(&self) -> String {
        "The Rabinovich-Fabrikant system is a set of three coupled ordinary differential equations that exhibit chaotic behavior for certain values of the parameters α and β. Chaotic behavior has been observed for α=1.1, γ=0.87. Common initial state is (x=0.1, y=-0.1, z=0.1).".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Rabinovich%E2%80%93Fabrikant_equations"
    }
}
impl ChaosFormula for RabinovichFabrikant {
    fn formula(&self) -> &[&'static str] {
        &[
            "dx= y (z - 1 + x²) + γ x",
            "dy= x (3 z + 1 - x² + γ y)",
            "dz= -2 z (α + x y)",
        ]
    }
}

impl System<Time, State3> for GenesioTesi {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = y[1];
        dy[1] = y[2];
        dy[2] = -self.c * y[0] - self.b * y[1] - self.a * y[2] + y[0] * y[0];
    } // Design, Analysis of the GenesioTest Chaotic System and its Electronic Experimental Implemenation (with default values)
}
impl Default for GenesioTesi {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 3.03,
            c: 5.55,
        }
    }
}
impl ChaosDescription for GenesioTesi {
    fn description(&self) -> String {
        "See the paper 'Design, Analysis of the GenesioTest Chaotic System and its Electronic Experimental Implemenation'.".into()
    }
    fn reference(&self) -> &'static str {
        "https://www.researchgate.net/publication/303369826_Design_analysis_of_the_Genesio-Tesi_chaotic_system_and_its_electronic_experimental_implementation"
    }
}
impl ChaosFormula for GenesioTesi {
    fn formula(&self) -> &[&'static str] {
        &["dx= y", "dy= z", "dz= -c x - b y - a z + x²"]
    }
}

impl System<Time, State3> for BurkeShaw {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = -self.s * (y[0] + y[1]);
        dy[1] = -y[1] - self.s * y[0] * y[2];
        dy[2] = self.s * y[0] * y[1] + self.v;
    } // Burke-Shaw attractor by Paul Bourke October 2010
}
impl Default for BurkeShaw {
    fn default() -> Self {
        Self { s: 10.0, v: 4.272 }
    }
}
impl ChaosDescription for BurkeShaw {
    fn description(&self) -> String {
        "The Burke-Shaw system produces attractors with complex dynamics and is often studied in the context of chaotic systems. The parameter 's' controls the strength of the coupling between x and y, as well as the nonlinearity. Parameter 'v' influences z and affects the formation of the attractor. Changes in the parameters produce unpredictable patterns.".into()
    }
    fn reference(&self) -> &'static str {
        "https://paulbourke.net/fractals/burkeshaw/"
    }
}
impl ChaosFormula for BurkeShaw {
    fn formula(&self) -> &[&'static str] {
        &["dx= -s (x + y)", "dy= -y - s x z", "dz= s x y + v"]
    }
}

impl System<Time, State3> for Halvorsen {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = -self.a * y[0] - 4.0 * y[1] - 4.0 * y[2] - y[1] * y[1];
        dy[1] = -self.a * y[1] - 4.0 * y[2] - 4.0 * y[0] - y[2] * y[2];
        dy[0] = -self.a * y[2] - 4.0 * y[0] - 4.0 * y[1] - y[0] * y[0];
    } // voriallaz.com #11 The Halvorsen Attractor
}
impl Default for Halvorsen {
    fn default() -> Self {
        Self { a: 1.4 }
    }
}
impl ChaosDescription for Halvorsen {
    fn description(&self) -> String {
        "The Halvorsen system is known for its butterfly attractor, which is a type of chaotic attractor with wing-like structure. Parameter 'a' controls the rate at which the system's trajectories diverge, affecting the sensitivity to the initial conditions. For some values the system exhibits chaotic dynamics, while for others it may produce periodic orbits or fixed points.".into()
    }
    fn reference(&self) -> &'static str {
        "https://youtube.com/watch?v=-7AlkO8bcEk"
    }
}
impl ChaosFormula for Halvorsen {
    fn formula(&self) -> &[&'static str] {
        &[
            "dx= -a x - 4 y - 4 z - y²",
            "dy= -a y - 4 z - 4 x - z²",
            "dz= -a z - 4 x - 4 y - x²",
        ]
    }
}

impl System<Time, State3> for ThreeSpeciesLotkaVolterra {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = y[0] * (self.b - self.a11 * y[0] - self.a12 * y[1] - self.a13 * y[2]);
        dy[1] = y[1] * (-self.d1 + self.a21 * y[0] - self.a23 * y[2]);
        dy[2] = y[2] * (-self.d2 + self.a31 * y[0] + self.a32 * y[1]);
    } // food-chain or two-predators-one prey model with a13=a31=a23=a32=0
      // Analysis of three species Lotka-Volterra food web models with omnivory 2015
}
impl Default for ThreeSpeciesLotkaVolterra {
    fn default() -> Self {
        Self {
            b: 5.0,   // growth rate of resource x
            d1: 1.0,  // death rate of prey y
            d2: 1.2,  // death rate of predator z
            a11: 0.4, // rate of consumption
            a12: 1.0,
            a13: 0.0, // varies from 0 to 20
            a21: 1.0,
            a23: 1.0,
            a32: 1.0,
            a31: 0.1,
        } // section 3.5.3
    }
}
impl ChaosDescription for ThreeSpeciesLotkaVolterra {
    fn description(&self) -> String {
        "See the paper 'Analysis of three species Lotka–Volterra food web models with omnivory'"
            .into()
    }
    fn reference(&self) -> &'static str {
        "https://doi.org/10.1016/j.jmaa.2015.01.035"
    }
}
impl ChaosFormula for ThreeSpeciesLotkaVolterra {
    fn formula(&self) -> &[&'static str] {
        &["See equation (1.1) in the linked paper"]
    }
}

impl System<Time, State3> for Rikitake {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = -self.mu * y[0] + y[2] * y[1];
        dy[1] = -self.mu * y[1] + y[0] * (y[2] - self.a);
        dy[2] = 1.0 - y[0] * y[1];
    } // Rikitake Python implementation by hukenovs/chaospy on github with defaults
}
impl Default for Rikitake {
    fn default() -> Self {
        Self { a: 5.0, mu: 2.0 }
    }
}
impl ChaosDescription for Rikitake {
    fn description(&self) -> String {
        "Rikitake system attempts to explain the reversal of the Earth's magnetic field.".into()
    }
    fn reference(&self) -> &'static str {
        "https://github.com/hukenovs/chaospy"
    }
}
impl ChaosFormula for Rikitake {
    fn formula(&self) -> &[&'static str] {
        &["dx= -μ x + z y", "dy= -μ y + x (z - a)", "dz= 1 - x y"]
    }
}

impl HindmarshRose {
    fn phi(&self, x: ChaosFloat) -> ChaosFloat {
        -self.a * x.powi(3) + self.b * x.powi(2)
    }
    fn psi(&self, x: ChaosFloat) -> ChaosFloat {
        -self.c - self.d * x.powi(2)
    }
}
impl System<Time, State3> for HindmarshRose {
    fn system(&self, _t: Time, y: &State3, dy: &mut State3) {
        dy[0] = y[1] + self.phi(y[0]) - y[2] + self.i;
        dy[1] = self.psi(y[0]) - y[1];
        dy[2] = self.r * (4.0 * (y[0] + 8.0 / 5.0) - y[2]);
    } // HindmarshRoseSystem on Wikipedia (with defaults) - s,x_R fixed at (4, -8/5)
}
impl Default for HindmarshRose {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 3.0,
            c: 1.0,
            d: 5.0,   // a-d working of fast ion channels
            r: 0.001, // working of the slow ion channels - of order 0.001
            i: 1.0,   // current that enters the neuron - between -10 and 10
        }
    }
}
impl ChaosDescription for HindmarshRose {
    fn description(&self) -> String {
        "The Hindmarsh-Rose model is a system of nonlinear differential equations that describe neuronal activity, particularly the spiking and bursting of neurons. The state (x, y, z) models the membrane potential (x), the fast current (y - e.g. sodium and potassium), and the slow adaptation current of the neuron (z). The parameters 'a' and 'b' model the fast ion channels. Parameters 'c' and 'd' are related to the transport of ions through these fast channels. 'r' represents the time scale of the adaption current (typically about 0.001), and i describes the current that enters the neuron (typically between -10 and 10).".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Hindmarsh–Rose_model"
    }
}
impl ChaosFormula for HindmarshRose {
    fn formula(&self) -> &[&'static str] {
        &[
            "φ(x)= -a x³ + b x²",
            "ψ(x)= -c - d x²",
            "dx= y + φ(x) - z + i",
            "dy= ψ(x) - y",
            "dz= r (4 (x + 8/5) - z)",
        ]
    }
}

impl System<Time, State4> for Ababneh {
    fn system(&self, _t: Time, y: &State4, dy: &mut State4) {
        let (x, y, z, w) = (y[0], y[1], y[2], y[3]);
        dy[0] = self.b * w - w * y;
        dy[1] = w * x - x * z;
        dy[2] = x * y - y;
        dy[3] = self.a * (x - w);
    } // A new four dimensional chaotic attractor paper by M. Ababneh with defaults Ch. 3
      // initital values (1,1,1,1)
}

impl Integrator for Ababneh {
    type Input = State4;
    type Output = State4;
    fn integrate(&self, y0: &State4) -> Result<IntoIter<State4>, Error> {
        let mut stepper = Dopri5::new(
            self.clone(),
            0.0,
            self.integration_time(),
            1e-1,
            y0.to_owned(),
            0.25,
            0.25,
        );
        match stepper.integrate() {
            Ok(_) => {
                let mut iter = stepper.y_out().to_owned().into_iter();
                iter.next();
                Ok(iter)
            }
            Err(e) => Err(e),
        }
    }
}

impl Default for Ababneh {
    fn default() -> Self {
        Self { a: 23.0, b: -6.0 }
    } // stable for b < 7.5
}
impl ChaosDescription for Ababneh {
    fn description(&self) -> String {
        "See 'A new four dimensional chaotic attractor' paper by M. Ababneh. Try initital values (x=1, y=1, z=1, w=1). The system is supposed to be stable for b<7.5.".into()
    }
    fn reference(&self) -> &'static str {
        "https://doi.org/10.1016/j.asej.2016.08.020"
    }
}
impl ChaosFormula for Ababneh {
    fn formula(&self) -> &[&'static str] {
        &[
            "dx= b w - w y",
            "dy= w x - x z",
            "dz= x y - y",
            "dw= a (x - w)",
        ]
    }
}

impl System<Time, State4> for WeiWang {
    fn system(&self, _t: Time, y: &State4, dy: &mut State4) {
        let (x, y, z, w) = (y[0], y[1], y[2], y[3]);
        dy[0] = self.a * (y - x);
        dy[1] = -self.b * y + x * z + self.k;
        dy[2] = self.d - (x * y).exp();
        dy[3] = self.c * z * w;
    } // A new four dimensional chaotic system and its circuit implementation paper by Wang with defaults
      // initital values (1, -3, 0.1, 7) unstable...
}

impl Integrator for WeiWang {
    type Input = State4;
    type Output = State4;
    fn integrate(&self, y0: &State4) -> Result<IntoIter<State4>, Error> {
        let mut stepper = Dop853::new(
            self.clone(),
            0.0,
            self.integration_time(),
            1e-1,
            y0.to_owned(),
            1e-2,
            1e-2,
        );
        match stepper.integrate() {
            Ok(_) => {
                let mut iter = stepper.y_out().to_owned().into_iter();
                iter.next();
                Ok(iter)
            }
            Err(e) => Err(e),
        }
    }
}

impl Default for WeiWang {
    fn default() -> Self {
        Self {
            a: 2.6,
            b: 0.2,
            c: 5.0,
            d: 17.0,
            k: 3.0,
        }
    }
}
impl ChaosDescription for WeiWang {
    fn description(&self) -> String {
        "See 'A new four dimensional chaotic system and its circuit implementation' paper by Wang."
            .into()
    }
    fn reference(&self) -> &'static str {
        "https://doi.org/10.3389/fphy.2022.906138"
    }
}
impl ChaosFormula for WeiWang {
    fn formula(&self) -> &[&'static str] {
        &[
            "dx= a (y - x)",
            "dy= -b y + x z + k",
            "dz= d - exp(x y)",
            "dw= c z w",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lorentz() {
        let num_points = 2;
        let (x, y, z) = (1.0, 2.0, 3.0);
        let distr = vec![
            InitialDistributionVariant::Fixed(Fixed { value: x }),
            InitialDistributionVariant::Fixed(Fixed { value: y }),
            InitialDistributionVariant::Fixed(Fixed { value: z }),
        ];
        let mut chaos_data = ChaosData::<State3>::new(num_points, &distr);
        let system = Lorenz::default();
        let mut solver = OdeSolver::new(system);
        let data = chaos_data.data_mut();
        solver.initial_states(data);
        solver.execute(data, 1);
        let v = chaos_data.data()[0];
        assert_ne!(
            v.expect("Should not be None"),
            State3::new(x, y, z),
            "State should has changed!"
        );
        let w = chaos_data.data()[1];
        assert_eq!(v, w, "Both states should be changed deterministically!");
    }
}
