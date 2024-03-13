use super::chaotic_function_configs::*;
use crate::chaos::data::*;
use crate::chaos::labels::{ChaosDescription, ChaosFormula};
use std::f64::consts::PI;
pub trait DiscreteMap {
    type State;
    fn execute(&self, v: &mut Self::State, t: &Time);
}
#[derive(PartialEq, Clone, Debug)]
pub struct SimpleDiscreteMap<P> {
    conf: P,
}

impl<P> SimpleDiscreteMap<P> {
    pub fn new(conf: P) -> Self {
        Self { conf }
    }
}

impl<V, P: DiscreteMap<State = V>> DiscreteMap for SimpleDiscreteMap<P> {
    type State = V;
    fn execute(&self, v: &mut V, t: &Time) {
        self.conf.execute(v, t);
    }
}

impl DiscreteMap for Logistic {
    type State = State1;
    fn execute(&self, v: &mut State1, _t: &Time) {
        let x = v[0];
        v[0] = self.r * x * (1.0 - x);
    } // Logistic map on Wikipedia (with defaults)
}
impl Default for Logistic {
    fn default() -> Self {
        Self { r: 1.0 }
    }
}
impl ChaosDescription for Logistic {
    fn description(&self) -> String {
        "The well-known Logistic map. See the bifurcation diagram by ranging over the parameter"
            .into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Logistic_map"
    }
}
impl ChaosFormula for Logistic {
    fn formula(&self) -> &[&'static str] {
        &["x= r x (1 - x)"]
    }
}

impl DiscreteMap for Tent {
    type State = State1;
    fn execute(&self, v: &mut State1, _t: &Time) {
        let x = v[0];
        v[0] = self.mu * x.min(1.0 - x);
    } // Tent map on Wikipedia (with defaults)
}
impl Default for Tent {
    fn default() -> Self {
        Self { mu: 1.9 }
    }
}
impl ChaosDescription for Tent {
    fn description(&self) -> String {
        "The Tent map lives on the unit interval [0, 1]. The piecewise linear map is widely used as a simple model for chaotic dynamics. The map exhibits simple behavior for μ<1, but becomes chaotic with a sensitive dependendy on initial condition x, for μ>1.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Tent_map"
    }
}
impl ChaosFormula for Tent {
    fn formula(&self) -> &[&'static str] {
        &["x= μ min(x, 1 - x)"]
    }
}

impl DiscreteMap for Gauss {
    type State = State1;
    fn execute(&self, v: &mut State1, _t: &Time) {
        let x = v[0];
        v[0] = (-self.alpha * x.powi(2)).exp() + self.beta;
    } // Gauss iterated map on Wikipedia (with defaults)
}
impl Default for Gauss {
    fn default() -> Self {
        Self {
            alpha: 4.9,
            beta: -0.58,
        }
    }
}
impl ChaosDescription for Gauss {
    fn description(&self) -> String {
        "The Gauss iterated map is a nonlinear mapping over the reals by the Gaussian function. It is also referred to as the mouse map because its bifurcation diagram resembles a mouse. Try out parameter ranges.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Gauss_iterated_map"
    }
}
impl ChaosFormula for Gauss {
    fn formula(&self) -> &[&'static str] {
        &["x= exp(-α x²) + β"]
    }
}

impl DiscreteMap for Circle {
    type State = State1;
    fn execute(&self, v: &mut State1, _t: &Time) {
        let theta = v[0];
        v[0] =
            (theta + self.omega + self.k * (2.0 * PI * theta).sin() / (2.0 * PI)).rem_euclid(1.0);
    } // Arnold tongue standard circle map on Wikipedia (with defaults)
}
impl Default for Circle {
    fn default() -> Self {
        Self {
            omega: 1.0 / 3.0,
            k: PI,
        }
    }
}
impl ChaosDescription for Circle {
    fn description(&self) -> String {
        "The Arnold tongue standard circle map is a mathematical model used to study the synchronization phenomena, particularly in oscillating systems. The state θ represents the phase, parameter ω is the natural frequency (affecting rotation number), and parameter 'k' is the strength of the periodic forcing, influencing the appearance of Arnold tongues. These tongues are regions in the parameter space where the system locks into a mode with a rational rotation number. Try parameter ranges over the parameters to see bifurcations.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Arnold_tongue"
    }
}
impl ChaosFormula for Circle {
    fn formula(&self) -> &[&'static str] {
        &["θ= (θ + ω + k sin(2π θ) / 2π) mod 1"]
    }
}

