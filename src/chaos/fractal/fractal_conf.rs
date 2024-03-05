use crate::chaos::data::*;
use crate::chaos::functions::*;
use paste::paste;

use super::fractal_generator::AlgebraElement;
const DEFAULT_A: ChaosFloat = 0.5;
const DEFAULT_TRI_R: ChaosFloat = 50.0;
const DEFAULT_MANDELBROT_R: ChaosFloat = 2.0;
const DEFAULT_BIOMORPH_R: ChaosFloat = 10.0;
const DEFAULT_ITERATIONS_COLOR: usize = 255;
pub const DEFAULT_ITERATIONS_PICARD: usize = 30;
const DEFAULT_ITERATIONS_BIOMORPH: usize = 10;
pub trait EscapeConf {
    fn max_iterations(&self) -> usize;
    fn max_radius(&self) -> ChaosFloat;
}

macro_rules! implement_escape_conf {
    ($($variant:ident $max_iterations:expr, [ $($elem:ident),* ] ),*)=> {
        $(
            paste!{
                $(
                    impl EscapeConf for [<$variant $elem>] {
                        fn max_radius(&self) -> ChaosFloat {
                            self.r
                        }
                        fn max_iterations(&self) -> usize {
                            $max_iterations
                        }
                    }
                )*
            }
        )*
    };
}

implement_escape_conf! {
    MandelbrotPower DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    MandelbrotProbability DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    MandelbrotSinus DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    MandelbrotSinh DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    MandelbrotZubieta DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    MandelbrotBiomorph DEFAULT_ITERATIONS_BIOMORPH, [Complex, Dual, Perplex, Quaternion],
    JuliaPower DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    JuliaProbability DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    JuliaSinus DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    JuliaSinh DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    JuliaZubieta DEFAULT_ITERATIONS_COLOR, [Complex, Dual, Perplex, Quaternion],
    JuliaBiomorph DEFAULT_ITERATIONS_BIOMORPH, [Complex, Dual, Perplex, Quaternion]
}

pub trait SimpleConf {
    fn power_n(&self) -> i32;
    fn par_a(&self) -> ChaosFloat;
}

macro_rules! implement_simple_conf {
    ($($variant:ident [ $($elem:ident),* ] ),*)=> {
        $(
            paste!{
                $(
                    impl SimpleConf for [<$variant $elem>] {
                        fn power_n(&self) -> i32 {
                            self.n.round() as i32
                        }
                        fn par_a(&self) -> ChaosFloat {
                            0.0
                        }
                    }
                )*
            }

        )*
    };
}

implement_simple_conf! {
    MandelbrotPower [Complex, Dual, Perplex, Quaternion],
    MandelbrotSinus [Complex, Dual, Perplex, Quaternion],
    MandelbrotSinh  [Complex, Dual, Perplex, Quaternion],
    MandelbrotZubieta  [Complex, Dual, Perplex, Quaternion],
    JuliaPower  [Complex, Dual, Perplex, Quaternion],
    JuliaSinus  [Complex, Dual, Perplex, Quaternion],
    JuliaSinh  [Complex, Dual, Perplex, Quaternion],
    JuliaZubieta  [Complex, Dual, Perplex, Quaternion]
}

macro_rules! implement_simple_conf_with_a {
    ($($variant:ident [ $($elem:ident),* ] ),*)=> {
        $(
            paste!{
                $(
                    impl SimpleConf for [<$variant $elem>] {
                        fn power_n(&self) -> i32 {
                            self.n.round() as i32
                        }
                        fn par_a(&self) -> ChaosFloat{
                            self.a
                        }
                    }
                )*
            }

        )*
    };
}

implement_simple_conf_with_a! {
    MandelbrotProbability  [Complex, Dual, Perplex, Quaternion],
    MandelbrotPicard  [Complex, Dual, Perplex, Quaternion],
    JuliaProbability  [Complex, Dual, Perplex, Quaternion],
    JuliaPicard  [Complex, Dual, Perplex, Quaternion]
}

