use ode_solvers::{Vector1, Vector2, Vector3, Vector4, Vector5, Vector6};
use paste::paste;
use std::cmp::Ordering;
use strum_macros::IntoStaticStr;
pub type ChaosFloat = f64;
pub type Complex = nalgebra::Complex<ChaosFloat>;
pub type Quaternion = nalgebra::geometry::Quaternion<ChaosFloat>;
pub type Dual = num_dual::Dual64;
pub type Perplex = perplex_num::Perplex<ChaosFloat>;
pub type InitState = Vec<ChaosFloat>;
pub type Time = f64;

pub trait StateIndex {
    fn ind(&self, i: usize) -> ChaosFloat;
}

pub trait FromStateVec {
    fn from(state: InitState) -> Self;
}

pub const VALID_MIN: ChaosFloat = i16::MIN as ChaosFloat;
pub const VALID_MAX: ChaosFloat = i16::MAX as ChaosFloat;
fn is_valid_number(x: &ChaosFloat) -> bool {
    if x.is_finite() {
        return VALID_MIN <= *x && *x <= VALID_MAX;
    }
    false
}
pub trait ValidStateCheck {
    fn is_valid(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistributionDimensions {
    State(usize),               // number of elements n in the State vector
    Particle(usize),            // position and velocity with n coords each
    Fractal(FractalDimensions), // start (aka c) and iterated element x with n coords each
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoStaticStr)]
pub enum FractalDimensions {
    Complex,
    Dual,
    Perplex,
    Quaternion,
}

pub const DIMS_INIT_PARTICLEXY: usize = 7;
pub const NUM_DIMS_PARTICLEXY: usize = 8;
pub const DIMS_INIT_PARTICLEXYZ: usize = 9;
pub const NUM_DIMS_PARTICLEXYZ: usize = 10;
pub const DIMS_INIT_FRACTALCOMPLEX: usize = 2;
pub const NUM_DIMS_FRACTALCOMPLEX: usize = 5;
pub const DIMS_INIT_FRACTALDUAL: usize = 2;
pub const DIMS_INIT_FRACTALPERPLEX: usize = 2;
pub const NUM_DIMS_FRACTALDUAL: usize = 5;
pub const NUM_DIMS_FRACTALPERPLEX: usize = 5;
pub const DIMS_INIT_FRACTALQUATERNION: usize = 4;
pub const NUM_DIMS_FRACTALQUATERNION: usize = 9;
impl DistributionDimensions {
    pub fn number_of_dimensions(&self) -> usize {
        match self {
            DistributionDimensions::State(n) => {
                match n {
                    0 | 1 => *n,
                    _ => *n + 2, // number of states + min + max
                }
            }
            DistributionDimensions::Particle(n) => *n * 2 + 4, // cartesian coordinates for pos and vel + parity, mass and charge + radius
            DistributionDimensions::Fractal(d) => match d {
                FractalDimensions::Complex => 5,
                FractalDimensions::Dual => 5,
                FractalDimensions::Perplex => 5,
                FractalDimensions::Quaternion => 9,
            }, // *n * 2 + 1 - coordinates for c and z as well as the iteration count
        }
    }
    pub fn num_init_dimensions(&self) -> usize {
        match self {
            DistributionDimensions::State(n) => *n,
            DistributionDimensions::Particle(n) => *n * 2 + 3, // cartesian coordinates for pos and vel + parity, mass and charge
            DistributionDimensions::Fractal(d) => match d {
                FractalDimensions::Complex => 2, // z_0 re and im
                FractalDimensions::Dual => 2,
                FractalDimensions::Perplex => 2,    // z_0 re and eps
                FractalDimensions::Quaternion => 4, // z_0 w i j k
            }, // *n * 2 + 1 - coordinates for c and z as well as the iteration count
        }
    }
}

pub const DIMS_PARTICLEXY: DistributionDimensions = DistributionDimensions::Particle(2);
pub const DIMS_PARTICLEXYZ: DistributionDimensions = DistributionDimensions::Particle(3);
pub const DIMS_FRACTALCOMPLEX: DistributionDimensions =
    DistributionDimensions::Fractal(FractalDimensions::Complex);
pub const DIMS_FRACTALDUAL: DistributionDimensions =
    DistributionDimensions::Fractal(FractalDimensions::Dual);
pub const DIMS_FRACTALPERPLEX: DistributionDimensions =
    DistributionDimensions::Fractal(FractalDimensions::Perplex);
pub const DIMS_FRACTALQUATERNION: DistributionDimensions =
    DistributionDimensions::Fractal(FractalDimensions::Quaternion);
pub type State1 = Vector1<ChaosFloat>;
pub const DIMS_STATE1: DistributionDimensions = DistributionDimensions::State(1);
impl StateIndex for State1 {
    fn ind(&self, _i: usize) -> ChaosFloat {
        self[0]
    }
}
impl ValidStateCheck for State1 {
    fn is_valid(&self) -> bool {
        is_valid_number(&self[0])
    }
}

macro_rules! impl_state_traits_and_dims {
    ($($variant:expr),*) => {
        paste!{
            $(
                pub type [<State $variant>] = [<Vector $variant>]<ChaosFloat>;
                #[allow(unused)]
                pub const [<DIMS_STATE $variant>]: DistributionDimensions = DistributionDimensions::State($variant);
                impl StateIndex for [<State $variant>] {
                    fn ind(&self, i: usize) -> ChaosFloat {
                        match i.cmp(& $variant){
                            Ordering::Less => self[i],
                            Ordering::Equal => self.min(),
                            Ordering::Greater => self.max()
                        }
                    }
                }
                impl ValidStateCheck for [<State $variant>] {
                    fn is_valid(&self) -> bool {
                        self.iter().all(is_valid_number)
                    }
                }
            )*
        }
    };
}

impl_state_traits_and_dims! {
    2, 3, 4, 5, 6
}

impl FromStateVec for State1 {
    fn from(state: InitState) -> Self {
        State1::new(state[0])
    }
}

impl FromStateVec for State2 {
    fn from(state: InitState) -> Self {
        State2::new(state[0], state[1])
    }
}

impl FromStateVec for State3 {
    fn from(state: InitState) -> Self {
        State3::new(state[0], state[1], state[2])
    }
}

impl FromStateVec for State4 {
    fn from(state: InitState) -> Self {
        State4::new(state[0], state[1], state[2], state[3])
    }
}

impl FromStateVec for State5 {
    fn from(state: InitState) -> Self {
        State5::new(state[0], state[1], state[2], state[3], state[4])
    }
}

impl FromStateVec for State6 {
    fn from(state: InitState) -> Self {
        State6::new(state[0], state[1], state[2], state[3], state[4], state[5])
    }
}