impl DiscreteMap for Chirikov {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let (p, theta) = (v[0], v[1]);
        let p_new = (p + self.k * (theta).sin()).rem_euclid(2.0 * PI);
        v[0] = p_new;
        v[1] = (theta + p_new).rem_euclid(2.0 * PI);
    } // Standard map on Wikipedia (with defaults)
}
impl Default for Chirikov {
    fn default() -> Self {
        Self { k: 0.6 }
    }
}
impl ChaosDescription for Chirikov {
    fn description(&self) -> String {
        "The area-preserving Chirikov, or Standard map, represents dynamics of a kicked rotator. The state is given by the angular momentum p, and the angle 'θ'. It is a fundamental map in the study of Hamiltonian chaos with applications in accelerator physics and celestial mechanics. The parameter 'k' measures the intensity of the kicks. Small values lead to regular orbits, while larger values of 'k' introduce nonlinearity and chaos.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Standard_map"
    }
}
impl ChaosFormula for Chirikov {
    fn formula(&self) -> &[&'static str] {
        &["p= (p + k sin(θ)) mod 2π", "θ= (θ + p_new) mod 2π"]
    }
}

impl DiscreteMap for Henon {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let (x, y) = (v[0], v[1]);
        v[0] = 1.0 - self.a * x * x + y;
        v[1] = self.b * x;
    } // Henon map on Wikipedia (with defaults)
}
impl Default for Henon {
    fn default() -> Self {
        Self { a: 1.4, b: 0.3 }
    }
}
impl ChaosDescription for Henon {
    fn description(&self) -> String {
        "The well-known Hénon map.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/H%C3%A9non_map"
    }
}
impl ChaosFormula for Henon {
    fn formula(&self) -> &[&'static str] {
        &["x= 1 - a x² + y", "y= b x"]
    }
}

impl DiscreteMap for ArnoldsCat {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let (x, y) = (v[0], v[1]);
        v[0] = (2.0 * x + y).rem_euclid(1.0);
        v[1] = (x + y).rem_euclid(1.0);
    } // ArnoldsCat on Wikipedia
}
impl Default for ArnoldsCat {
    fn default() -> Self {
        Self {}
    }
}
impl ChaosDescription for ArnoldsCat {
    fn description(&self) -> String {
        "Arnold's cat map is a chaotic map from the torus into itself, named after Vladimir Arnold, who demonstrated its effects in the 1960s using an image of a cat, hence the name. It is a simple and pedagogical example for hyperbolic toral automorphism.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Arnold%27s_cat_map"
    }
}
impl ChaosFormula for ArnoldsCat {
    fn formula(&self) -> &[&'static str] {
        &["x= (2 x + y) mod 1", "y= (x + y) mod 1"]
    }
}

impl DiscreteMap for Bogdanov {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        let y_new = (1.0 + self.eps) * y + self.k * x * (x - 1.0) + self.mu * x * y;
        v[0] = x + y_new;
        v[1] = y_new;
    } // Bogdanov on Wikipedia with defaults
}
impl Default for Bogdanov {
    fn default() -> Self {
        Self {
            eps: 0.0,
            k: 1.2,
            mu: 0.0,
        }
    }
}
impl ChaosDescription for Bogdanov {
    fn description(&self) -> String {
        "The Bogdanov map is closely related to the Bogdanov-Takens bifurcation. It is used for understanding the effects of dissipative perturbations on Hamiltonian structures and can approximate dynamics of periodically forced oscillators. Parameter ε represents dissipation, k relates to the nonlinearity, and μ controls the coupling between the two dimensions.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Bogdanov_map/"
    }
}
impl ChaosFormula for Bogdanov {
    fn formula(&self) -> &[&'static str] {
        &["x= x + y_new", "y= (1.0 + ε) y + k x (x - 1) + μ x y"]
    }
}

