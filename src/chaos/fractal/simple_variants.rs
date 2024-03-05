use super::fractal_conf::*;
use super::fractal_data::*;
use super::fractal_generator::*;
use super::julia::JuliaSimple;
use super::mandelbrot::MandelbrotSimple;
use crate::chaos::data::ChaosFloat;
use crate::chaos::data::Time;
use crate::chaos::functions::DiscreteMap;
use paste::paste;
use rand_distr::Distribution;

#[derive(Clone, Debug)]
pub struct SimplePower {
    power_n: i32,
}

impl SimpleFractalFn for SimplePower {
    fn new<C: SimpleConf>(conf: C) -> Self {
        Self {
            power_n: conf.power_n(),
        }
    }
    fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        match z.power(self.power_n) {
            Some(z) => z.add(c),
            None => E::large_element(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SimpleProbability {
    r_gen: rand_distr::Uniform<ChaosFloat>,
    r_threshold: ChaosFloat,
    power_1_n: ChaosFloat,
}

impl SimpleFractalFn for SimpleProbability {
    fn new<C: SimpleConf>(conf: C) -> Self {
        let r_gen = rand_distr::Uniform::new(0.0, 1.0);
        Self {
            r_gen,
            r_threshold: conf.par_a(),
            power_1_n: 1.0 / (conf.power_n() as ChaosFloat),
        }
    }
    fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        // https://paulbourke.net/fractals/reversejulia/
        let mut rng = rand::thread_rng();
        let r = self.r_gen.sample(&mut rng);
        let s = if r <= self.r_threshold { -1.0 } else { 1.0 };
        let scalar = s * z.sub(c).norm().powf(self.power_1_n);
        let sigma = z.arg() / 2.0;
        let re = scalar * sigma.cos();
        let im = scalar * sigma.sin();
        z.with_xy(re, im)
    }
}

#[derive(Clone, Debug)]
pub struct SimpleSinus {
    power_n: i32,
}

impl SimpleFractalFn for SimpleSinus {
    fn new<C: SimpleConf>(conf: C) -> Self {
        Self {
            power_n: conf.power_n(),
        }
    }
    fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        // https://paulbourke.net/fractals/sinjulia/
        match z.power(self.power_n) {
            Some(z) => z.sinus().mul(c),
            None => E::large_element(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct SimpleSinh {
    power_n: i32,
}

impl SimpleFractalFn for SimpleSinh {
    fn new<C: SimpleConf>(conf: C) -> Self {
        Self {
            power_n: conf.power_n(),
        }
    }
    fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        // https://paulbourke.net/fractals/sinh/
        match z.sinus_hyperbolicus().power(self.power_n) {
            Some(z) => z.absolute().add(c),
            None => E::large_element(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct SimpleZubieta {
    power_n: i32,
}

impl SimpleFractalFn for SimpleZubieta {
    fn new<C: SimpleConf>(conf: C) -> Self {
        Self {
            power_n: conf.power_n(),
        }
    }
    fn next_z_n<E: AlgebraElement>(&self, z: &E, c: &E) -> E {
        // https://paulbourke.net/fractals/Zubieta/
        if let Some(z_power) = z.power(self.power_n) {
            if let Some(ref c_div_z) = c.div(z) {
                return z_power.add(c_div_z);
            }
        }
        E::large_element()
    }
}

macro_rules! generate_simple_variants {
    ($( $variant: ident ),*)=> {
        paste!{
            $(
                #[derive(Clone, Debug)]
                pub struct [<Mandelbrot $variant>]<E> {
                    generator: MandelbrotSimple<E, [<Simple $variant>]>,
                }
                impl<E> [<Mandelbrot $variant>]<E> {
                    pub fn new<P: EscapeConf + SimpleConf>(conf: P) -> Self {
                        let generator = MandelbrotSimple::new(conf);
                        Self { generator }
                    }
                }

                impl<E: AlgebraElement> DiscreteMap for [<Mandelbrot $variant>]<E> {
                    type State = FractalData<E>;
                    fn execute(&self, v: &mut FractalData<E>, t: &Time) {
                        self.generator.iteration(v, t);
                    }
                }
                #[derive(Clone, Debug)]
                pub struct [<Julia $variant>]<C> {
                    generator: JuliaSimple<C, [<Simple $variant>]>,
                }
                impl<C: AlgebraElement> [<Julia $variant>]<C> {
                    pub fn new<P: JuliaConf<Element = C> + EscapeConf + SimpleConf>(conf: P) -> Self {
                        let generator = JuliaSimple::new(conf);
                        Self { generator }
                    }
                }

                impl<E: AlgebraElement> DiscreteMap for [<Julia $variant>]<E> {
                    type State = FractalData<E>;
                    fn execute(&self, v: &mut FractalData<E>, t: &Time) {
                        self.generator.iteration(v, t);
                    }
                }
            )*
        }

    };
}

generate_simple_variants! {Power, Probability, Sinus, Sinh, Zubieta}
