use super::discrete_maps::check_zaslavskii;
use crate::chaos::data::ChaosFloat;
use crate::chaos::fractal::*;
use paste::paste;
use std::f64::consts::PI;
use std::fmt;
macro_rules! generate_chaotic_function_configs {
    ($($variant:ident $par_check_code:ident{ $($field:ident: ($field_min:expr, $field_max:expr)),* } ),*)=> {
        $(
            #[derive(PartialEq, Clone, Debug)]
            pub struct $variant {
                $(pub $field: ChaosFloat,)*
            }
            paste!{
                impl $variant {
                    $(pub const [<RANGE_ $field:upper>]: (ChaosFloat, ChaosFloat) = (($field_min + ChaosFloat::EPSILON), ($field_max - ChaosFloat::EPSILON));)*
                    pub fn par_range_check(&mut self) {
                        $par_check_code(self)
                    }

                }
            }
            impl fmt::Display for $variant{
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
                    write!(f, "{}( ", stringify!($variant))?;
                    $(write!(f, "{}={:.2} ", stringify!($field), self.$field)?;)*
                    write!(f, ")")
                }
            }
        )*
    };
}
fn no_check<T>(_t: &mut T) {}

generate_chaotic_function_configs! {
    Logistic no_check { r: (0.0, 4.0) },
    Tent no_check{ mu: (0.0, 4.0) },
    Gauss no_check { alpha: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), beta: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY) },
    Circle no_check { omega: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), k: (0.0, 4.0 * PI) },
    Chirikov no_check { k: (0.0, 2.0 * PI) },
    Henon no_check { a: (-2.0, 2.0), b: (-1.5, 1.5) },
    ArnoldsCat no_check { },
    Bogdanov no_check { eps: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), k: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), mu: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY) },
    Chialvo no_check { a: (0.0, 1.0), b: (0.0, 1.0) },
    DeJongRing no_check {  },
    Duffing no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY) },
    Tinkerbell no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), c: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY), d: (ChaosFloat::NEG_INFINITY, ChaosFloat:: INFINITY) },
    Baker no_check { },
    Clifford no_check { a: (0.0, ChaosFloat:: INFINITY), b: (0.0, ChaosFloat:: INFINITY), c: (0.0, ChaosFloat:: INFINITY), d: (0.0, ChaosFloat:: INFINITY) },
    Ikeda no_check { u: (0.0, ChaosFloat:: INFINITY) },
    Gingerbreadman no_check { },
    KaplanYorke no_check { alpha: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Rulkov no_check { alpha: (0.0, 10.0), mu: (0.0, 0.1), delta: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Zaslavskii check_zaslavskii { eps: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), nu: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), mu: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Shah no_check { alpha: (5.0, ChaosFloat::INFINITY), beta: (-10.0, 10.0), gamma: (-1.0, 1.0), delta: (-1.0, 1.0) },
    Memristive no_check { k: (-10.0, 10.0), a: (-10.0, 10.0) },
    Sfsimm no_check { p: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b: (0.0, 2.0*PI), r: (0.0, 2.0*PI) },
    MandelbrotPowerComplex no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotProbabilityComplex no_check { a: (0.0, 1.0),  r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinusComplex no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinhComplex no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotZubietaComplex no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotPicardComplex no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotBiomorphComplex check_mandelbrotbiomorph_complex {r: (0.0, ChaosFloat::INFINITY), m_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_re: (-1.0, 1.0), a_im: (-1.0, 1.0), b_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPowerComplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaProbabilityComplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a: (0.0, 1.0),  r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinusComplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinhComplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaZubietaComplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPicardComplex no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaBiomorphComplex check_juliabiomorph_complex {r: (0.0, ChaosFloat::INFINITY), c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_re: (-1.0, 1.0), a_im: (-1.0, 1.0), b_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotPowerDual no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotProbabilityDual no_check { a: (0.0, 1.0),  r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinusDual no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinhDual no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotZubietaDual no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotPicardDual no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotBiomorphDual check_mandelbrotbiomorph_dual {r: (0.0, ChaosFloat::INFINITY), m_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_re: (-1.0, 1.0), a_im: (-1.0, 1.0), b_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPowerDual no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaProbabilityDual no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a: (0.0, 1.0),  r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinusDual no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinhDual no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaZubietaDual no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPicardDual no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaBiomorphDual check_juliabiomorph_dual {r: (0.0, ChaosFloat::INFINITY), c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_re: (-1.0, 1.0), a_im: (-1.0, 1.0), b_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotPowerPerplex no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotProbabilityPerplex no_check { a: (0.0, 1.0),  r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinusPerplex no_check { r: (0.0, 20.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinhPerplex no_check { r: (0.0, 20.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotZubietaPerplex no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotPicardPerplex no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotBiomorphPerplex check_mandelbrotbiomorph_perplex {r: (0.0, ChaosFloat::INFINITY), m_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_re: (-1.0, 1.0), a_im: (-1.0, 1.0), b_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPowerPerplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaProbabilityPerplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a: (0.0, 1.0),  r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinusPerplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, 20.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinhPerplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, 20.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaZubietaPerplex no_check { c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPicardPerplex no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaBiomorphPerplex check_juliabiomorph_perplex {r: (0.0, ChaosFloat::INFINITY), c_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_re: (-1.0, 1.0), a_im: (-1.0, 1.0), b_re: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_im: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotPowerQuaternion no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotProbabilityQuaternion no_check { a: (0.0, 1.0),  r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinusQuaternion no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotSinhQuaternion no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotZubietaQuaternion no_check { r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotPicardQuaternion no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    MandelbrotBiomorphQuaternion check_mandelbrotbiomorph_quaternion {r: (0.0, ChaosFloat::INFINITY), m_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_w: (-1.0, 1.0), a_i: (-1.0, 1.0), a_j: (-1.0, 1.0), a_k: (-1.0, 1.0), b_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPowerQuaternion no_check { c_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaProbabilityQuaternion no_check { c_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a: (0.0, 1.0), r: (0.0, 2.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinusQuaternion no_check { c_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaSinhQuaternion no_check { c_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaZubietaQuaternion no_check { c_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), r: (0.0, ChaosFloat::INFINITY), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaPicardQuaternion no_check {a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    JuliaBiomorphQuaternion check_juliabiomorph_quaternion {r: (0.0, ChaosFloat::INFINITY), c_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), m_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), a_w: (-1.0, 1.0), a_i: (-1.0, 1.0), a_j: (-1.0, 1.0), a_k: (-1.0, 1.0), b_w: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_i: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_j: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b_k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), alpha: (0.0, 1.0), n: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY)  },
    Brusselator no_check { a: (0.0, ChaosFloat::INFINITY), b: (0.0, ChaosFloat::INFINITY) },
    VanDerPol no_check { mu: (0.0, 4.0) },
    QuadrupTwoOrbit no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Lorenz no_check { sigma: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), beta: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), rho: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Rossler no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Chen no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Aizawa no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), d: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), e: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), f: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    ChuasCircuit no_check { alpha: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), beta: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    RabinovichFabrikant no_check { alpha: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), gamma: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    GenesioTesi no_check { a: (0.0, ChaosFloat::INFINITY), b: (0.0, ChaosFloat::INFINITY), c: (0.0, ChaosFloat::INFINITY) },
    BurkeShaw no_check { s: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), v: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    Halvorsen no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    ThreeSpeciesLotkaVolterra no_check { b: (0.0, ChaosFloat::INFINITY), d1: (0.0, ChaosFloat::INFINITY), d2: (0.0, ChaosFloat::INFINITY),  a11: (0.0, ChaosFloat::INFINITY), a12: (0.0, ChaosFloat::INFINITY), a13: (0.0, ChaosFloat::INFINITY), a21: (0.0, ChaosFloat::INFINITY), a23: (0.0, ChaosFloat::INFINITY), a31: (0.0, ChaosFloat::INFINITY), a32: (0.0, ChaosFloat::INFINITY) },
    Rikitake no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), mu: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    HindmarshRose no_check { a: (0.0, ChaosFloat::INFINITY), b: (0.0, ChaosFloat::INFINITY), c: (0.0, ChaosFloat::INFINITY), d: (0.0, ChaosFloat::INFINITY), r: (0.0, 0.1), i: (-10.0, 10.0) },
    Ababneh no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    WeiWang no_check { a: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), b: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), c: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), d: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), k: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    ParticleXY no_check { s: (-10.0, 10.0), m: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), l: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) },
    ParticleXYZ no_check { s: (-10.0, 10.0), m: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY), l: (ChaosFloat::NEG_INFINITY, ChaosFloat::INFINITY) }
}