impl DiscreteMap for Chialvo {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        v[0] = x.powi(2) * (y - x).exp() + 0.02;
        v[1] = self.a * y - self.b * x + 0.28;
    } // Chialvo on Wikipedia with defaults (with fixed offsets k and c)
}
impl Default for Chialvo {
    fn default() -> Self {
        Self { a: 0.1, b: 0.9 }
    }
}
impl ChaosDescription for Chialvo {
    fn description(&self) -> String {
        "Chialvo map models dynamics of excitable systems such as neurons. Parameter a influences the time constant of recovery, affecting how quickly the system responds to changes, while b modulates the activation-dependence of the recovery process. Offsets k=0.02 and c=0.28 are fixed.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Chialvo_map/"
    }
}
impl ChaosFormula for Chialvo {
    fn formula(&self) -> &[&'static str] {
        &["x= x² exp(y - x) + 0.02", "y= a y - bx + 0.28"]
    }
}

impl DiscreteMap for DeJongRing {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        v[0] = -6.56 * (1.4 * x).sin() - (1.56 * y).sin();
        v[1] = 1.4 * (1.40 * x).cos() + (1.56 * y).cos();
    } // https://paulbourke.net/fractals/peterdejong/ The one ring fixed params
}
impl Default for DeJongRing {
    fn default() -> Self {
        Self {}
    }
}
impl ChaosDescription for DeJongRing {
    fn description(&self) -> String {
        "A variation of Peter de Jong Attractors with a fixed set of parameters that resembles 'The one ring'.".into()
    }
    fn reference(&self) -> &'static str {
        "https://paulbourke.net/fractals/peterdejong/"
    }
}
impl ChaosFormula for DeJongRing {
    fn formula(&self) -> &[&'static str] {
        &[
            "x= -6.56 sin(1.4 x) - sin(1.56 y)",
            "y= 1.4 cos(1.4 x) + cos(1.56 y)",
        ]
    }
}

impl DiscreteMap for Duffing {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        v[0] = y;
        v[1] = -self.b * x + self.a * y - y.powi(3);
    } // Duffing map on Wikipedia (with defaults)
}
impl Default for Duffing {
    fn default() -> Self {
        Self { a: 2.75, b: 0.2 }
    }
}
impl ChaosDescription for Duffing {
    fn description(&self) -> String {
        "The Duffing map is derived from the non-linear second order differential Duffing equation. It is used to model the behavior of a damped and driven oscillator with a non-linear restoring force. Parameters a and b control linear stiffness and the amount of non-linearity in the restoring force, respectively.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Duffing_map"
    }
}
impl ChaosFormula for Duffing {
    fn formula(&self) -> &[&'static str] {
        &["x= y", "y= -b x + a y - y³"]
    }
}

impl DiscreteMap for Tinkerbell {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        v[0] = x * x - y * y + self.a * x + self.b * y;
        v[1] = 2.0 * x * y + self.c * x + self.d * y;
    } // Tinkerbell map on Wikipedia (with defaults)
}
impl Default for Tinkerbell {
    fn default() -> Self {
        Self {
            a: 0.9,     // 0.3
            b: -0.6013, // 0.6
            c: 2.0,     // 2.0
            d: 0.5,     // 0.27
        }
    }
}
impl ChaosDescription for Tinkerbell {
    fn description(&self) -> String {
        "The 2D quadratic map is characterized by the parameters, whereby a and b affect the nonlinearity (folding of the phase space), and c,d control stretching and squeezing. Different parameter combinations can lead to a variety of behavior, from periodic orbits to chaotic attractors. Try for instance a=0.3, b=0.6, c=2, d=0.27 with initial state (x=-0.72, y=-0.64).".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Tinkerbell_map"
    }
}
impl ChaosFormula for Tinkerbell {
    fn formula(&self) -> &[&'static str] {
        &["x= x² - y² + a x + b y", "y= 2 x y + c x + d y"]
    }
}

