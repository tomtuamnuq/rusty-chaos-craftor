use super::fractal_conf::*;
use super::fractal_data::*;
use super::fractal_generator::*;
use crate::chaos::data::Time;
use crate::chaos::functions::DiscreteMap;
#[derive(Clone, Debug)]
pub struct JuliaSimple<E, F> {
    c: E,
    escape: IterRadiusEscape,
    // https://paulbourke.net/fractals/juliaset/
    next_z_n_callback: F,
}

impl<E, F: SimpleFractalFn> JuliaSimple<E, F> {
    pub fn new<P: JuliaConf<Element = E> + EscapeConf + SimpleConf>(params: P) -> Self {
        let escape = IterRadiusEscape::new(&params);
        Self {
            c: params.c(),
            escape,
            next_z_n_callback: F::new(params),
        }
    }
}

impl<E: AlgebraElement, F: SimpleFractalFn> FractalGenerator for JuliaSimple<E, F> {
    type Element = E;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool {
        self.escape.check_escape(fractal)
    }
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element {
        self.next_z_n_callback.next_z_n(fractal.z_n(), &self.c)
    }
}

#[derive(Clone, Debug)]
pub struct JuliaPicard<E> {
    c: E, // aka r
    picard: Picard,
    escape: IterRadiusEscape,
}

impl<E: AlgebraElement> JuliaPicard<E> {
    pub fn new<P: JuliaConf<Element = E> + MannConf + SimpleConf>(params: P) -> Self {
        let c = params.c();
        let c_max = c.norm();
        let alpha_k_max = Picard::alpha_k_max(&params);
        let max_radius = c_max.max(alpha_k_max);
        let escape = IterRadiusEscape::new_const(DEFAULT_ITERATIONS_PICARD, max_radius);
        let picard = Picard::new(params);
        Self { c, picard, escape }
    }
}

impl<E: AlgebraElement> FractalGenerator for JuliaPicard<E> {
    type Element = E;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool {
        self.escape.check_escape(fractal)
    }
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element {
        self.picard.next_z_n(fractal.z_n(), &self.c)
    }
}

impl<E: AlgebraElement> DiscreteMap for JuliaPicard<E> {
    type State = FractalData<E>;
    fn execute(&self, v: &mut FractalData<E>, t: &Time) {
        self.iteration(v, t)
    }
}

#[derive(Clone, Debug)]
pub struct JuliaBiomorph<E> {
    c: E, // aka r
    viscosity: Biomorph<E>,
}

impl<E: AlgebraElement> JuliaBiomorph<E> {
    pub fn new<P: JuliaConf<Element = E> + MannConf + EscapeConf + ViscosityConf<Element = E>>(
        params: P,
    ) -> Self {
        let c = params.c();
        let viscosity = Biomorph::new(params);
        Self { c, viscosity }
    }
}

impl<E: AlgebraElement> FractalGenerator for JuliaBiomorph<E> {
    // A novel approach to generate Mandelbrot sets, Julia sets and biomorphs via viscosity approximation method
    // https://doi.org/10.1016/j.chaos.2022.112540
    type Element = E;
    fn is_set_element(&self, fractal: &mut FractalData<Self::Element>) -> bool {
        self.viscosity.check_escape(fractal)
    }
    fn next_z_n(&self, fractal: &FractalData<Self::Element>) -> Self::Element {
        self.viscosity.next_z_n(fractal.z_n(), &self.c)
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

impl<E: AlgebraElement> DiscreteMap for JuliaBiomorph<E> {
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

    fn get_first_data_element(data: ChaosData<FractalComplex>) -> FractalComplex {
        let fractal = data.data().first().unwrap().as_ref().unwrap();
        fractal.clone()
    }
    #[test]
    fn test_julia_simple() {
        let conf = JuliaSinusComplex::default();
        let map = JuliaSinus::new(conf);
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
        let fractal = get_first_data_element(data);
        assert_eq!(fractal.num_iterations(), 1);
        assert_ne!(*fractal.z_n(), z);
    }

    #[test]
    fn test_julia_viscosity() {
        let conf = JuliaBiomorphComplex::default();
        let map = JuliaBiomorph::new(conf);
        dbg!(map.clone());
        let (a, b) = (1.0, 1.0);
        let z_0 = Complex::new(a, b);
        let distr = [
            InitialDistributionVariant::Fixed(Fixed { value: a }),
            InitialDistributionVariant::Fixed(Fixed { value: b }),
        ];
        let data = {
            let mut data = ChaosData::<FractalComplex>::new(2, &distr);
            data.data_mut().iter_mut().for_each(|fractal| {
                let fractal = fractal.as_mut().unwrap();
                dbg!(fractal.clone());
                assert_eq!(*fractal.z_0(), z_0);
                assert_eq!(*fractal.z_n(), z_0);
                map.execute(fractal, &0.0);
            });
            data
        };
        let fractal = get_first_data_element(data);
        dbg!(fractal.clone());
        assert_eq!(fractal.num_iterations(), 1);
        assert_ne!(*fractal.z_n(), z_0);
    }
}
