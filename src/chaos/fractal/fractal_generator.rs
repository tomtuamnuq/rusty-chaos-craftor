use std::cmp::Ordering;

use nalgebra::ComplexField;
use num_dual::DualNum;

use super::fractal_conf::*;
use super::fractal_data::*;
use crate::chaos::data::*;
pub trait AlgebraElement {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn scale(&self, factor: ChaosFloat) -> Self;
    fn div(&self, other: &Self) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn power(&self, n: i32) -> Option<Self>
    where
        Self: std::marker::Sized;
    fn exponential(&self) -> Self;
    fn sinus(&self) -> Self;
    fn sinus_hyperbolicus(&self) -> Self;
    fn arg(&self) -> ChaosFloat;
    fn real(&self) -> ChaosFloat;
    fn absolute(&self) -> Self;
    fn norm(&self) -> ChaosFloat;
    fn zero_element() -> Self;
    fn large_element() -> Self;
    fn real_norm(&self) -> ChaosFloat;
    fn imaginary_norm(&self) -> ChaosFloat;
}

impl AlgebraElement for Complex {
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }
    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }
    fn mul(&self, other: &Self) -> Self {
        *self * *other
    }
    fn scale(&self, factor: ChaosFloat) -> Self {
        factor * *self
    }
    fn div(&self, other: &Self) -> Option<Self> {
        if other.re == 0.0 && other.im == 0.0 {
            None
        } else {
            Some(*self / *other)
        }
    }
    fn power(&self, n: i32) -> Option<Self> {
        if n < 0 && self.re == 0.0 && self.im == 0.0 {
            None
        } else {
            Some(self.powi(n))
        }
    }
    fn exponential(&self) -> Self {
        self.exp()
    }
    fn sinus(&self) -> Self {
        self.sin()
    }
    fn sinus_hyperbolicus(&self) -> Self {
        self.sinh()
    }
    fn real(&self) -> ChaosFloat {
        self.re
    }
    fn arg(&self) -> ChaosFloat {
        self.argument()
    }
    fn absolute(&self) -> Self {
        let re = self.re.abs();
        let im = self.im.abs();
        Self::new(re, im)
    }
    fn norm(&self) -> ChaosFloat {
        self.modulus()
    }
    fn zero_element() -> Self {
        Self::new(0.0, 0.0)
    }
    fn large_element() -> Self {
        Self::new(ChaosFloat::MAX, 0.0)
    }
    fn real_norm(&self) -> ChaosFloat {
        self.re.abs()
    }
    fn imaginary_norm(&self) -> ChaosFloat {
        self.im.abs()
    }
}

impl AlgebraElement for Perplex {
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }
    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }
    fn mul(&self, other: &Self) -> Self {
        *self * *other
    }
    fn scale(&self, factor: ChaosFloat) -> Self {
        *self * factor
    }
    fn div(&self, other: &Self) -> Option<Self> {
        *self / *other
    }
    fn power(&self, n: i32) -> Option<Self> {
        self.powi(n)
    }
    fn exponential(&self) -> Self {
        self.exp()
    }
    fn sinus(&self) -> Self {
        self.sin()
    }
    fn sinus_hyperbolicus(&self) -> Self {
        self.sinh()
    }
    fn absolute(&self) -> Self {
        Self::new(self.t.abs(), self.x.abs())
    }
    fn real(&self) -> ChaosFloat {
        self.real()
    }
    fn arg(&self) -> ChaosFloat {
        Perplex::arg(*self)
    }
    fn norm(&self) -> ChaosFloat {
        self.magnitude()
    }
    fn zero_element() -> Self {
        Self::new(0.0, 0.0)
    }
    fn large_element() -> Self {
        Self::new(ChaosFloat::MAX, 0.0)
    }
    fn real_norm(&self) -> ChaosFloat {
        self.t.abs()
    }
    fn imaginary_norm(&self) -> ChaosFloat {
        self.x.abs()
    }
}

