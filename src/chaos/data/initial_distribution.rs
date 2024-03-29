use super::ChaosFloat;
use crate::chaos::labels::ChaosDescription;
use paste::paste;
use rand_distr;
use serde::{Deserialize, Serialize};
use std::fmt;

use strum_macros::IntoStaticStr;
type Feature = Vec<ChaosFloat>;
pub type Features = Vec<Feature>;
pub type InitFeatures = Vec<Feature>;

fn data_gen_from_distribution<D: rand_distr::Distribution<ChaosFloat>, R: rand::Rng>(
    distr: D,
    num_samples: usize,
    rng: &mut R,
) -> Feature {
    let mut data_gen = distr.sample_iter(rng);
    (0..num_samples).map(|_| data_gen.next().unwrap()).collect()
}

pub fn eye(num_points: usize, d: &Eye, n: usize) -> Feature {
    (0..num_points)
        .map(|i| if i == n { d.value.to_owned() } else { 0.0 })
        .collect()
}

pub fn linspace(num_points: usize, d: &Linspace) -> Feature {
    let step = if num_points > 1 {
        (d.high - d.low) / ((num_points - 1) as ChaosFloat)
    } else {
        0.0
    };
    let start = d.low;
    (0..num_points)
        .map(|i| start + step * (i as ChaosFloat))
        .collect()
}

fn geomspace(num_points: usize, d: &Geomspace) -> Feature {
    let (a, b) = (d.start, d.end);
    let sign = a.signum();
    let start = a.abs().ln();
    let step = if num_points > 1 {
        (b.abs().ln() - start) / ((num_points - 1) as ChaosFloat)
    } else {
        0.0
    };
    (0..num_points)
        .map(|i| {
            let exponent = start + step * (i as ChaosFloat);
            sign * exponent.exp()
        })
        .collect()
}

fn logspace(num_points: usize, d: &Logspace) -> Feature {
    let step = if num_points > 1 {
        (d.end - d.start) / ((num_points - 1) as ChaosFloat)
    } else {
        0.0
    };
    let start = d.start;
    let base = d.base.abs();
    let sign = d.base.signum();
    (0..num_points)
        .map(|i| {
            let exponent = start + step * (i as ChaosFloat);
            sign * base.powf(exponent)
        })
        .collect()
}

pub fn hyper_mesh_grid(meshes_init: InitFeatures, // values per axis to broadcast
) -> Features {
    match meshes_init.len() {
        0 | 1 => meshes_init,
        2 => hyper_mesh_2(meshes_init),
        3 => hyper_mesh_3(meshes_init),
        4 => hyper_mesh_4(meshes_init),
        _ => todo!("Implement meshes generically!"),
    }
}

fn hyper_mesh_2(xy_spaces: InitFeatures) -> Features {
    let (x_space, y_space) = (xy_spaces[0].as_slice(), xy_spaces[1].as_slice());
    let total_num_points = x_space.len() * y_space.len();
    let (mut x_grid, mut y_grid) = (
        Vec::with_capacity(total_num_points),
        Vec::with_capacity(total_num_points),
    );
    for x in x_space {
        for y in y_space {
            x_grid.push(*x);
            y_grid.push(*y);
        }
    }
    vec![x_grid, y_grid]
}

fn hyper_mesh_3(xyz_spaces: InitFeatures) -> Features {
    let (x_space, y_space, z_space) = (
        xyz_spaces[0].as_slice(),
        xyz_spaces[1].as_slice(),
        xyz_spaces[2].as_slice(),
    );
    let total_num_points = x_space.len() * y_space.len() * z_space.len();
    let (mut x_grid, mut y_grid, mut z_grid) = (
        Vec::with_capacity(total_num_points),
        Vec::with_capacity(total_num_points),
        Vec::with_capacity(total_num_points),
    );
    for x in x_space {
        for y in y_space {
            for z in z_space {
                x_grid.push(*x);
                y_grid.push(*y);
                z_grid.push(*z);
            }
        }
    }
    vec![x_grid, y_grid, z_grid]
}