impl DiscreteMap for Baker {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        let flo = (2.0 * x).floor();
        v[0] = 2.0 * x - flo;
        v[1] = (y + flo) / 2.0;
    } // Baker map on Wikipedia - unfolded variant (with defaults)
}
impl Default for Baker {
    fn default() -> Self {
        Self {}
    }
}
impl ChaosDescription for Baker {
    fn description(&self) -> String {
        "The baker's map is a chaotic map from the unit square into itself. It is named after a kneading operation that bakers apply to dough: the dough is cut in half, and the two halves are stacked on one another, and compressed. The here implemented unfolded variant does not involve folding one of the sliced halves before stacking. It can be used to model deterministic diffusion in physics.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Baker%27s_map"
    }
}
impl ChaosFormula for Baker {
    fn formula(&self) -> &[&'static str] {
        &["x= 2x - floor(2x)", "y= (y + floor(2x)) / 2"]
    }
}

impl DiscreteMap for Clifford {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        v[0] = (self.a * y).sin() + self.c * (self.a * x).cos();
        v[1] = (self.b * x).sin() + self.d * (self.b * y).cos();
    } // Clifford Attractors on paulbourke.net (with parameter sets)
}
impl Default for Clifford {
    fn default() -> Self {
        Self {
            a: 2.0, // 0.3
            b: 1.6, // 0.6
            c: 1.0, // 2.0
            d: 0.7, // 0.27
        }
    }
}
impl ChaosDescription for Clifford {
    fn description(&self) -> String {
        "Clifford map creates intricate fractal patterns, whereby the parameters can be tuned to produce distinct and complex dynamics. See the reference for interesting parameter combinations. Try for instance a=0.3, b=0.6, c=2, d=0.27. The sensitivity to the parameters makes the map useful for studying chaos theory and applications such as cryptography and random number generation.".into()
    }
    fn reference(&self) -> &'static str {
        "https://paulbourke.net/fractals/clifford/"
    }
}
impl ChaosFormula for Clifford {
    fn formula(&self) -> &[&'static str] {
        &["x= sin(a y) + c cos(a x)", "y= sin(b x) + d cos(b y)"]
    }
}

impl DiscreteMap for Ikeda {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        let t = 0.4 - 6.0 / (1.0 + x * x + y * y);
        v[0] = 1.0 + self.u * (x * t.cos() - y * t.sin());
        v[1] = self.u * (x * t.sin() + y * t.cos());
    } // Ikeda Map on wikipedia (with parameter range)
}
impl Default for Ikeda {
    fn default() -> Self {
        Self { u: 0.6 } // for u >= 0.6 chaotic
    }
}
impl ChaosDescription for Ikeda {
    fn description(&self) -> String {
        "Ikeda models complex behavior of light in a nonlinear optical resonator. The value 't' represents a phase shift that introduces nonlinearity and sensitivity on the current state (x,y). The parameter 'u' acts as a scaling factor for the nonlinearity.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Ikeda_map"
    }
}
impl ChaosFormula for Ikeda {
    fn formula(&self) -> &[&'static str] {
        &[
            "x= 1 + u (x cos(t) - y sin(t))",
            "y= u (x sin(t) + y cos(t))",
            "t= 0.4 - 6 / (1 + x² + y²)",
        ]
    }
}

impl DiscreteMap for Gingerbreadman {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        v[0] = 1.0 - y + x.abs();
        v[1] = x;
    } // Gingerbreadman on Wikipedia
}
impl Default for Gingerbreadman {
    fn default() -> Self {
        Self {}
    }
}
impl ChaosDescription for Gingerbreadman {
    fn description(&self) -> String {
        "Gingerbreadman is a 2D chaotic map with a piecewise linear transformation that generates a fractal pattern resembling a gingerbred man when rotated. It creates chaotic sequences in certain regions while maintaining stability in others.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Gingerbreadman_map"
    }
}
impl ChaosFormula for Gingerbreadman {
    fn formula(&self) -> &[&'static str] {
        &["x= 1 - y + |x|", "y= x"]
    }
}