impl AlgebraElement for Dual {
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }
    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }
    fn mul(&self, other: &Self) -> Self {
        *self * *other
    }
    fn scale(&self, factor: ChaosFloat) -> Self {
        Self::new(factor * self.re, self.eps)
    }
    fn div(&self, other: &Self) -> Option<Self> {
        if other.re == 0.0 {
            None
        } else {
            Some(*self / *other)
        }
    }
    fn power(&self, n: i32) -> Option<Self> {
        if n < 1 && self.re == 0.0 {
            None
        } else {
            Some(self.powi(n))
        }
    }
    fn exponential(&self) -> Self {
        self.exp()
    }
    fn sinus(&self) -> Self {
        self.sin()
    }
    fn sinus_hyperbolicus(&self) -> Self {
        self.sinh()
    }
    fn absolute(&self) -> Self {
        let re = self.re.abs();
        let eps = self.eps.abs();
        Self::new(re, eps)
    }
    fn real(&self) -> ChaosFloat {
        self.re
    }
    fn arg(&self) -> ChaosFloat {
        let re = self.re.abs();
        self.eps.abs().atan2(re)
    }
    fn norm(&self) -> ChaosFloat {
        self.real_norm()
    }
    fn zero_element() -> Self {
        Self::new(0.0, 0.0)
    }
    fn large_element() -> Self {
        Self::new(ChaosFloat::MAX, 0.0)
    }
    fn real_norm(&self) -> ChaosFloat {
        self.re.abs()
    }
    fn imaginary_norm(&self) -> ChaosFloat {
        0.0
    }
}

impl AlgebraElement for Quaternion {
    fn add(&self, other: &Self) -> Self {
        *self + *other
    }
    fn sub(&self, other: &Self) -> Self {
        *self - *other
    }
    fn mul(&self, other: &Self) -> Self {
        *self * *other
    }
    fn scale(&self, factor: ChaosFloat) -> Self {
        factor * *self
    }
    fn div(&self, other: &Self) -> Option<Self> {
        self.left_div(other)
    }
    fn power(&self, n: i32) -> Option<Self> {
        if n < 1 && self.w == 0.0 && self.i == 0.0 && self.j == 0.0 && self.k == 0.0 {
            None
        } else {
            Some(self.powf(n as ChaosFloat))
        }
    }
    fn exponential(&self) -> Self {
        self.exp()
    }
    fn sinus(&self) -> Self {
        self.sin()
    }
    fn sinus_hyperbolicus(&self) -> Self {
        self.sinh()
    }
    fn real(&self) -> ChaosFloat {
        self.w
    }
    fn arg(&self) -> ChaosFloat {
        // see angle method in UnitQuaternion
        let w = self.w.abs();
        self.imag().norm().atan2(w)
    }
    fn absolute(&self) -> Self {
        let w = self.w.abs();
        let (i, j, k) = (self.i.abs(), self.j.abs(), self.k.abs());
        Self::new(w, i, j, k)
    }
    fn norm(&self) -> ChaosFloat {
        self.magnitude()
    }
    fn zero_element() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
    fn large_element() -> Self {
        Self::new(ChaosFloat::MAX, 0.0, 0.0, 0.0)
    }
    fn real_norm(&self) -> ChaosFloat {
        self.w.abs()
    }
    fn imaginary_norm(&self) -> ChaosFloat {
        self.imag().norm()
    }
}

pub trait FractalGenerator {
    type Element;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool;
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element;
    fn iteration(&self, fractal: &mut FractalData<Self::Element>, _t: &Time) {
        if self.is_set_element(fractal) {
            let next_z = self.next_z_n(fractal);
            fractal.set_next_z_n(next_z);
        }
    }
}

pub fn iter_radius_escape_check<E: AlgebraElement>(
    fractal: &mut FractalData<E>,
    max_iterations: usize,
    max_radius: ChaosFloat,
) -> bool {
    let n = fractal.num_iterations();
    match n.cmp(&max_iterations) {
        Ordering::Less => {
            let is_set_element = fractal.z_n().norm() < max_radius;
            if !is_set_element && n == 0 {
                fractal.set_first()
            }
            is_set_element
        }
        Ordering::Equal => {
            fractal.set_last();
            false
        }
        Ordering::Greater => false,
    }
}

#[derive(Clone, Debug)]
pub struct IterRadiusEscape {
    max_iterations: usize,
    max_radius: ChaosFloat,
}
impl IterRadiusEscape {
    pub fn max_radius(&self) -> ChaosFloat {
        self.max_radius
    }
    pub fn new_const(max_iterations: usize, max_radius: ChaosFloat) -> Self {
        Self {
            max_iterations,
            max_radius,
        }
    }
    pub fn new<P: EscapeConf>(params: &P) -> Self {
        Self {
            max_iterations: params.max_iterations(),
            max_radius: params.max_radius(),
        }
    }
    pub fn check_escape<E: AlgebraElement>(&self, fractal: &mut FractalData<E>) -> bool {
        iter_radius_escape_check(fractal, self.max_iterations, self.max_radius)
    }
}

pub trait SimpleFractalFn {
    fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E;
    fn new<C: SimpleConf>(conf: C) -> Self;
}
#[derive(Clone, Debug)]
pub struct Picard {
    a: ChaosFloat,
    power_n: i32,
    alpha: ChaosFloat,
}

