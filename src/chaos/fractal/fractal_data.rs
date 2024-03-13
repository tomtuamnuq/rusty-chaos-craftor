use crate::chaos::data::*;
use nalgebra::ComplexField;

#[derive(Clone, Debug)]
pub struct FractalData<E> {
    z_0: E,
    z_n: E,
    n: usize,
    biomorph: bool,
    first: bool,
    last: bool,
}
impl<E> FractalData<E> {
    pub fn z_0(&self) -> &E {
        &self.z_0
    }
    pub fn z_n(&self) -> &E {
        &self.z_n
    }
    pub fn set_next_z_n(&mut self, z: E) {
        self.z_n = z;
        self.n += 1;
    }
    pub fn num_iterations(&self) -> usize {
        self.n
    }
    pub fn set_biomorph(&mut self) {
        self.biomorph = true;
    }
    pub fn set_first(&mut self) {
        self.first = true;
    }
    pub fn set_last(&mut self) {
        self.last = true;
    }
    pub fn biomorph(&self) -> bool {
        self.biomorph
    }
    pub fn first(&self) -> bool {
        self.first
    }
    pub fn last(&self) -> bool {
        self.last
    }
}
impl<E: Copy> FractalData<E> {
    pub fn new(z: E) -> Self {
        Self {
            z_0: z,
            z_n: z,
            n: 0,
            biomorph: false,
            first: false,
            last: false,
        }
    }
}

impl<T> ValidStateCheck for FractalData<T> {
    // not necessary since the norm of z_n is checked by IterRadiusEscape
    fn is_valid(&self) -> bool {
        true
    }
}

pub type FractalComplex = FractalData<Complex>;
pub type FractalDual = FractalData<Dual>;
pub type FractalPerplex = FractalData<Perplex>;
pub type FractalQuaternion = FractalData<Quaternion>;

impl FromStateVec for FractalComplex {
    fn from(state: InitState) -> Self {
        let z = Complex::new(state[0], state[1]);
        Self::new(z)
    }
}

impl StateIndex for FractalComplex {
    fn ind(&self, i: usize) -> ChaosFloat {
        match i {
            0 => self.z_0.real(),
            1 => self.z_0.imaginary(),
            2 => self.n as ChaosFloat,
            3 => self.z_n.real(),
            4 => self.z_n.imaginary(),
            _ => 0.0,
        }
    }
}

impl FromStateVec for FractalDual {
    fn from(state: InitState) -> Self {
        let z = Dual::new(state[0], state[1]);
        Self::new(z)
    }
}

impl StateIndex for FractalDual {
    fn ind(&self, i: usize) -> ChaosFloat {
        match i {
            0 => self.z_0.re,
            1 => self.z_0.eps,
            2 => self.n as ChaosFloat,
            3 => self.z_n.re,
            4 => self.z_n.eps,
            _ => 0.0,
        }
    }
}
impl FromStateVec for FractalPerplex {
    fn from(state: InitState) -> Self {
        let z = Perplex::new(state[0], state[1]);
        Self::new(z)
    }
}

impl StateIndex for FractalPerplex {
    fn ind(&self, i: usize) -> ChaosFloat {
        match i {
            0 => self.z_0.t,
            1 => self.z_0.x,
            2 => self.n as ChaosFloat,
            3 => self.z_n.t,
            4 => self.z_n.x,
            _ => 0.0,
        }
    }
}
impl FromStateVec for FractalQuaternion {
    fn from(state: InitState) -> Self {
        let z = Quaternion::new(state[0], state[1], state[2], state[3]);
        Self::new(z)
    }
}

impl StateIndex for FractalQuaternion {
    fn ind(&self, i: usize) -> ChaosFloat {
        match i {
            0 => self.z_0.w,
            1 => self.z_0.i,
            2 => self.z_0.j,
            3 => self.z_0.k,
            4 => self.n as ChaosFloat,
            5 => self.z_n.w,
            6 => self.z_n.i,
            7 => self.z_n.j,
            8 => self.z_n.k,
            _ => 0.0,
        }
    }
}