fn hyper_mesh_4(xyzw_spaces: InitFeatures) -> Features {
    let (x_space, y_space, z_space, w_space) = (
        xyzw_spaces[0].as_slice(),
        xyzw_spaces[1].as_slice(),
        xyzw_spaces[2].as_slice(),
        xyzw_spaces[3].as_slice(),
    );
    let total_num_points = x_space.len() * y_space.len() * z_space.len() * w_space.len();
    let (mut x_grid, mut y_grid, mut z_grid, mut w_grid) = (
        Vec::with_capacity(total_num_points),
        Vec::with_capacity(total_num_points),
        Vec::with_capacity(total_num_points),
        Vec::with_capacity(total_num_points),
    );
    for x in x_space {
        for y in y_space {
            for z in z_space {
                for w in w_space {
                    x_grid.push(*x);
                    y_grid.push(*y);
                    z_grid.push(*z);
                    w_grid.push(*w);
                }
            }
        }
    }
    vec![x_grid, y_grid, z_grid, w_grid]
}

impl InitialDistributionVariant {
    pub fn is_mesh(&self) -> bool {
        matches!(self, InitialDistributionVariant::Mesh(_))
    }
    pub fn space_from_mesh(&self) -> Self {
        match self {
            InitialDistributionVariant::Mesh(mesh) => {
                InitialDistributionVariant::Linspace(Linspace {
                    low: mesh.start,
                    high: mesh.end,
                })
            }
            _ => todo!("Implement additional mappings and a default!"),
        }
    }

    pub fn random_space_from_mesh(&self) -> Self {
        match self {
            InitialDistributionVariant::Mesh(mesh) => {
                InitialDistributionVariant::Uniform(Uniform {
                    low: mesh.start,
                    high: mesh.end,
                })
            }
            _ => *self, // TODO add linspace etc.
        }
    }