impl Picard {
    pub fn new<P: MannConf + SimpleConf>(params: P) -> Self {
        Self {
            a: params.par_a(),
            power_n: params.power_n(),
            alpha: params.alpha(),
        }
    }
    pub fn alpha_k_max<P: MannConf + SimpleConf>(params: &P) -> ChaosFloat {
        let (alpha, k) = (params.alpha(), params.power_n());
        if k < 2 {
            0.0
        } else {
            let k = k as ChaosFloat;
            (2.0 / alpha).powf(1.0 / (k - 1.0))
        }
    }
    fn q_c<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        match z.power(self.power_n) {
            Some(z_pow) => z_pow.add(&z.scale(self.a)).add(c),
            None => E::large_element(),
        }
    }
    pub fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        let q_z = self.q_c(z, c);
        let u = q_z.scale(self.alpha).add(&z.scale(1.0 - self.alpha));
        self.q_c(&u, c)
    }
}

#[derive(Clone, Debug)]
pub struct Biomorph<E> {
    m: E,
    a: E,
    b: E,
    power_n: i32,
    alpha: ChaosFloat,
    escape: IterRadiusEscape,
}

impl<E: AlgebraElement> Biomorph<E> {
    pub fn new<P: MannConf + EscapeConf + ViscosityConf<Element = E>>(params: P) -> Self {
        let (m, a, b) = (params.m(), params.a(), params.b());
        let (power_n, alpha) = (params.power_n(), params.alpha());
        let escape = IterRadiusEscape::new(&params);
        Self {
            m,
            a,
            b,
            power_n,
            alpha,
            escape,
        }
    }
    pub fn next_z_n(&self, z: &E, r: &E) -> E {
        if let Some(z_pow) = z.power(self.power_n) {
            let g_z = self.a.mul(z).add(&self.b);
            let w_z = z_pow.add(&self.m.mul(z)).add(r);
            g_z.scale(self.alpha).add(&w_z.scale(1.0 - self.alpha))
        } else {
            E::large_element()
        }
    }
    pub fn check_escape(&self, fractal: &mut FractalData<E>) -> bool {
        self.escape.check_escape(fractal)
    }
    pub fn biomorph(&self, fractal: &mut FractalData<E>) {
        let r = self.escape.max_radius();
        let is_biomorph = fractal.z_n().real_norm() >= r && fractal.z_n().imaginary_norm() >= r;
        if is_biomorph {
            fractal.set_biomorph()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transcendental {
    a: ChaosFloat,
    b: ChaosFloat,
    power_n: i32,
    alpha: ChaosFloat,
    alpha_b_max: ChaosFloat,
}

impl Transcendental {
    pub fn new<P: MannConf + TransConf>(params: P) -> Self {
        Self {
            a: params.par_a(),
            b: params.par_b(),
            power_n: params.power_n(),
            alpha: params.alpha(),
            alpha_b_max: Self::alpha_b_max(&params),
        }
    }
    pub fn alpha_b_max<P: MannConf + TransConf>(params: &P) -> ChaosFloat {
        let (alpha, n) = (params.alpha(), params.power_n());
        if n < 2 {
            0.0
        } else {
            let n = n as ChaosFloat;
            let b = params.par_b().abs();
            (b + 2.0 / alpha).powf(1.0 / (n - 1.0))
        }
    }
    fn q_c<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        match z.power(self.power_n) {
            Some(z_pow) => z_pow
                .exponential()
                .scale(self.a)
                .add(&z.scale(self.b))
                .add(c),
            None => E::large_element(),
        }
    }
    pub fn is_set_element<E: AlgebraElement>(&self, c: &E, fractal: &mut FractalData<E>) -> bool {
        let n = fractal.num_iterations();
        match n.cmp(&DEFAULT_ITERATIONS_TRANSCENDENTAL) {
            Ordering::Less => {
                let is_set_element = {
                    let z_norm = fractal.z_n().norm();
                    let r_1 = c.norm().max(self.alpha_b_max);
                    if r_1 < z_norm {
                        false
                    } else {
                        let power_1_n = if self.power_n == 0 {
                            1.0
                        } else {
                            1.0 / (self.power_n as f64)
                        };
                        match fractal.z_0().power(self.power_n) {
                            Some(z_pow) => {
                                let r_2 = (self.a.abs() * z_pow.real()).powf(power_1_n);
                                r_2 < z_norm
                            }
                            None => false,
                        }
                    }
                };

                if !is_set_element && n == 0 {
                    fractal.set_first()
                }
                is_set_element
            }
            Ordering::Equal => {
                fractal.set_last();
                false
            }
            Ordering::Greater => false,
        }
    }
    pub fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        let q_z = self.q_c(z, c);
        q_z.scale(self.alpha).add(&z.scale(1.0 - self.alpha))
    }
}