pub trait JuliaConf {
    type Element;
    fn c(&self) -> Self::Element;
}

macro_rules! implement_simple_julia_conf {
    ($($variant:ident ),*)=> {
            $(
                paste!{
                    impl JuliaConf for [<Julia $variant Complex>] {
                        type Element = Complex;
                        fn c(&self) -> Self::Element {
                            Self::Element::new(self.c_re, self.c_im)
                        }
                    }
                    impl JuliaConf for [<Julia $variant Dual>] {
                        type Element = Dual;
                        fn c(&self) -> Self::Element {
                            Self::Element::new(self.c_re, self.c_im)
                        }
                    }
                    impl JuliaConf for [<Julia $variant Perplex>] {
                        type Element = Perplex;
                        fn c(&self) -> Self::Element {
                            Self::Element::new(self.c_re, self.c_im)
                        }
                    }
                    impl JuliaConf for [<Julia $variant Quaternion>] {
                        type Element = Quaternion;
                        fn c(&self) -> Self::Element {
                            Self::Element::new(self.c_w, self.c_i, self.c_j, self.c_k)
                        }
                    }
                }

            )*
    };
}

implement_simple_julia_conf! {
    Power, Probability, Sinus, Sinh, Zubieta, Picard, Biomorph
}

pub trait MannConf {
    fn alpha(&self) -> ChaosFloat;
}
macro_rules! implement_mann_conf {
    ($($variant:ident [ $($elem:ident),* ] ),*)=> {
        $(
            paste!{
                $(
                    impl MannConf for [<$variant $elem>] {
                        fn alpha(&self) -> ChaosFloat {
                            self.alpha
                        }
                    }
                )*
            }

        )*
    };
}

implement_mann_conf! {
    MandelbrotPicard [Complex, Dual, Perplex, Quaternion],
    MandelbrotBiomorph [Complex, Dual, Perplex, Quaternion],
    JuliaPicard [Complex, Dual, Perplex, Quaternion],
    JuliaBiomorph [Complex, Dual, Perplex, Quaternion]
}

pub trait ViscosityConf {
    type Element: AlgebraElement;
    fn power_n(&self) -> i32;
    fn m(&self) -> Self::Element;
    fn a(&self) -> Self::Element;
    fn b(&self) -> Self::Element;
}

pub fn check_mandelbrotbiomorph_complex(config: &mut MandelbrotBiomorphComplex) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_re = DEFAULT_A;
        config.a_im = 0.0;
    }
}
impl ViscosityConf for MandelbrotBiomorphComplex {
    type Element = Complex;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_re, self.m_im)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_re, self.a_im)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_re, self.b_im)
    }
}

pub fn check_mandelbrotbiomorph_dual(config: &mut MandelbrotBiomorphDual) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_re = DEFAULT_A;
        config.a_im = 0.0;
    }
}
impl ViscosityConf for MandelbrotBiomorphDual {
    type Element = Dual;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_re, self.m_im)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_re, self.a_im)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_re, self.b_im)
    }
}
pub fn check_mandelbrotbiomorph_perplex(config: &mut MandelbrotBiomorphPerplex) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_re = DEFAULT_A;
        config.a_im = 0.0;
    }
}
impl ViscosityConf for MandelbrotBiomorphPerplex {
    type Element = Perplex;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_re, self.m_im)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_re, self.a_im)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_re, self.b_im)
    }
}

pub fn check_mandelbrotbiomorph_quaternion(config: &mut MandelbrotBiomorphQuaternion) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_w = DEFAULT_A;
        config.a_i = 0.0;
        config.a_j = 0.0;
        config.a_k = 0.0;
    }
}
impl ViscosityConf for MandelbrotBiomorphQuaternion {
    type Element = Quaternion;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_w, self.m_i, self.m_j, self.m_k)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_w, self.a_i, self.a_j, self.a_k)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_w, self.b_i, self.b_j, self.b_k)
    }
}

