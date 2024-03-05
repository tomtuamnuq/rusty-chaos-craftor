use super::fractal_conf::*;
use crate::chaos::data::*;
use crate::chaos::functions::*;
use crate::chaos::labels::{ChaosDescription, ChaosFormula};
use paste::paste;

trait PrettyPrintElem {
    fn pretty_print(&self) -> String;
}

impl PrettyPrintElem for Complex {
    fn pretty_print(&self) -> String {
        format!("Complex({:.2}, {:.2})", self.re, self.im)
    }
}
impl PrettyPrintElem for Dual {
    fn pretty_print(&self) -> String {
        format!("Dual({:.2}, {:.2})", self.re, self.eps)
    }
}
impl PrettyPrintElem for Perplex {
    fn pretty_print(&self) -> String {
        format!("Perplex({:.2}, {:.2})", self.t, self.x)
    }
}
impl PrettyPrintElem for Quaternion {
    fn pretty_print(&self) -> String {
        format!(
            "Q({:.2}, {:.2}, {:.2}, {:.2})",
            self.w, self.i, self.j, self.k
        )
    }
}

macro_rules! implement_description {
    ($($elem:ident ),*)=> {
        const LABEL_ITER_ESCAPE_COLOR: &str = "Iteration criteria: num iter < 255";
        const LABEL_ITER_ESCAPE_PICARD: &str = "Iteration criteria: num iter < 30";
        const LABEL_ITER_ESCAPE_BIOMORPH: &str = "Iteration criteria: num iter < 10";
        paste!{
            $(
                impl ChaosDescription for [<MandelbrotPower $elem>] {
                    fn description(&self) -> String{
                        format!("The well-known Mandelbrot set generator for a polynomial f(z)=z^n+z0. Chosen is n={}", self.power_n())
                    }
                    fn reference(&self) -> &'static str{
                        "https://wikipedia.org/wiki/Mandelbrot_set"
                    }
                }
                impl ChaosFormula for [<MandelbrotPower $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= z^n + z0",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }
                impl ChaosDescription for [<JuliaPower $elem>] {
                    fn description(&self) -> String{
                        format!("The well-known Julia set generator for a polynomial f(z)=z^n+c. Chosen is n={} and c={}", self.power_n(), self.c().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://wikipedia.org/wiki/Julia_set"
                    }
                }
                impl ChaosFormula for [<JuliaPower $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= z^n + c",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }

                impl ChaosDescription for [<MandelbrotProbability $elem>] {
                    fn description(&self) -> String{
                        format!("Adjusted probability reverse Julia adaptation for a Mandelbrot set. Inspired by Roger Bagula and Paul Bourke. Chosen is n={} and a={:.2}", self.power_n(), self.par_a())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/reversejulia/"
                    }
                }
                impl ChaosFormula for [<MandelbrotProbability $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "p= Uniform(0, 1)",
                            "s= 1 if p > a else -1",
                            "z re= s (||z - z0||)^(1/n) cos(arg(z)/2)",
                            "z im= s (||z - z0||)^(1/n) sin(arg(z)/2)",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }
                impl ChaosDescription for [<JuliaProbability $elem>] {
                    fn description(&self) -> String{
                        format!("Adjusted probability reverse Julia by Roger Bagula and Paul Bourke. Chosen is n={}, a={:.2} and c={}", self.power_n(), self.par_a(), self.c().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/reversejulia/"
                    }
                }
                impl ChaosFormula for [<JuliaProbability $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "p= Uniform(0, 1)",
                            "s= 1 if p > a else -1",
                            "z re= s (||z - c||)^(1/n) cos(arg(z)/2)",
                            "z im= s (||z - c||)^(1/n) sin(arg(z)/2)",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }

                impl ChaosDescription for [<MandelbrotSinus $elem>] {
                    fn description(&self) -> String{
                        format!("Mandelbrot set generator for the repeated multiplication of the sinus of a power of z. Inspired by Paul Bourke. Chosen is n={}", self.power_n())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/sinjulia/"
                    }
                }
                impl ChaosFormula for [<MandelbrotSinus $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= z0 sin(z^n)",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }
                impl ChaosDescription for [<JuliaSinus $elem>] {
                    fn description(&self) -> String{
                        format!("Julia set generator for the repeated multiplication of 'c' with the sinus of a power of z. Inspired by Paul Bourke. Chosen is n={} and c={}", self.power_n(), self.c().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/sinjulia/"
                    }
                }
                impl ChaosFormula for [<JuliaSinus $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= c sin(z^n)",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }

                impl ChaosDescription for [<MandelbrotSinh $elem>] {
                    fn description(&self) -> String{
                        format!("Mandelbrot set generator for the component-wise absolute value of the power of the sinus hyperbolicus of z. Inspired by Paul Bourke and Whittaker Courtney. Chosen is n={}", self.power_n())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/sinh/"
                    }
                }
                impl ChaosFormula for [<MandelbrotSinh $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= abs(sinh(z)^n) + z0",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }
                impl ChaosDescription for [<JuliaSinh $elem>] {
                    fn description(&self) -> String{
                        format!("Julia set generator for the component-wise absolute value of the power of the sinus hyperbolicus of z. Inspired by Paul Bourke and Whittaker Courtney. Chosen is n={} and c={}", self.power_n(), self.c().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/sinh/"
                    }
                }
                impl ChaosFormula for [<JuliaSinh $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= abs(sinh(z)^n) + c",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }

                impl ChaosDescription for [<MandelbrotZubieta $elem>] {
                    fn description(&self) -> String{
                        format!("Mandelbrot set generator for the power of z plus the result of division between z0 and z. Inspired by Paul Bourke and Santiago Zubieta. Chosen is n={}", self.power_n())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/Zubieta/"
                    }
                }
                impl ChaosFormula for [<MandelbrotZubieta $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= z^n + z0 / z",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }
                impl ChaosDescription for [<JuliaZubieta $elem>] {
                    fn description(&self) -> String{
                        format!("Julia set generator for the power of z plus the result of division between c and z. Inspired by Paul Bourke and Santiago Zubieta. Chosen is n={} and c={}", self.power_n(), self.c().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://paulbourke.net/fractals/Zubieta/"
                    }
                }
                impl ChaosFormula for [<JuliaZubieta $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "z= z^n + c / z",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_COLOR
                        ]
                    }
                }

                impl ChaosDescription for [<MandelbrotPicard $elem>] {
                    fn description(&self) -> String{
                        format!("Picard-Mann iteration for the generation of Mandelbrot sets for the polynomial f(z)=z^n + a z + z0. This is an adaptation of Algorithm 1 in 'On the quaternion Julia sets via Picard–Mann iteration' to the Mandelbrot set and extended by adding a linearly scaled z term with a={:.2}. Chosen is n={} and α={:.2}", self.par_a(), self.power_n(), self.alpha())
                    }
                    fn reference(&self) -> &'static str{
                        "https://doi.org/10.1007/s11071-023-08785-0"
                    }
                }
                impl ChaosFormula for [<MandelbrotPicard $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "f(z)= z^n + a z + z0",
                            "z= f( (1 - α) z + α f(z) )",
                            "z0 from initial distribution",
                            "r= max(|z0|, (2 / α)^(n - 1) )",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_PICARD
                        ]
                    }
                }
                impl ChaosDescription for [<JuliaPicard $elem>] {
                    fn description(&self) -> String{
                        format!("Picard-Mann iteration for the generation of Julia sets for the polynomial f(z)=z^n + a z + c. This is an extension of Algorithm 1 in 'On the quaternion Julia sets via Picard–Mann iteration' by adding a linearly scaled z term with a={:.2}. Chosen is n={}, α={:.2} and c={}", self.par_a(), self.power_n(), self.alpha(), self.c().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://doi.org/10.1007/s11071-023-08785-0"
                    }
                }
                impl ChaosFormula for [<JuliaPicard $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "f(z)= z^n + a z + c",
                            "z= f( (1 - α) z + α f(z) )",
                            "z0 from initial distribution",
                            "r= max(|c|, (2 / α)^(n - 1) )",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_PICARD
                        ]
                    }
                }

                impl ChaosDescription for [<MandelbrotBiomorph $elem>] {
                    fn description(&self) -> String{
                        format!("An adaptation of Algorithm 3 in 'A novel approach to generate Mandelbrot sets, Julia sets and biomorphs via viscosity approximation method' for biomorphic Mandelbrot sets with the polynomial w(z)= z^n + m z + z0. Chosen are n={}, m={} and α={:.2}. Viscosity approximation function g(z)= a z + b is set with a= {} and b= {}", self.power_n(), self.m().pretty_print(), self.alpha(), self.a().pretty_print(), self.b().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://doi.org/10.1016/j.chaos.2022.112540"
                    }
                }
                impl ChaosFormula for [<MandelbrotBiomorph $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "w(z)= z^n + m z + z0",
                            "g(z)= a z + b",
                            "z= (1 - α) w(z) + α g(z)",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_BIOMORPH,
                            "Biomorph criteria: abs(z_re) < r or abs(z_im) < r"
                        ]
                    }
                }
                impl ChaosDescription for [<JuliaBiomorph $elem>] {
                    fn description(&self) -> String{
                        format!("An application of Algorithm 3 in 'A novel approach to generate Mandelbrot sets, Julia sets and biomorphs via viscosity approximation method' with the polynomial w(z)= z^n + m z + c. Chosen are n={}, m={} and α={:.2} with c={}. Viscosity approximation function g(z)= a z + b is set with a= {} and b= {}", self.power_n(), self.m().pretty_print(), self.alpha(), self.c().pretty_print(), self.a().pretty_print(), self.b().pretty_print())
                    }
                    fn reference(&self) -> &'static str{
                        "https://doi.org/10.1016/j.chaos.2022.112540"
                    }
                }
                impl ChaosFormula for [<JuliaBiomorph $elem>]{
                    fn formula(&self) -> &[&'static str]{
                        &[
                            "w(z)= z^n + m z + c",
                            "g(z)= a z + b",
                            "z= (1 - α) w(z) + α g(z)",
                            "z0 from initial distribution",
                            "Bounding criteria: |z| < r",
                            LABEL_ITER_ESCAPE_BIOMORPH,
                            "Biomorph criteria: abs(z_re) < r or abs(z_im) < r"
                        ]
                    }
                }
            )*
        }
    };
}

implement_description! {
    Complex, Dual, Perplex, Quaternion
}