impl DiscreteMap for KaplanYorke {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        v[0] = (2.0 * x).rem_euclid(0.99995);
        v[1] = self.alpha * y + (4.0 * PI * x).cos();
    } // Kaplan Yorke map on Wikipedia - Calculation method (with defaults)
}
impl Default for KaplanYorke {
    fn default() -> Self {
        Self { alpha: 0.2 }
    }
}
impl ChaosDescription for KaplanYorke {
    fn description(&self) -> String {
        "The Kaplan Yorke map is a simple, straightforward process that is able to produce complex chaotic behaviour. It can be applied to study signal processing or secure communications. The parameter α significantly influences the system's dynamics.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Kaplan%E2%80%93Yorke_map"
    }
}
impl ChaosFormula for KaplanYorke {
    fn formula(&self) -> &[&'static str] {
        &["x= (2 x) mod 0.99995", "y= α y + cos(4π x)"]
    }
}

impl DiscreteMap for Rulkov {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0]; // membrane potential of neuron
        let y = v[1]; // slow variable (gating variables analog)
        v[0] = self.alpha / (1.0 + x.powi(2)) + y;
        v[1] = y - self.mu * (x - self.delta);
    } // Rulkov on Wikipedia (no defaults)
}

impl Default for Rulkov {
    fn default() -> Self {
        Self {
            alpha: 4.0,  // nonlinearity - chaotic burst for alpha > 4
            mu: 0.001,   // 0 < mu << 1
            delta: 0.01, // external dc current
        }
    }
}
impl ChaosDescription for Rulkov {
    fn description(&self) -> String {
        "The Rulkov map models a biological neuron, whereby x represents the neurons membrane potential. The small parameter μ (0<μ<<1) produces a slow y. δ is an external dc current and α a nonlinearity parameter. Chaotic bursting occurs for α > 4.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Rulkov_map"
    }
}
impl ChaosFormula for Rulkov {
    fn formula(&self) -> &[&'static str] {
        &["x= α / (1 + x²) + y", "y= y - μ (x - δ)"]
    }
}

impl Zaslavskii {
    fn mu(r: ChaosFloat) -> ChaosFloat {
        (1.0 - (-r).exp()) / r
    }
    pub fn new(eps: ChaosFloat, nu: ChaosFloat, r: ChaosFloat) -> Self {
        Self {
            eps,
            nu,
            r,
            mu: Self::mu(r),
        }
    }
}

pub fn check_zaslavskii(conf: &mut Zaslavskii) {
    conf.mu = Zaslavskii::mu(conf.r)
}

impl DiscreteMap for Zaslavskii {
    type State = State2;
    fn execute(&self, v: &mut State2, _t: &Time) {
        let x = v[0];
        let y = v[1];
        let cos_x = (2.0 * PI * x).cos();
        v[0] = (x + self.nu * (1.0 + self.mu * y) + self.eps * self.nu * self.mu * cos_x)
            .rem_euclid(1.0);
        v[1] = (-self.r).exp() * (y + self.eps * cos_x);
    } // Zaslavskii on Wikipedia (with defaults)
}

impl Default for Zaslavskii {
    fn default() -> Self {
        Zaslavskii::new(5.0, 0.2, 2.0)
    }
}
impl ChaosDescription for Zaslavskii {
    fn description(&self) -> String {
        "The Zaslavskii Map, also known as dissipative kicked rotor, or dissipative standard map, describes a periodically kicking perturbation with ε as perturbation parameter.".into()
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Zaslavskii_map"
    } // TODO http://www.scholarpedia.org/article/Zaslavsky_map
}
impl ChaosFormula for Zaslavskii {
    fn formula(&self) -> &[&'static str] {
        &[
            "x= (x + ν (1 + μ y) + ε ν μ cos(2π x)) mod 1",
            "y= exp(-r) (y + ε cos(2π x))",
            "μ= (1 - exp(-r)) / r",
        ]
    }
}