pub fn check_juliabiomorph_complex(config: &mut JuliaBiomorphComplex) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_re = DEFAULT_A;
        config.a_im = 0.0;
    }
}
impl ViscosityConf for JuliaBiomorphComplex {
    type Element = Complex;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_re, self.m_im)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_re, self.a_im)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_re, self.b_im)
    }
}

pub fn check_juliabiomorph_dual(config: &mut JuliaBiomorphDual) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_re = DEFAULT_A;
        config.a_im = 0.0;
    }
}
impl ViscosityConf for JuliaBiomorphDual {
    type Element = Dual;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_re, self.m_im)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_re, self.a_im)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_re, self.b_im)
    }
}

pub fn check_juliabiomorph_perplex(config: &mut JuliaBiomorphPerplex) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_re = DEFAULT_A;
        config.a_im = 0.0;
    }
}
impl ViscosityConf for JuliaBiomorphPerplex {
    type Element = Perplex;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_re, self.m_im)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_re, self.a_im)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_re, self.b_im)
    }
}

pub fn check_juliabiomorph_quaternion(config: &mut JuliaBiomorphQuaternion) {
    let a = config.a();
    if a.norm() > 1.0 {
        config.a_w = DEFAULT_A;
        config.a_i = 0.0;
        config.a_j = 0.0;
        config.a_k = 0.0;
    }
}
impl ViscosityConf for JuliaBiomorphQuaternion {
    type Element = Quaternion;
    fn power_n(&self) -> i32 {
        self.n.round() as i32
    }
    fn m(&self) -> Self::Element {
        Self::Element::new(self.m_w, self.m_i, self.m_j, self.m_k)
    }
    fn a(&self) -> Self::Element {
        Self::Element::new(self.a_w, self.a_i, self.a_j, self.a_k)
    }
    fn b(&self) -> Self::Element {
        Self::Element::new(self.b_w, self.b_i, self.b_j, self.b_k)
    }
}

impl Default for MandelbrotPowerComplex {
    fn default() -> Self {
        Self {
            r: DEFAULT_MANDELBROT_R,
            n: 2.0,
        }
    }
}
impl Default for MandelbrotPowerDual {
    fn default() -> Self {
        Self {
            r: DEFAULT_MANDELBROT_R,
            n: 2.0,
        }
    }
}

impl Default for MandelbrotPowerPerplex {
    fn default() -> Self {
        Self {
            r: 0.5,
            n: 5.0,
        }
    }
}
impl Default for MandelbrotPowerQuaternion {
    // fix z_j and z_k to 0
    fn default() -> Self {
        Self { r: 10.0, n: 1.0 }
    }
}

impl Default for MandelbrotProbabilityComplex {
    fn default() -> Self {
        Self {
            a: 0.99,
            r: 1.0,
            n: 2.0,
        }
    }
}
impl Default for MandelbrotProbabilityDual {
    fn default() -> Self {
        Self {
            a: 0.99,
            r: 1.0,
            n: 2.0,
        }
    }
}
impl Default for MandelbrotProbabilityPerplex {
    fn default() -> Self {
        Self {
            a: 0.25,
            r: 0.75,
            n: 2.0,
        }
    }
}
impl Default for MandelbrotProbabilityQuaternion {
    fn default() -> Self {
        Self {
            a: 0.99,
            r: 1.0,
            n: 2.0,
        }
    }
}

impl Default for MandelbrotSinusComplex {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 1.0,
        }
    }
}
impl Default for MandelbrotSinusDual {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 1.0,
        }
    }
}
impl Default for MandelbrotSinusPerplex {
    fn default() -> Self {
        Self { r: 0.25, n: -10.0 }
    }
}
impl Default for MandelbrotSinusQuaternion {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 1.0,
        }
    }
}

impl Default for MandelbrotSinhComplex {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 4.0,
        }
    }
}
impl Default for MandelbrotSinhDual {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 4.0,
        }
    }
}
impl Default for MandelbrotSinhPerplex {
    fn default() -> Self {
        Self { r: 5.0, n: -1.0 }
    }
}
impl Default for MandelbrotSinhQuaternion {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 4.0,
        }
    }
}

