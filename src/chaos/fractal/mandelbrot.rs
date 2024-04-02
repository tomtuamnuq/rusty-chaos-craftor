use std::marker::PhantomData;

use super::fractal_conf::*;
use super::fractal_data::*;
use super::fractal_generator::*;
use crate::chaos::data::ChaosFloat;
use crate::chaos::data::Time;
use crate::chaos::functions::DiscreteMap;
#[derive(Clone, Debug)]
pub struct MandelbrotSimple<E, F> {
    // https://paulbourke.net/fractals/mandelbrot/
    escape: IterRadiusEscape,
    next_z_n_callback: F,
    algebra_type: PhantomData<E>,
}

impl<E, F: SimpleFractalFn> MandelbrotSimple<E, F> {
    pub fn new<P: EscapeConf + SimpleConf>(params: P) -> Self {
        let escape = IterRadiusEscape::new(&params);
        Self {
            escape,
            next_z_n_callback: F::new(params),
            algebra_type: PhantomData,
        }
    }
}

impl<E: AlgebraElement, F: SimpleFractalFn> FractalGenerator for MandelbrotSimple<E, F> {
    type Element = E;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool {
        self.escape.check_escape(fractal)
    }
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element {
        self.next_z_n_callback
            .next_z_n(fractal.z_n(), fractal.z_0())
    }
}

#[derive(Clone, Debug)]
pub struct MandelbrotPicard<E> {
    picard: Picard,
    alpha_k_max: ChaosFloat,
    algebra_element_type: PhantomData<E>,
}

impl<E: AlgebraElement> MandelbrotPicard<E> {
    pub fn new<P: MannConf + SimpleConf>(params: P) -> Self {
        let alpha_k_max = Picard::alpha_k_max(&params);
        let picard = Picard::new(params);
        Self {
            picard,
            alpha_k_max,
            algebra_element_type: PhantomData,
        }
    }
}

impl<E: AlgebraElement> FractalGenerator for MandelbrotPicard<E> {
    type Element = E;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool {
        let c_max = fractal.z_0().norm();
        let max_radius = self.alpha_k_max.max(c_max);
        iter_radius_escape_check(fractal, DEFAULT_ITERATIONS_PICARD, max_radius)
    }
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element {
        self.picard.next_z_n(fractal.z_n(), fractal.z_0())
    }
}

impl<E: AlgebraElement> DiscreteMap for MandelbrotPicard<E> {
    type State = FractalData<E>;
    fn execute(&self, v: &mut FractalData<E>, t: &Time) {
        self.iteration(v, t)
    }
}

#[derive(Clone, Debug)]
pub struct MandelbrotBiomorph<E> {
    viscosity: Biomorph<E>,
}

impl<E: AlgebraElement> MandelbrotBiomorph<E> {
    pub fn new<P: MannConf + EscapeConf + ViscosityConf<Element = E>>(params: P) -> Self {
        let viscosity = Biomorph::new(params);
        Self { viscosity }
    }
}

impl<E: AlgebraElement> FractalGenerator for MandelbrotBiomorph<E> {
    // A novel approach to generate Mandelbrot sets, Julia sets and biomorphs via viscosity approximation method
    // https://doi.org/10.1016/j.chaos.2022.112540
    type Element = E;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool {
        self.viscosity.check_escape(fractal)
    }
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element {
        self.viscosity.next_z_n(fractal.z_n(), fractal.z_0())
    }
    fn iteration(&self, fractal: &mut FractalData<Self::Element>, _t: &Time) {
        if self.is_set_element(fractal) {
            let next_z = self.next_z_n(fractal);
            fractal.set_next_z_n(next_z);
        } else {
            self.viscosity.biomorph(fractal);
        }
    }
}

impl<E: AlgebraElement> DiscreteMap for MandelbrotBiomorph<E> {
    type State = FractalData<E>;
    fn execute(&self, v: &mut FractalData<E>, t: &Time) {
        self.iteration(v, t)
    }
}

#[derive(Clone, Debug)]
pub struct MandelbrotTranscendental<E> {
    trans: Transcendental,
    algebra_element_type: PhantomData<E>,
}

impl<E: AlgebraElement> MandelbrotTranscendental<E> {
    pub fn new<P: MannConf + TransConf>(params: P) -> Self {
        let trans = Transcendental::new(params);
        Self {
            trans,
            algebra_element_type: PhantomData,
        }
    }
}

impl<E: AlgebraElement + Clone> FractalGenerator for MandelbrotTranscendental<E> {
    type Element = E;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool {
        let z_0 = fractal.z_0().clone();
        self.trans.is_set_element(&z_0, fractal)
    }
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element {
        self.trans.next_z_n(fractal.z_n(), fractal.z_0())
    }
}

impl<E: AlgebraElement + Clone> DiscreteMap for MandelbrotTranscendental<E> {
    type State = FractalData<E>;
    fn execute(&self, v: &mut FractalData<E>, t: &Time) {
        self.iteration(v, t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chaos::data::*;
    use crate::chaos::fractal::*;
    use crate::chaos::functions::*;
    #[test]
    fn test_mandelbrot() {
        let conf = MandelbrotSinusComplex::default();
        let map = MandelbrotSinus::new(conf);
        let (a, b) = (1.0, 1.0);
        let distr = [
            InitialDistributionVariant::Fixed(Fixed { value: a }),
            InitialDistributionVariant::Fixed(Fixed { value: b }),
        ];
        let data = {
            let mut data = ChaosData::<FractalComplex>::new(2, &distr);
            data.data_mut().iter_mut().for_each(|fractal| {
                let fractal = fractal.as_mut().unwrap();
                map.execute(fractal, &0.0);
            });
            data
        };
        let z = Complex::new(a, b);
        let fractal = data.data().first().unwrap().as_ref().unwrap();
        assert_eq!(fractal.num_iterations(), 1);
        assert_ne!(*fractal.z_n(), z);
    }
}