impl DiscreteMap for Shah {
    type State = State3;
    fn execute(&self, v: &mut State3, t: &Time) {
        let (x, y, z) = (v[0], v[1], v[2]);
        v[0] = y + self.alpha * x.sin() + self.gamma * z.cos();
        v[1] = x + x.sin() * y.cos() + z.tan();
        v[2] = x * t.sin() + y * t.cos() + self.beta * z.atan() - self.delta;
    } // A three dimensional chaotic map and their applications to digital audio security paper by Shah et al. (with ranges)
      // initial state (0.0705, 0.00001, 0.0038) not working ?
}
impl Default for Shah {
    fn default() -> Self {
        Self {
            alpha: 5.0, // >= 5
            beta: 1.0,  // -10 <= beta <= 10
            gamma: 0.1,
            delta: 0.1, // -1 <= gamma,delta <= 1
        }
    }
}
impl ChaosDescription for Shah {
    fn description(&self) -> String {
        "'A three dimensional chaotic map and their applications to digital audio security'. Try initial value (x=0.0705, y=0.00001, z=0.0038)".into()
    }
    fn reference(&self) -> &'static str {
        "https://doi.org/10.1007/s11042-021-10697-3"
    }
}
impl ChaosFormula for Shah {
    fn formula(&self) -> &[&'static str] {
        &[
            "x= y + α sin(x) + γ cos(z)",
            "y= x + sin(x) cos(y) + tan(z)",
            "z= x sin(t) + y cos(t) + β atanh(z) - δ",
        ]
    }
}

impl Memristive {
    fn tri(q: ChaosFloat) -> ChaosFloat {
        let mut m = 0.0;
        for i in 1..11 {
            let i_pi = PI * (i as ChaosFloat);
            m += (4.0 / i_pi.powi(2)) * (i_pi.cos() - 1.0) * (i_pi * q).cos();
        }
        m
    } // Memristive function approximation (3-2)
}
impl DiscreteMap for Memristive {
    type State = State3;
    fn execute(&self, v: &mut State3, _t: &Time) {
        let (x, y, q) = (v[0], v[1], v[2]);
        v[0] = 0.2 * (PI * x).sin() + self.k * Memristive::tri(q) * y;
        v[1] = self.a * (PI * x).cos() + (PI * y).cos();
        v[2] = q + 0.6 * (PI * q).sin() + 0.2 * (PI * y).sin();
    } // Design and Analysis of a Three-Dimensional Discrete Memristive Chaotic System with In nite Wide Parameter Range paper by Huang et al. (with defaults Table 1 Chaotic State)
      // initial state (1,1,0.8)
}
impl Default for Memristive {
    fn default() -> Self {
        Self { k: 0.35, a: 3.0 } // -10 <= k,a <= 10
    }
}
impl ChaosDescription for Memristive {
    fn description(&self) -> String {
        "'Design and Analysis of a Three-Dimensional Discrete Memristive Chaotic System with Infinite Wide Parameter Range'. Test the following initial state (x=1,y=1,q=0.8).".into()
    }
    fn reference(&self) -> &'static str {
        "http://dx.doi.org/10.21203/rs.3.rs-1109068/v1"
    }
}
impl ChaosFormula for Memristive {
    fn formula(&self) -> &[&'static str] {
        &[
            "x= 0.2 sin(π x) + k tri(q) y",
            "y= a cos(π x) + cos(π y)",
            "q= q + 0.6 sin(π q) + 0.2 sin(π y)",
            "For tri(q) see memristive function approximation (3-2). The memristive fct. is approximated by 11 steps."
        ]
    }
}