impl Default for MandelbrotZubietaComplex {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 2.0,
        }
    }
}
impl Default for MandelbrotZubietaDual {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 2.0,
        }
    }
}
impl Default for MandelbrotZubietaPerplex {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 2.0,
        }
    }
}
impl Default for MandelbrotZubietaQuaternion {
    fn default() -> Self {
        Self {
            r: DEFAULT_TRI_R,
            n: 2.0,
        }
    }
}

impl Default for JuliaPowerComplex {
    fn default() -> Self {
        Self {
            c_re: -0.5,
            c_im: -0.05,
            r: DEFAULT_MANDELBROT_R,
            n: 3.0,
        }
    }
}
impl Default for JuliaPowerDual {
    fn default() -> Self {
        Self {
            c_re: -0.5,
            c_im: -0.05,
            r: DEFAULT_MANDELBROT_R,
            n: 3.0,
        }
    }
}
impl Default for JuliaPowerPerplex {
    fn default() -> Self {
        Self {
            c_re: -0.25,
            c_im: 0.05,
            r: 10.0,
            n: -2.0,
        }
    }
}
impl Default for JuliaPowerQuaternion {
    // On the quaternion Julia sets via Picard–Mann iteration
    // https://doi.org/10.1007/s11071-023-08785-0
    // Fig. 9d - z_w and z_i N(0,1) - fix z_j and z_k to 0
    fn default() -> Self {
        Self {
            c_w: -0.1,
            c_i: 0.6,
            c_j: 0.9,
            c_k: -0.3,
            r: ChaosFloat::sqrt(2.0), // Algoritm 1 R=max(c.norm(), (2/alpha)^(1/n-1))
            n: 3.0,
        }
    }
}

impl Default for JuliaProbabilityComplex {
    fn default() -> Self {
        Self {
            c_re: -0.75,
            c_im: 0.1,
            a: 0.99,
            r: 1.0,
            n: 2.0,
        }
    }
}
impl Default for JuliaProbabilityDual {
    fn default() -> Self {
        Self {
            c_re: -0.75,
            c_im: 0.1,
            a: 0.99,
            r: 1.0,
            n: 2.0,
        }
    }
}
impl Default for JuliaProbabilityPerplex {
    fn default() -> Self {
        Self {
            c_re: -0.75,
            c_im: 0.0,
            a: 0.99,
            r: 2.0,
            n: -2.0,
        }
    }
}
impl Default for JuliaProbabilityQuaternion {
    fn default() -> Self {
        Self {
            c_w: -0.75,
            c_i: 0.1,
            c_j: 0.1,
            c_k: 0.1,
            a: 0.99,
            r: 1.0,
            n: 2.0,
        }
    }
}

impl Default for JuliaSinusComplex {
    fn default() -> Self {
        Self {
            c_re: 0.0,
            c_im: 1.5,
            r: DEFAULT_TRI_R,
            n: 1.0,
        }
    }
}
impl Default for JuliaSinusDual {
    fn default() -> Self {
        Self {
            c_re: 0.0,
            c_im: 1.5,
            r: DEFAULT_TRI_R,
            n: 1.0,
        }
    }
}
impl Default for JuliaSinusPerplex {
    fn default() -> Self {
        Self {
            c_re: 0.0,
            c_im: 1.0,
            r: 0.5,
            n: -2.0,
        }
    }
}
impl Default for JuliaSinusQuaternion {
    fn default() -> Self {
        Self {
            c_w: 0.0,
            c_i: 1.5,
            c_j: 0.0,
            c_k: 0.0,
            r: DEFAULT_TRI_R,
            n: 1.0,
        }
    }
}