    pub fn data_generation<R: rand::Rng>(&self, num_points: usize, rng: &mut R) -> Vec<ChaosFloat> {
        match self {
            InitialDistributionVariant::Normal(d) => data_gen_from_distribution(
                rand_distr::Normal::new(d.mean.to_owned(), d.std_dev.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Cauchy(d) => data_gen_from_distribution(
                rand_distr::Cauchy::new(d.median.to_owned(), d.scale.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Uniform(d) => data_gen_from_distribution(
                rand_distr::Uniform::new(d.low.to_owned(), d.high.to_owned()),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Exponential(d) => data_gen_from_distribution(
                rand_distr::Exp::new(d.lambda.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::LogNormal(d) => data_gen_from_distribution(
                rand_distr::LogNormal::new(d.mean.to_owned(), d.std_dev.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Poisson(d) => data_gen_from_distribution(
                rand_distr::Poisson::new(d.mean.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Pareto(d) => data_gen_from_distribution(
                rand_distr::Pareto::new(d.scale.to_owned(), d.shape.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::StudentT(d) => data_gen_from_distribution(
                rand_distr::StudentT::new(d.dof.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Weibull(d) => data_gen_from_distribution(
                rand_distr::Weibull::new(d.lambda.to_owned(), d.k.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Gamma(d) => data_gen_from_distribution(
                rand_distr::Gamma::new(d.shape.to_owned(), d.scale.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Beta(d) => data_gen_from_distribution(
                rand_distr::Beta::new(d.alpha.to_owned(), d.beta.to_owned()).unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::Triangular(d) => data_gen_from_distribution(
                rand_distr::Triangular::new(d.low.to_owned(), d.high.to_owned(), d.mode.to_owned())
                    .unwrap(),
                num_points,
                rng,
            ),
            InitialDistributionVariant::ChiSquared(d) => data_gen_from_distribution(
                rand_distr::ChiSquared::new(d.dof.to_owned()).unwrap(),
                num_points,
                rng,
            ),

            // non probabilistic
            InitialDistributionVariant::Fixed(d) => {
                (0..num_points).map(|_| d.value.to_owned()).collect()
            }
            InitialDistributionVariant::Linspace(d) => linspace(num_points, d),
            InitialDistributionVariant::Geomspace(d) => geomspace(num_points, d),
            InitialDistributionVariant::Eye(d) => eye(num_points, d, 0),
            InitialDistributionVariant::Mesh(d) => linspace(
                num_points,
                &Linspace {
                    low: d.start,
                    high: d.end,
                },
            ),
            InitialDistributionVariant::Logspace(d) => logspace(num_points, d),
        }
    }
}

impl From<&InitialDistributionVariant> for String {
    fn from(value: &InitialDistributionVariant) -> Self {
        let value_str: &'static str = value.into();
        value_str.into()
    }
}

impl Default for InitialDistributionVariant {
    fn default() -> Self {
        InitialDistributionVariant::Normal(Default::default())
    }
}
macro_rules! generate_initial_distribution_variants {
    ($($variant:ident $par_check_code:ident{ $($field:ident: ($field_min:expr, $field_max:expr)),* } ),*)=> {
        #[derive(PartialEq, Clone, Copy, IntoStaticStr)]
        pub enum InitialDistributionVariant {
            $($variant($variant),)*
        }

        $(
            #[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
            #[serde(default)]
            pub struct $variant {
                $(pub $field: ChaosFloat,)*
            }
            paste!{
                impl $variant {
                    $(pub const [<RANGE_ $field:upper>]: std::ops::RangeInclusive<f64> = ($field_min + f64::EPSILON)..=($field_max - f64::EPSILON);)*
                    pub fn par_range_check(&mut self) {
                        $par_check_code(self)
                    }
                }
                impl fmt::Display for $variant{
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
                        write!(f, "{}( ", stringify!($variant))?;
                        $(write!(f, "{}={:.2} ", stringify!($field), self.$field)?;)*
                        write!(f, ")")
                    }
                }
            }
        )*
    };
}

fn no_check<T>(_t: &mut T) {}
fn triangular_check(t: &mut Triangular) {
    if t.high < t.low {
        t.high = t.low
    }
    if !(t.mode >= t.low && t.high >= t.mode) {
        if t.mode < t.low {
            t.mode = t.low;
        }
        if t.high < t.mode {
            t.high = t.mode;
        }
    }
}

fn geomspace_check(g: &mut Geomspace) {
    if g.start == 0.0 {
        g.start = ChaosFloat::EPSILON;
    }
    if g.end == 0.0 {
        g.end = ChaosFloat::EPSILON;
    }
    if g.start.is_sign_negative() != g.end.is_sign_negative() {
        g.start = -g.start;
    }
    if g.start == g.end {
        g.end += ChaosFloat::EPSILON
    } else if g.start > g.end {
        std::mem::swap(&mut g.end, &mut g.start);
    }
}

generate_initial_distribution_variants! {
    Normal no_check { mean: (f64::NEG_INFINITY, f64::INFINITY), std_dev: (0.0, f64::INFINITY) },
    Cauchy no_check { median: (f64::NEG_INFINITY, f64::INFINITY), scale: (0.0, f64::INFINITY) },
    Uniform no_check { low: (f64::NEG_INFINITY, f64::INFINITY), high: (f64::NEG_INFINITY, f64::INFINITY) },
    Exponential no_check { lambda: (0.0, f64::INFINITY) },
    LogNormal no_check { mean: (f64::NEG_INFINITY, f64::INFINITY), std_dev: (0.0, f64::INFINITY) },
    Poisson no_check { mean: (0.0, f64::INFINITY) },
    Pareto no_check { scale: (0.0, f64::INFINITY), shape: (0.0, f64::INFINITY) },
    StudentT no_check { dof: (0.0, f64::INFINITY) },
    Weibull no_check { lambda: (0.0, f64::INFINITY), k: (0.0, f64::INFINITY) },
    Gamma no_check { shape: (0.0, f64::INFINITY), scale: (0.0, f64::INFINITY) },
    Beta no_check { alpha: (0.0, f64::INFINITY), beta: (0.0, f64::INFINITY) },
    Triangular triangular_check { low: (f64::NEG_INFINITY, f64::INFINITY), high: (f64::NEG_INFINITY, f64::INFINITY), mode: (f64::NEG_INFINITY, f64::INFINITY) },
    ChiSquared no_check { dof: (0.0, f64::INFINITY) },
    Fixed no_check { value: (f64::NEG_INFINITY, f64::INFINITY) },
    Linspace no_check { low: (f64::NEG_INFINITY, f64::INFINITY), high: (f64::NEG_INFINITY, f64::INFINITY) },
    Mesh no_check { start: (f64::NEG_INFINITY, f64::INFINITY), end: (f64::NEG_INFINITY, f64::INFINITY) },
    Geomspace geomspace_check { start: (f64::NEG_INFINITY, f64::INFINITY), end: (f64::NEG_INFINITY, f64::INFINITY) },
    Eye no_check { value: (f64::NEG_INFINITY, f64::INFINITY) },
    Logspace no_check { start: (f64::NEG_INFINITY, f64::INFINITY), end: (f64::NEG_INFINITY, f64::INFINITY), base:(f64::NEG_INFINITY, f64::INFINITY)  }
}

impl Default for Normal {
    fn default() -> Self {
        Normal {
            mean: 0.0,
            std_dev: 1.0,
        }
    }
}
impl ChaosDescription for Normal {
    fn description(&self) -> String {
        format!("The well-known Gaussian distribution: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Normal_distribution"
    }
}

impl Default for Cauchy {
    fn default() -> Self {
        Cauchy {
            median: 1.0,
            scale: 1.0,
        }
    }
}
impl ChaosDescription for Cauchy {
    fn description(&self) -> String {
        format!("The Cauchy, also known as Lorentz distribution, models the ratio of two normal random variables. In comparison with Normal, it has a taller peak and slower decay in its tails. The median determines the central location, whereas scale controls how quickly the tails decay: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Cauchy_distribution"
    }
}

impl Default for Uniform {
    fn default() -> Self {
        Uniform {
            low: -1.0,
            high: 1.0,
        }
    }
}
impl ChaosDescription for Uniform {
    fn description(&self) -> String {
        format!("The well-known continuous Uniform distribution: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Continuous_uniform_distribution"
    }
}

impl Default for Exponential {
    fn default() -> Self {
        Exponential { lambda: 1.0 }
    }
}
impl ChaosDescription for Exponential {
    fn description(&self) -> String {
        format!("The Exponential distribution is the continuous probability distribution of the distance between events that occur continuously and independently at a constant average rate lambda: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Exponential_distribution"
    }
}

impl Default for LogNormal {
    fn default() -> Self {
        LogNormal {
            mean: 0.0,
            std_dev: 1.0,
        }
    }
}
impl ChaosDescription for LogNormal {
    fn description(&self) -> String {
        format!(
            "The distribution of a random variable whose logarithm is normally distributed: {}",
            self
        )
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Log-normal_distribution"
    }
}

impl Default for Poisson {
    fn default() -> Self {
        Poisson { mean: 1.0 }
    }
}
impl ChaosDescription for Poisson {
    fn description(&self) -> String {
        format!("The discrete Poisson probability distribution models a number of events occuring within a fixed interval of time or space. It is commonly used for count data, where outcomes are non-negative integers (e.g. customer purchases). The single parameter represents the mean number of events within an interval: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Poisson_distribution"
    }
}

impl Default for Pareto {
    fn default() -> Self {
        Pareto {
            scale: 1.0,
            shape: 1.0,
        }
    }
}
impl ChaosDescription for Pareto {
    fn description(&self) -> String {
        format!("The continuous Pareto power-law distribution models various observable phenomena such as the wealth in society. A distribution with a shape (tail index) of 1.16 reflects the 80-20 rule (Pareto principle). The scale represents the lower bound (minimum possible value) : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Pareto_distribution"
    }
}

impl Default for StudentT {
    fn default() -> Self {
        StudentT { dof: 1.0 }
    }
}
impl ChaosDescription for StudentT {
    fn description(&self) -> String {
        format!("The Student's t distribution generalizes the standard normal distribution. It is symmetric around zero and bell-shaped, but has heavier tails. The degrees of freedom (dof) represents the number of independent observations used to estimate a population parameter such as the mean. It is crucial for assessing statistical significance and confidence intervals : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Student%27s_t-distribution"
    }
}

impl Default for Weibull {
    fn default() -> Self {
        Weibull {
            lambda: 1.0,
            k: 1.0,
        }
    }
}
impl ChaosDescription for Weibull {
    fn description(&self) -> String {
        format!("The continuous Weibull models times to failure or times between events, such as maximum one-day rainfalls or the time a user spends on a web page. The positive lambda is the scale parameter and k defines the failure rate. The rate decreases for k<1, is constant for k=1, and increases for k>1 (like an aging process) : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Weibull_distribution"
    }
}

impl Default for Gamma {
    fn default() -> Self {
        Gamma {
            shape: 1.0,
            scale: 1.0,
        }
    }
}
impl ChaosDescription for Gamma {
    fn description(&self) -> String {
        format!("Gamma is a generalization of Exponential and Chi-Squared. In Bayesian statistics it is commonly used as a conjugate prior for various types of inverse scale rate parameters such as lambda of Exponential : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Gamma_distribution"
    }
}

impl Default for Beta {
    fn default() -> Self {
        Beta {
            alpha: 1.0,
            beta: 1.0,
        }
    }
}
impl ChaosDescription for Beta {
    fn description(&self) -> String {
        format!("Beta is a family of continuous distributions with values on the interval [0, 1]. It has broad applications in task duration modeling and project planning. The two shape parameters provide a flexible and adaptable probability distribution : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Beta_distribution"
    }
}

impl Default for Triangular {
    fn default() -> Self {
        Triangular {
            low: 0.0,
            high: 1.0,
            mode: 0.5,
        }
    }
}
impl ChaosDescription for Triangular {
    fn description(&self) -> String {
        format!("A probability density function (PDF) with a triangle shape defined by the parameters. The support is low≤x≤high, with a maximum probability of 2/(high-low) at x=mode : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Triangular_distribution"
    }
}

impl Default for ChiSquared {
    fn default() -> Self {
        ChiSquared { dof: 1.0 }
    }
}
impl ChaosDescription for ChiSquared {
    fn description(&self) -> String {
        format!("The distribution of a sum of dof (degrees of freedom) independent standard normal random variables. It is a special case of Gamma and used in hypothesis testing (goodness of fit) : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Chi-squared_distribution"
    }
}

impl Default for Fixed {
    fn default() -> Self {
        Fixed { value: 0.0 }
    }
}
impl ChaosDescription for Fixed {
    fn description(&self) -> String {
        format!("Each data sample will have the same real number: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Real_number"
    }
}

impl Default for Eye {
    fn default() -> Self {
        Eye { value: 1.0 }
    }
}
impl ChaosDescription for Eye {
    fn description(&self) -> String {
        format!("Takes on the parameter value if the sample index is the same as the feature index (seen as a matrix). All other indice pairs are assigned to 0.0:  : {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://wikipedia.org/wiki/Identity_matrix"
    }
}

impl Default for Linspace {
    fn default() -> Self {
        Linspace {
            low: -1.0,
            high: 1.0,
        }
    }
}
impl ChaosDescription for Linspace {
    fn description(&self) -> String {
        format!("A sequence of equally spaced numbers within the interval [low, high], including both endpoints: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://numpy.org/doc/stable/reference/generated/numpy.linspace.html"
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            start: -1.0,
            end: 1.0,
        }
    }
}
impl ChaosDescription for Mesh {
    fn description(&self) -> String {
        format!("A (hyper)-mesh of points. A single mesh is a Linspace with n points. Two times a mesh provides a grid with n² points. Three times mesh results in a 3D mesh of n³ points, and so on. Each space generates feature values in [low, high], including both endpoints: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://numpy.org/doc/stable/reference/generated/numpy.meshgrid.html"
    }
}

impl Default for Geomspace {
    fn default() -> Self {
        Geomspace {
            start: 1.0,
            end: 1000.0,
        }
    }
}
impl ChaosDescription for Geomspace {
    fn description(&self) -> String {
        format!("Generates a sequence of equally spaced numbers on a logarithmic scale with geometric progression: {}", self)
    }
    fn reference(&self) -> &'static str {
        "https://numpy.org/doc/stable/reference/generated/numpy.geomspace.html"
    }
}

impl Default for Logspace {
    fn default() -> Self {
        Logspace {
            start: 0.0,
            end: 3.0,
            base: 2.0,
        }
    }
}
impl ChaosDescription for Logspace {
    fn description(&self) -> String {
        format!(
            "Generates a sequence of equally spaced numbers on a logarithmic scale with base: {}",
            self
        )
    }
    fn reference(&self) -> &'static str {
        "https://numpy.org/doc/stable/reference/generated/numpy.logspace.html"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_display() {
        let normal = Normal::default();
        assert_eq!(
            normal.to_string(),
            String::from("Normal( mean=0.00 std_dev=1.00 )")
        );
        assert_eq!(
            normal.description(),
            String::from("The well-known Gaussian distribution: Normal( mean=0.00 std_dev=1.00 )")
        );
    }
}