impl DiscreteMap for Sfsimm {
    type State = State4;
    fn execute(&self, v: &mut State4, _t: &Time) {
        let (x, y, z, w) = (v[0], v[1], v[2], v[3]);
        if x == 0.0 {
            v[0] = ChaosFloat::NAN;
        } else {
            v[0] = (self.b / x).sin() * self.p * (self.r * w).sin();
        }
        if y == 0.0 {
            v[1] = ChaosFloat::NAN;
        } else {
            v[1] = (self.b / y).sin() * self.p * (self.r * x).sin();
        }
        if z == 0.0 {
            v[2] = ChaosFloat::NAN;
        } else {
            v[2] = (self.b / z).sin() * self.p * (self.r * y).sin();
        }
        if w == 0.0 {
            v[3] = ChaosFloat::NAN;
        } else {
            v[3] = (self.b / w).sin() * self.p * (self.r * z).sin();
        }
    } // The new four dimensional fractional chaotic map with constant and variable order paper by Hamadneh et al. (with defaults)
      // initial state (0.5, 0.5, 0.99, 0.99)
}
impl Default for Sfsimm {
    fn default() -> Self {
        Self {
            p: 1.0, // amplitude
            b: 1.9, // frequency
            r: PI,  // internal perturbation frequency
        }
    }
}
impl ChaosDescription for Sfsimm {
    fn description(&self) -> String {
        "'The New Four-Dimensional Fractional Chaotic Map with Constant and Variable-Order: Chaos, Control and Synchronization'. See the linked paper for a discussion of 'the dynamics of the discrete 4D sinusoidal feedback sine iterative chaotic map with infinite collapse (ICMIC) modulation map (SF-SIMM) with fractional-order'. Implemented is only the 4D discrete time SF-SIMM, see equation 10, and not the fractional order version. Test the following initial state (x=0.5, y=0.5, z=0.99, w=0.99). Parameter p is the amplitude, b the frequency, and r an internal perturbation frequency.".into()
    }
    fn reference(&self) -> &'static str {
        "https://doi.org/10.3390/math11204332"
    }
}
impl ChaosFormula for Sfsimm {
    fn formula(&self) -> &[&'static str] {
        &[
            "x= p sin(b/x) sin(r w)",
            "y= p sin(b/y) sin(r x)",
            "z= p sin(b/z) sin(r y)",
            "w= p sin(b/w) sin(r z)",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_henon_map() {
        let num_points = 1;
        let (x, y) = (1.0, 2.0);
        let distr = vec![
            InitialDistributionVariant::Fixed(Fixed { value: x }),
            InitialDistributionVariant::Fixed(Fixed { value: y }),
        ];
        let mut chaos_data = ChaosData::<State2>::new(num_points, &distr);
        assert_eq!(chaos_data.data(), &[Some(State2::new(x, y))]);
        let map = Henon::default();
        chaos_data.data_mut().iter_mut().for_each(|v| {
            if let Some(v) = v.as_mut() {
                map.execute(v, &0.0);
            }
        });
        let x_1 = 1.0 - 1.4 * x * x + y;
        let y_1 = 0.3 * x;
        assert_eq!(chaos_data.data(), &[Some(State2::new(x_1, y_1))]);
    }
    #[test]
    fn test_logistic_map() {
        let num_points = 3;
        let (r, x) = (0.5, 0.5);
        let distr = vec![InitialDistributionVariant::Fixed(Fixed { value: x })];
        let mut chaos_data = ChaosData::<State1>::new(num_points, &distr);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State1::new(x)),
                Some(State1::new(x)),
                Some(State1::new(x))
            ]
        );
        let map = Logistic { r };
        chaos_data.data_mut().iter_mut().for_each(|v| {
            if let Some(v) = v.as_mut() {
                map.execute(v, &0.0);
            }
        });
        let x_1 = r * x * (1.0 - x);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State1::new(x_1)),
                Some(State1::new(x_1)),
                Some(State1::new(x_1))
            ]
        );
    }
    #[test]
    fn test_description() {
        let map = Sfsimm::default();
        assert_eq!(map.formula().len(), 4);
    }
}