impl Default for JuliaSinhComplex {
    fn default() -> Self {
        Self {
            c_re: 0.0,
            c_im: 1.5,
            r: DEFAULT_TRI_R,
            n: 4.0,
        }
    }
}
impl Default for JuliaSinhDual {
    fn default() -> Self {
        Self {
            c_re: 0.0,
            c_im: 1.5,
            r: DEFAULT_TRI_R,
            n: 4.0,
        }
    }
}
impl Default for JuliaSinhPerplex {
    fn default() -> Self {
        Self {
            c_re: 0.1,
            c_im: -1.0,
            r: 15.0,
            n: 3.0,
        }
    }
}
impl Default for JuliaSinhQuaternion {
    fn default() -> Self {
        Self {
            c_w: 0.0,
            c_i: 1.5,
            c_j: 0.0,
            c_k: 0.0,
            r: DEFAULT_TRI_R,
            n: 4.0,
        }
    }
}

impl Default for JuliaZubietaComplex {
    fn default() -> Self {
        Self {
            c_re: 0.0,
            c_im: 1.5,
            r: DEFAULT_TRI_R,
            n: 2.0,
        }
    }
}
impl Default for JuliaZubietaDual {
    fn default() -> Self {
        Self {
            c_re: 0.0,
            c_im: 1.5,
            r: DEFAULT_TRI_R,
            n: 2.0,
        }
    }
}

impl Default for JuliaZubietaPerplex {
    fn default() -> Self {
        Self {
            c_re: 0.5,
            c_im: 0.5,
            r: 20.0,
            n: 3.0,
        }
    }
}
impl Default for JuliaZubietaQuaternion {
    fn default() -> Self {
        Self {
            c_w: 0.0,
            c_i: 1.5,
            c_j: 0.0,
            c_k: 0.0,
            r: DEFAULT_TRI_R,
            n: 2.0,
        }
    }
}

impl Default for MandelbrotPicardComplex {
    fn default() -> Self {
        Self {
            // Figure 25
            a: 0.0,
            alpha: 0.4,
            n: 5.0,
        }
    }
}
impl Default for MandelbrotPicardDual {
    fn default() -> Self {
        Self {
            a: 0.75,
            alpha: 0.5,
            n: 3.0,
        }
    }
}
impl Default for MandelbrotPicardPerplex {
    fn default() -> Self {
        Self {
            a: 0.5,
            alpha: 0.4,
            n: 3.0,
        }
    }
}

impl Default for MandelbrotPicardQuaternion {
    fn default() -> Self {
        // On the quaternion Julia sets via Picard–Mann iteration
        // https://doi.org/10.1007/s11071-023-08785-0
        // Fig. 4 - z_w, z_i and z_k mesh -1 to 1 and 15 samples - fix z_k to 0
        Self {
            a: 0.0,
            alpha: 0.99,
            n: 2.0,
        }
    }
}

impl Default for JuliaPicardComplex {
    fn default() -> Self {
        Self {
            // Figure 31
            c_re: -1.58,
            c_im: -0.2,
            a: 0.0,
            alpha: 0.6,
            n: 2.0,
        }
    }
}
impl Default for JuliaPicardDual {
    fn default() -> Self {
        Self {
            c_re: 0.1,
            c_im: -0.1,
            a: 0.1,
            alpha: 0.01,
            n: 2.0,
        }
    }
}
impl Default for JuliaPicardPerplex {
    fn default() -> Self {
        Self {
            c_re: 0.1,
            c_im: 0.1,
            a: 0.5,
            alpha: 0.5,
            n: 3.0,
        }
    }
}

impl Default for JuliaPicardQuaternion {
    fn default() -> Self {
        // On the quaternion Julia sets via Picard–Mann iteration
        // https://doi.org/10.1007/s11071-023-08785-0
        // Fig. 4 - z_w, z_i and z_k mesh -1 to 1 and 15 samples - fix z_k to 0
        Self {
            c_w: -0.591,
            c_i: -0.399,
            c_j: 0.339,
            c_k: 0.437,
            a: 0.0,
            alpha: 1.0,
            n: 2.0,
        }
    }
}

impl Default for MandelbrotBiomorphComplex {
    fn default() -> Self {
        // Fig. 20 with rational see 4.1.3 (m=0)
        Self {
            r: DEFAULT_BIOMORPH_R,
            m_re: 0.0,
            m_im: 0.0,
            a_re: 0.0,
            a_im: 0.89,
            b_re: 0.1,
            b_im: 0.0,
            alpha: 0.5,
            n: -5.0,
        }
    }
}
impl Default for MandelbrotBiomorphDual {
    fn default() -> Self {
        // TODO Dual
        Self {
            r: DEFAULT_BIOMORPH_R,
            m_re: -0.1,
            m_im: 2.07,
            a_re: DEFAULT_A,
            a_im: 0.0,
            b_re: 0.2,
            b_im: 0.0,
            alpha: 0.48,
            n: 5.0,
        }
    }
}

impl Default for MandelbrotBiomorphPerplex {
    fn default() -> Self {
        Self {
            r: 20.0,
            m_re: 0.1,
            m_im: 0.0,
            a_re: 0.0,
            a_im: 0.0,
            b_re: -0.1,
            b_im: 0.0,
            alpha: 0.9,
            n: -2.0,
        }
    }
}
impl Default for MandelbrotBiomorphQuaternion {
    fn default() -> Self {
        // On the quaternion Julia sets via Picard–Mann iteration
        // https://doi.org/10.1007/s11071-023-08785-0
        // Fig. 17a - z_w and z_i N(0,1) - fix z_j and z_k to 0
        Self {
            r: 2.0,
            m_w: 1.0,
            m_i: 0.0,
            m_j: 0.0,
            m_k: 0.0,
            a_w: 0.1,
            a_i: 0.0,
            a_j: 0.0,
            a_k: 0.0,
            b_w: 0.1,
            b_i: 0.0,
            b_j: 0.0,
            b_k: 0.0,
            alpha: 0.4,
            n: 5.0,
        }
    }
}

impl Default for JuliaBiomorphComplex {
    fn default() -> Self {
        // Fig. 20 with rational see 4.1.3 (m=0)
        Self {
            r: DEFAULT_BIOMORPH_R,
            c_re: 0.3241,
            c_im: -0.08743,
            m_re: 0.0,
            m_im: 0.0,
            a_re: 0.0,
            a_im: 0.89,
            b_re: 0.1,
            b_im: 0.0,
            alpha: 0.5,
            n: -5.0,
        }
    }
}
impl Default for JuliaBiomorphDual {
    fn default() -> Self {
        // TODO Dual
        Self {
            r: DEFAULT_BIOMORPH_R,
            c_re: -0.01,
            c_im: 0.008,
            m_re: -0.1,
            m_im: 2.07,
            a_re: DEFAULT_A,
            a_im: 0.0,
            b_re: 0.2,
            b_im: 0.0,
            alpha: 0.48,
            n: 5.0,
        }
    }
}
impl Default for JuliaBiomorphPerplex {
    fn default() -> Self {
        Self {
            r: 20.0,
            c_re: -0.1,
            c_im: 1.0,
            m_re: -0.1,
            m_im: 1.0,
            a_re: 0.1,
            a_im: -0.1,
            b_re: 0.1,
            b_im: -0.2,
            alpha: 0.3,
            n: -2.0,
        }
    }
}
impl Default for JuliaBiomorphQuaternion {
    fn default() -> Self {
        // On the quaternion Julia sets via Picard–Mann iteration
        // https://doi.org/10.1007/s11071-023-08785-0
        // Fig. 8 - z_w and z_i N(0,1) - fix z_j and z_k to 0
        Self {
            r: 30.0,
            c_w: -0.1,
            c_i: 0.6,
            c_j: 0.9,
            c_k: -0.3,
            m_w: 0.0,
            m_i: 0.0,
            m_j: 0.0,
            m_k: 0.0,
            a_w: 1.0,
            a_i: 0.0,
            a_j: 0.0,
            a_k: 0.0,
            b_w: 0.0,
            b_i: 0.0,
            b_j: 0.0,
            b_k: 0.0,
            alpha: 0.08,
            n: 3.0,
        }
    }
}
