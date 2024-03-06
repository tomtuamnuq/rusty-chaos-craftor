use super::execute::SelectedChaoticFunction;
use crate::gui::tooltips::*;
use crate::gui::*;
use crate::chaos::functions as chaotic_function_configs;
use crate::chaos::{
    data::*,
    fractal::*,
    labels::{ChaosDescription, ChaosFormula},
    DiscreteMapVec, OdeSolver, OdeSystemSolverVec, ParticleXYSystemSolver, ParticleXYZSystemSolver,
    SimpleDiscreteMap,
};

use egui::Ui;
use paste::paste;
use strum_macros::EnumIter;

const PARAMETER_DELTA: f64 = 0.1;
const PARAMETER_RANGE_NUM: usize = 200;

fn parameter_view_single(
    par: &mut f64,
    suffix: &str,
    (par_min, par_max): (f64, f64),
    ui: &mut Ui,
) -> bool {
    let (par_min, par_max) = limit_par_range(par_min, par_max);
    let response = ui.add(
        egui::DragValue::new(par)
            .speed(PARAMETER_DELTA)
            .clamp_range(par_min..=par_max) // Range inclusive
            .suffix(format!(" {}", suffix)),
    );
    response.changed()
}

fn limit_par_range(par_min: f64, par_max: f64) -> (f64, f64) {
    let (mut par_min, par_max) = (par_min.max(PARAMETER_MIN), par_max.min(PARAMETER_MAX));
    if par_min == par_max {
        par_min = par_max - PARAMETER_DELTA;
    }
    (par_min, par_max)
}
fn parameter_view_ranged(
    chosen_par_range: &mut (f64, f64),
    num_params: &mut usize,
    suffix: &str,
    (total_par_min, total_par_max): (f64, f64),
    ui: &mut Ui,
) -> bool {
    let (total_par_min, total_par_max) = limit_par_range(total_par_min, total_par_max);
    let response = ui.add(
        egui::DragValue::new(&mut chosen_par_range.0)
            .speed(PARAMETER_DELTA) // TODO depend on num params
            .clamp_range(total_par_min..=total_par_max) // Range inclusive
            .suffix(format!("Min {}", suffix)),
    );
    let par_range_min_changed = response.changed();
    let response = ui.add(
        egui::DragValue::new(&mut chosen_par_range.1)
            .speed(PARAMETER_DELTA)
            .clamp_range(total_par_min..=total_par_max) // Range inclusive
            .suffix(format!("Max {}", suffix)),
    );
    integer_slider(
        LABEL_NUM_PARAMS,
        num_params,
        PARAMETER_RANGE_NUM,
        ui,
        TIP_NUM_PARAMS,
    );
    par_range_min_changed || response.changed()
}

fn parameter_linspace(par_min: f64, par_max: f64, num_params: usize) -> Vec<f64> {
    let conf = Linspace {
        low: par_min,
        high: par_max,
    };
    linspace(num_params, &conf)
}

macro_rules! create_and_implement_map_view_variants {
    ([$( $discrete_map:ident $discrete_state:expr),*] [$( $fractal_fn:ident),*] [$( $continuous_ode:ident $continuous_state:expr),*] [$( $particle_dim:ident),*]) => {
        paste!{
            #[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter)]
            pub enum DiscreteMapView {
                #[default]
                $(
                    $discrete_map,
                )*
                $(
                    [<Mandelbrot $fractal_fn Complex>],
                    [<Mandelbrot $fractal_fn Dual>],
                    [<Mandelbrot $fractal_fn Perplex>],
                    [<Mandelbrot $fractal_fn Quaternion>],
                    [<Julia $fractal_fn Complex>],
                    [<Julia $fractal_fn Dual>],
                    [<Julia $fractal_fn Perplex>],
                    [<Julia $fractal_fn Quaternion>],
                )*
            }
            impl From<DiscreteMapView> for &'static str {
                fn from(val: DiscreteMapView) -> Self {
                    match val {
                        $(
                            DiscreteMapView::$discrete_map => stringify!($discrete_map),
                        )*
                        $(
                            DiscreteMapView::[<Mandelbrot $fractal_fn Complex>] => stringify!([<$fractal_fn>]),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Dual>] => stringify!([<$fractal_fn>]),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Perplex>] => stringify!([<$fractal_fn>]),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Quaternion>] => stringify!([<$fractal_fn>]),
                            DiscreteMapView::[<Julia $fractal_fn Complex>] => stringify!([<$fractal_fn>]),
                            DiscreteMapView::[<Julia $fractal_fn Dual>] => stringify!([<$fractal_fn>]),
                            DiscreteMapView::[<Julia $fractal_fn Perplex>] => stringify!([<$fractal_fn>]),
                            DiscreteMapView::[<Julia $fractal_fn Quaternion>] => stringify!([<$fractal_fn>]),
                        )*
                    }
                }
            }

            impl DiscreteMapView {
                pub fn dimensionality(&self) -> DistributionDimensions {
                    match self {
                        $(
                            Self::$discrete_map => [<DIMS_STATE $discrete_state>],
                        )*
                        $(
                            Self::[<Mandelbrot $fractal_fn Complex>] => DIMS_FRACTALCOMPLEX,
                            Self::[<Mandelbrot $fractal_fn Dual>] => DIMS_FRACTALDUAL,
                            Self::[<Mandelbrot $fractal_fn Perplex>] => DIMS_FRACTALPERPLEX,
                            Self::[<Mandelbrot $fractal_fn Quaternion>] => DIMS_FRACTALQUATERNION,
                            Self::[<Julia $fractal_fn Complex>] =>  DIMS_FRACTALCOMPLEX,
                            Self::[<Julia $fractal_fn Dual>] => DIMS_FRACTALDUAL,
                            Self::[<Julia $fractal_fn Perplex>] => DIMS_FRACTALPERPLEX,
                            Self::[<Julia $fractal_fn Quaternion>] => DIMS_FRACTALQUATERNION,
                        )*
                    }
                }
                pub fn is_mandelbrot(&self) -> bool {
                    match self {
                        $(
                            Self::[<Mandelbrot $fractal_fn Complex>] => true,
                            Self::[<Mandelbrot $fractal_fn Dual>] => true,
                            Self::[<Mandelbrot $fractal_fn Perplex>] => true,
                            Self::[<Mandelbrot $fractal_fn Quaternion>] => true,
                        )*
                        _ => false
                    }
                }
                pub fn is_julia(&self) -> bool {
                    match self {
                        $(
                            Self::[<Julia $fractal_fn Complex>] => true,
                            Self::[<Julia $fractal_fn Dual>] => true,
                            Self::[<Julia $fractal_fn Perplex>] => true,
                            Self::[<Julia $fractal_fn Quaternion>] => true,
                        )*
                        _ => false
                    }
                }
            }

            #[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter)]
            pub enum DifferentialSystemView {
                #[default]
                $(
                    $continuous_ode,
                )*
                $(
                    [<Particle $particle_dim>],
                )*
            }

            impl From<DifferentialSystemView> for &'static str {
                fn from(val: DifferentialSystemView) -> Self {
                    match val {
                        $(
                            DifferentialSystemView::$continuous_ode => stringify!($continuous_ode),
                        )*
                        $(
                            DifferentialSystemView::[<Particle $particle_dim>] => stringify!($particle_dim),
                        )*
                    }
                }
            }

            impl DifferentialSystemView {
                pub fn dimensionality(&self) -> DistributionDimensions {
                    match self {
                        $(
                            Self::$continuous_ode => [<DIMS_STATE $continuous_state>],
                        )*
                        $(
                            Self::[<Particle $particle_dim>] => [<DIMS_PARTICLE $particle_dim>],
                        )*
                    }
                }
            }
            #[allow(non_snake_case)] // for ease of copy paste
            #[derive(Default, PartialEq)]
            pub struct ChaosFunctionViewData {
                $(
                    [<$discrete_map:lower>]: $discrete_map,
                )*
                $(
                    [<mandelbrot $fractal_fn Complex>]: [<Mandelbrot $fractal_fn Complex>],
                    [<mandelbrot $fractal_fn Dual>]: [<Mandelbrot $fractal_fn Dual>],
                    [<mandelbrot $fractal_fn Perplex>]: [<Mandelbrot $fractal_fn Perplex>],
                    [<mandelbrot $fractal_fn Quaternion>]: [<Mandelbrot $fractal_fn Quaternion>],
                    [<julia $fractal_fn Complex>]: [<Julia $fractal_fn Complex>],
                    [<julia $fractal_fn Dual>]: [<Julia $fractal_fn Dual>],
                    [<julia $fractal_fn Perplex>]: [<Julia $fractal_fn Perplex>],
                    [<julia $fractal_fn Quaternion>]: [<Julia $fractal_fn Quaternion>],
                )*
                $(
                    [<$continuous_ode:lower>]: $continuous_ode,
                )*
                $(
                    [<particle $particle_dim>]: [<Particle $particle_dim>],
                )*
            }


            impl ChaosFunctionViewData {
                pub fn discrete_description(& self, view: &DiscreteMapView) -> (String, &'static str){
                    match view {
                        $(
                            DiscreteMapView::$discrete_map => {
                                let data = &self.[<$discrete_map:lower>].data;
                                (data.description(), data.reference())
                            },
                        )*
                        $(
                            DiscreteMapView::[<Mandelbrot $fractal_fn Complex>] => {
                                let data = &self.[<mandelbrot $fractal_fn Complex>].data;
                                (data.description(), data.reference())
                            },
                            DiscreteMapView::[<Mandelbrot $fractal_fn Dual>] => {
                                let data = &self.[<mandelbrot $fractal_fn Dual>].data;
                                (data.description(), data.reference())
                            },
                            DiscreteMapView::[<Mandelbrot $fractal_fn Perplex>] => {
                                let data = &self.[<mandelbrot $fractal_fn Perplex>].data;
                                (data.description(), data.reference())
                            },
                            DiscreteMapView::[<Mandelbrot $fractal_fn Quaternion>] => {
                                let data = &self.[<mandelbrot $fractal_fn Quaternion>].data;
                                (data.description(), data.reference())
                            },
                            DiscreteMapView::[<Julia $fractal_fn Complex>] => {
                                let data = &self.[<julia $fractal_fn Complex>].data;
                                (data.description(), data.reference())
                            },
                            DiscreteMapView::[<Julia $fractal_fn Dual>] => {
                                let data = &self.[<julia $fractal_fn Dual>].data;
                                (data.description(), data.reference())
                            },
                            DiscreteMapView::[<Julia $fractal_fn Perplex>] => {
                                let data = &self.[<julia $fractal_fn Perplex>].data;
                                (data.description(), data.reference())
                            },
                            DiscreteMapView::[<Julia $fractal_fn Quaternion>] => {
                                let data = &self.[<julia $fractal_fn Quaternion>].data;
                                (data.description(), data.reference())
                            },
                        )*
                    }
                }
                pub fn discrete_view_ui(&mut self, view: &DiscreteMapView, ui: &mut Ui) {
                    match view {
                        $(
                            DiscreteMapView::$discrete_map => self.[<$discrete_map:lower>].ui(ui),
                        )*
                        $(
                            DiscreteMapView::[<Mandelbrot $fractal_fn Complex>] => self.[<mandelbrot $fractal_fn Complex>].ui(ui),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Dual>] => self.[<mandelbrot $fractal_fn Dual>].ui(ui),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Perplex>] => self.[<mandelbrot $fractal_fn Perplex>].ui(ui),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Quaternion>] => self.[<mandelbrot $fractal_fn Quaternion>].ui(ui),
                            DiscreteMapView::[<Julia $fractal_fn Complex>] => self.[<julia $fractal_fn Complex>].ui(ui),
                            DiscreteMapView::[<Julia $fractal_fn Dual>] => self.[<julia $fractal_fn Dual>].ui(ui),
                            DiscreteMapView::[<Julia $fractal_fn Perplex>] => self.[<julia $fractal_fn Perplex>].ui(ui),
                            DiscreteMapView::[<Julia $fractal_fn Quaternion>] => self.[<julia $fractal_fn Quaternion>].ui(ui),
                        )*
                    }
                }
                pub fn map_discrete_view_to_maps_vec_variant(
                    &self,
                    view: &DiscreteMapView,
                ) -> SelectedChaoticFunction {
                    match view {
                        $(
                            DiscreteMapView::$discrete_map => SelectedChaoticFunction::from(self.[<$discrete_map:lower>].clone()),
                        )*
                        $(
                            DiscreteMapView::[<Mandelbrot $fractal_fn Complex>] => SelectedChaoticFunction::from(self.[<mandelbrot $fractal_fn Complex>].clone()),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Dual>] => SelectedChaoticFunction::from(self.[<mandelbrot $fractal_fn Dual>].clone()),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Perplex>] => SelectedChaoticFunction::from(self.[<mandelbrot $fractal_fn Perplex>].clone()),
                            DiscreteMapView::[<Mandelbrot $fractal_fn Quaternion>] => SelectedChaoticFunction::from(self.[<mandelbrot $fractal_fn Quaternion>].clone()),
                            DiscreteMapView::[<Julia $fractal_fn Complex>] => SelectedChaoticFunction::from(self.[<julia $fractal_fn Complex>].clone()),
                            DiscreteMapView::[<Julia $fractal_fn Dual>] => SelectedChaoticFunction::from(self.[<julia $fractal_fn Dual>].clone()),
                            DiscreteMapView::[<Julia $fractal_fn Perplex>] => SelectedChaoticFunction::from(self.[<julia $fractal_fn Perplex>].clone()),
                            DiscreteMapView::[<Julia $fractal_fn Quaternion>] => SelectedChaoticFunction::from(self.[<julia $fractal_fn Quaternion>].clone()),
                        )*
                    }
                }
                pub fn continuous_description(&self, view: &DifferentialSystemView) -> (String, &'static str) {
                    match view {
                        $(
                            DifferentialSystemView::$continuous_ode => {
                                let data = &self.[<$continuous_ode:lower>].data;
                                (data.description(), data.reference())
                            },
                        )*
                        $(
                            DifferentialSystemView::[<Particle $particle_dim>] =>{
                                let data = &self.[<particle $particle_dim>].data;
                                (data.description(), data.reference())
                            },
                        )*
                    }
                }
                pub fn continuous_view_ui(&mut self, view: &DifferentialSystemView, ui: &mut Ui) {
                    match view {
                        $(
                            DifferentialSystemView::$continuous_ode => self.[<$continuous_ode:lower>].ui(ui),
                        )*
                        $(
                            DifferentialSystemView::[<Particle $particle_dim>] => self.[<particle $particle_dim>].ui(ui),
                        )*
                    }
                }

                pub fn map_continuous_view_to_solver_vec_variant(
                    &self,
                    view: &DifferentialSystemView,
                ) -> SelectedChaoticFunction {
                    match view {
                        $(
                            DifferentialSystemView::$continuous_ode => SelectedChaoticFunction::from(self.[<$continuous_ode:lower>].clone()),
                        )*
                        $(
                            DifferentialSystemView::[<Particle $particle_dim>] => SelectedChaoticFunction::from(self.[<particle $particle_dim>].clone()),
                        )*
                    }
                }

            }
        } // paste
    };
}
create_and_implement_map_view_variants! {
    [
        Logistic 1,
        Tent 1,
        Gauss 1,
        Circle 1,
        Chirikov 2,
        Henon 2,
        ArnoldsCat 2,
        Bogdanov 2,
        Chialvo 2,
        DeJongRing 2,
        Duffing 2,
        Tinkerbell 2,
        Baker 2,
        Clifford 2,
        Ikeda 2,
        Gingerbreadman 2,
        KaplanYorke 2,
        Rulkov 2,
        Zaslavskii 2,
        Shah 3,
        Memristive 3,
        Sfsimm 4
    ]
    [Power, Probability, Sinus, Sinh, Zubieta, Picard, Biomorph]
    [
        Brusselator 2,
        VanDerPol 2,
        QuadrupTwoOrbit 2,
        Lorenz 3,
        Rossler 3,
        Chen 3,
        Aizawa 3,
        ChuasCircuit 3,
        RabinovichFabrikant 3,
        GenesioTesi 3,
        BurkeShaw 3,
        Halvorsen 3,
        ThreeSpeciesLotkaVolterra 3,
        Rikitake 3,
        HindmarshRose 3,
        Ababneh 4,
        WeiWang 4
    ]
    [XY, XYZ]
}

macro_rules! generate_view_variant {
    ($variant:ident { $([$field:ident, $field_label:expr]),* }) => {
        paste!{
            #[derive(PartialEq, Clone)]
            pub struct $variant {
                data: chaotic_function_configs::$variant,
                num_params: usize,
                $([<range_ $field>]: Option<(f64,f64)>,)*
            }

            impl $variant {
                #[allow(dead_code)]
                fn reset_ranges(&mut self){
                    $( self.[<range_ $field>] = None ;)*
                }
                #[allow(unused)]
                pub fn ui(&mut self, ui: &mut Ui) {
                    ui.collapsing("Info", |ui| {
                        ui.label(self.data.description());
                        group_vertical(ui, |ui|{
                            ui.horizontal(|ui|{
                                ui.heading("Formula ");
                                ui.hyperlink_to(stringify!($variant), self.data.reference());
                            });
                            self.data.formula().iter().for_each(|l| {
                                ui.label(*l);
                            })
                        });
                    });
                    #[allow(unused_mut)]
                    let mut par_changed = false;
                    $(
                        group_horizontal(ui,|ui| {
                            let par_label = $field_label;
                            let allowed_range = chaotic_function_configs::$variant::[<RANGE_ $field:upper>];
                            let is_no_range = self.[<range_ $field>].is_none();
                            let range_label = format!("Range {}", par_label);
                            let tooltip = if is_no_range{
                                format!("Toggle to specify an evenly spaced range over {} (Linspace). This may create a bifurcation diagram. The current chaotic data distribution is cloned for each parameter value.", par_label)
                            } else{
                                format!("Toggle to deactivate the range over {}. Toggling off takes the data set from the parameter with the smallest value and continuous with the previously selected single parameter.", par_label)
                            };
                            if clickable_button(range_label.as_str(), !is_no_range,true, ui, tooltip.as_str()){
                                self.reset_ranges();
                                if is_no_range{
                                    self.[<range_ $field>] = Some(allowed_range);
                                }
                            }
                            let field_par_changed = if let Some(par_range) = self.[<range_ $field>].as_mut(){
                                parameter_view_ranged(par_range, &mut self.num_params, par_label, allowed_range, ui)
                            }else{
                                parameter_view_single(&mut self.data.$field, par_label, allowed_range, ui)
                            };
                            par_changed = par_changed || field_par_changed;
                        });
                    )*
                    if par_changed {
                        self.data.par_range_check();
                    };
                }
            }
            impl Default for $variant{
                fn default()->Self{
                    Self{
                        data: Default::default(),
                        num_params: 10,
                        $([<range_ $field>]: None,)*
                    }
                }
            }
        }
    };
}
macro_rules! impl_discrete_variants {
    ($($variant:ident, $mapper:ident, { $([$field:ident, $field_label:expr]),* }),*) => {
        $(
            paste!{
                generate_view_variant!{
                    $variant { $([$field, $field_label]),* }
                }
                impl From<$variant> for SelectedChaoticFunction{
                    fn from(val: $variant)->Self{
                        $(
                            if let Some((par_min, par_max)) = val.[<range_ $field>]{
                                let par_values = parameter_linspace(par_min, par_max, val.num_params);
                                let discrete_maps = par_values.iter().map(|par|{
                                    let mut pars = val.data.clone();
                                    pars.$field = *par;
                                    $mapper::new(pars)
                                }).collect();
                                let discrete_vec = DiscreteMapVec::$variant(discrete_maps);
                                return SelectedChaoticFunction::ParametrizedDiscreteMaps(discrete_vec, stringify!($field), par_values);
                            }
                        )*
                        SelectedChaoticFunction::SingleDiscreteMap(DiscreteMapVec::$variant(vec![$mapper::new(val.data.clone())]))
                    }
                }
            }
        )*
    };
}

impl_discrete_variants! {
    Logistic, SimpleDiscreteMap, { [r, "r"] },
    Tent, SimpleDiscreteMap, {  [mu, "μ"] },
    Gauss, SimpleDiscreteMap, { [alpha, "α"], [beta, "β"] },
    Circle, SimpleDiscreteMap, { [omega, "ω"], [k, "k"] },
    Chirikov, SimpleDiscreteMap, { [k, "k"] },
    Henon, SimpleDiscreteMap, { [a, "a"], [b, "b"] },
    ArnoldsCat, SimpleDiscreteMap, {  },
    Bogdanov, SimpleDiscreteMap, { [eps, "ε"], [k, "k"],  [mu, "μ"] },
    Chialvo, SimpleDiscreteMap, { [a, "a"], [b, "b"] },
    DeJongRing, SimpleDiscreteMap, { },
    Duffing, SimpleDiscreteMap, { [a, "a"], [b, "b"] },
    Tinkerbell, SimpleDiscreteMap, { [a, "a"], [b, "b"], [c, "c"], [d, "d"] },
    Baker, SimpleDiscreteMap, {  },
    Clifford, SimpleDiscreteMap, { [a, "a"], [b, "b"], [c, "c"], [d, "d"] },
    Ikeda, SimpleDiscreteMap, { [u, "u"] },
    Gingerbreadman, SimpleDiscreteMap, {  },
    KaplanYorke, SimpleDiscreteMap, { [alpha, "α"] },
    Rulkov, SimpleDiscreteMap, { [alpha, "α"],  [mu, "μ"], [delta, "δ"] },
    Zaslavskii, SimpleDiscreteMap, { [eps, "ε"], [nu, "ν"], [r, "r"] },
    Shah, SimpleDiscreteMap, { [alpha, "α"], [beta, "β"], [gamma, "γ"], [delta, "δ"] },
    Memristive, SimpleDiscreteMap, { [k, "k"], [a, "a"] },
    Sfsimm, SimpleDiscreteMap, { [p, "p"], [b, "b"], [r, "r"] },
    MandelbrotPowerComplex, MandelbrotPower, { [r, "r"], [n, "n"] },
    MandelbrotProbabilityComplex, MandelbrotProbability, { [a, "a"], [r, "r"], [n, "n"] },
    MandelbrotSinusComplex, MandelbrotSinus, { [r, "r"], [n, "n"] },
    MandelbrotSinhComplex, MandelbrotSinh, { [r, "r"], [n, "n"] },
    MandelbrotZubietaComplex, MandelbrotZubieta, { [r, "r"], [n, "n"] },
    MandelbrotPicardComplex, MandelbrotPicard,  { [a, "a"], [alpha, "α"], [n, "n"]  },
    MandelbrotBiomorphComplex, MandelbrotBiomorph, { [r, "r"], [m_re, "m re"], [m_im, "m i"], [a_re, "a re"], [a_im, "a i"], [b_re, "b re"], [b_im, "b i"], [alpha, "α"], [n, "n"]  },
    JuliaPowerComplex, JuliaPower, { [c_re, "c re"], [c_im, "c i"], [r, "r"], [n, "n"] },
    JuliaProbabilityComplex, JuliaProbability, { [c_re, "c re"], [c_im, "c i"], [a, "a"], [r, "r"], [n, "n"] },
    JuliaSinusComplex, JuliaSinus, { [c_re, "c re"], [c_im, "c i"], [r, "r"], [n, "n"] },
    JuliaSinhComplex, JuliaSinh, { [c_re, "c re"], [c_im, "c i"], [r, "r"], [n, "n"] },
    JuliaZubietaComplex, JuliaZubieta, { [c_re, "c re"], [c_im, "c i"], [r, "r"], [n, "n"] },
    JuliaPicardComplex, JuliaPicard, {  [a, "a"], [c_re, "c re"], [c_im, "c i"], [alpha, "α"], [n, "n"]  },
    JuliaBiomorphComplex, JuliaBiomorph, { [r, "r"], [c_re, "c re"], [c_im, "c i"], [m_re, "m re"], [m_im, "m i"], [a_re, "a re"], [a_im, "a i"], [b_re, "b re"], [b_im, "b i"], [alpha, "α"], [n, "n"]  },
    MandelbrotPowerDual, MandelbrotPower, { [r, "r"], [n, "n"] },
    MandelbrotProbabilityDual, MandelbrotProbability, { [a, "a"], [r, "r"], [n, "n"] },
    MandelbrotSinusDual, MandelbrotSinus, { [r, "r"], [n, "n"]},
    MandelbrotSinhDual, MandelbrotSinh, { [r, "r"], [n, "n"]},
    MandelbrotZubietaDual, MandelbrotZubieta, { [r, "r"], [n, "n"]},
    MandelbrotPicardDual, MandelbrotPicard, {  [a, "a"], [alpha, "α"], [n, "n"]   },
    MandelbrotBiomorphDual, MandelbrotBiomorph, { [r, "r"], [m_re, "m re"], [m_im, "m ε"], [a_re, "a re"], [a_im, "a ε"], [b_re, "b re"], [b_im, "b ε"], [alpha, "α"], [n, "n"]   },
    JuliaPowerDual, JuliaPower, { [c_re, "c re"], [c_im, "c ε"], [r, "r"], [n, "n"] },
    JuliaProbabilityDual, JuliaProbability, { [c_re, "c re"], [c_im, "c ε"], [a, "a"], [r, "r"], [n, "n"] },
    JuliaSinusDual, JuliaSinus, { [c_re, "c re"], [c_im, "c ε"], [r, "r"], [n, "n"]},
    JuliaSinhDual, JuliaSinh, { [c_re, "c re"], [c_im, "c ε"], [r, "r"], [n, "n"]},
    JuliaZubietaDual, JuliaZubieta, { [c_re, "c re"], [c_im, "c ε"], [r, "r"], [n, "n"]},
    JuliaPicardDual, JuliaPicard, { [a, "a"], [c_re, "c re"], [c_im, "c ε"], [alpha, "α"], [n, "n"]   },
    JuliaBiomorphDual, JuliaBiomorph, { [r, "r"], [c_re, "c re"], [c_im, "c ε"], [m_re, "m re"], [m_im, "m ε"], [a_re, "a re"], [a_im, "a ε"], [b_re, "b re"], [b_im, "b ε"], [alpha, "α"], [n, "n"]   },
    MandelbrotPowerPerplex, MandelbrotPower, { [r, "r"], [n, "n"] },
    MandelbrotProbabilityPerplex, MandelbrotProbability, { [a, "a"], [r, "r"], [n, "n"] },
    MandelbrotSinusPerplex, MandelbrotSinus, { [r, "r"], [n, "n"]},
    MandelbrotSinhPerplex, MandelbrotSinh, { [r, "r"], [n, "n"]},
    MandelbrotZubietaPerplex, MandelbrotZubieta, { [r, "r"], [n, "n"]},
    MandelbrotPicardPerplex, MandelbrotPicard, { [a, "a"], [alpha, "α"], [n, "n"]   },
    MandelbrotBiomorphPerplex, MandelbrotBiomorph, { [r, "r"], [m_re, "m t"], [m_im, "m x"], [a_re, "a t"], [a_im, "a x"], [b_re, "b t"], [b_im, "b x"], [alpha, "α"], [n, "n"]   },
    JuliaPowerPerplex, JuliaPower, { [c_re, "c t"], [c_im, "c x"], [r, "r"], [n, "n"] },
    JuliaProbabilityPerplex, JuliaProbability, { [c_re, "c t"], [c_im, "c x"], [a, "a"], [r, "r"], [n, "n"] },
    JuliaSinusPerplex, JuliaSinus, { [c_re, "c t"], [c_im, "c x"], [r, "r"], [n, "n"]},
    JuliaSinhPerplex, JuliaSinh, { [c_re, "c t"], [c_im, "c x"], [r, "r"], [n, "n"]},
    JuliaZubietaPerplex, JuliaZubieta, { [c_re, "c t"], [c_im, "c x"], [r, "r"], [n, "n"]},
    JuliaPicardPerplex, JuliaPicard, { [a, "a"], [c_re, "c t"], [c_im, "c x"], [alpha, "α"], [n, "n"]   },
    JuliaBiomorphPerplex, JuliaBiomorph, { [r, "r"], [c_re, "c t"], [c_im, "c x"], [m_re, "m t"], [m_im, "m x"], [a_re, "a t"], [a_im, "a x"], [b_re, "b t"], [b_im, "b x"], [alpha, "α"], [n, "n"]   },
    MandelbrotPowerQuaternion, MandelbrotPower, { [r, "r"], [n, "n"] },
    MandelbrotProbabilityQuaternion, MandelbrotProbability, { [a, "a"], [r, "r"], [n, "n"] },
    MandelbrotSinusQuaternion, MandelbrotSinus, { [r, "r"], [n, "n"]},
    MandelbrotSinhQuaternion, MandelbrotSinh, { [r, "r"], [n, "n"]},
    MandelbrotZubietaQuaternion, MandelbrotZubieta, { [r, "r"], [n, "n"]},
    MandelbrotPicardQuaternion, MandelbrotPicard, { [a, "a"], [alpha, "α"], [n, "n"]   },
    MandelbrotBiomorphQuaternion, MandelbrotBiomorph, { [r, "r"], [m_w, "m w"], [m_i, "m i"], [m_j, "m j"], [m_k, "m k"], [a_w, "a w"], [a_i, "a i"], [a_j, "a j"], [a_k, "a k"], [b_w, "b w"], [b_i, "b i"], [b_j, "b j"], [b_k, "b k"], [alpha, "α"], [n, "n"]   },
    JuliaPowerQuaternion, JuliaPower, { [c_w, "c w"], [c_i, "c i"], [c_j, "c j"], [c_k, "c k"], [r, "r"], [n, "n"] },
    JuliaProbabilityQuaternion, JuliaProbability, { [c_w, "c w"], [c_i, "c i"], [c_j, "c j"], [c_k, "c k"], [a, "a"], [r, "r"], [n, "n"] },
    JuliaSinusQuaternion, JuliaSinus, { [c_w, "c w"], [c_i, "c i"], [c_j, "c j"], [c_k, "c k"], [r, "r"], [n, "n"]},
    JuliaSinhQuaternion, JuliaSinh, { [c_w, "c w"], [c_i, "c i"], [c_j, "c j"], [c_k, "c k"], [r, "r"], [n, "n"]},
    JuliaZubietaQuaternion, JuliaZubieta, { [c_w, "c w"], [c_i, "c i"], [c_j, "c j"], [c_k, "c k"], [r, "r"], [n, "n"]},
    JuliaPicardQuaternion, JuliaPicard, { [a, "a"], [c_w, "c w"], [c_i, "c i"], [c_j, "c j"], [c_k, "c k"], [alpha, "α"], [n, "n"]   },
    JuliaBiomorphQuaternion, JuliaBiomorph, { [r, "r"], [c_w, "c w"], [c_i, "c i"], [c_j, "c j"], [c_k, "c k"], [m_w, "m w"], [m_i, "m i"], [m_j, "m j"], [m_k, "m k"], [a_w, "a w"], [a_i, "a i"], [a_j, "a j"], [a_k, "a k"], [b_w, "b w"], [b_i, "b i"], [b_j, "b j"], [b_k, "b k"], [alpha, "α"], [n, "n"]   }
}

macro_rules! impl_continuous_variants {
    ($($variant:ident, $solver:ident, { $([$field:ident, $field_label:expr]),* }),*) => {
        $(
            paste!{
                generate_view_variant!{
                    $variant { $([$field, $field_label]),* }
                }
                impl From<$variant> for SelectedChaoticFunction{
                    fn from(val: $variant)->Self{
                        $(
                            if let Some((par_min, par_max)) = val.[<range_ $field>]{
                                let par_values = parameter_linspace(par_min, par_max, val.num_params);
                                let ode_solvers = par_values.iter().map(|par|{
                                    let mut pars = val.data.clone();
                                    pars.$field = *par;
                                    $solver::new(pars)
                                }).collect();
                                let ode_solver_vec = OdeSystemSolverVec::$variant(ode_solvers);
                                return SelectedChaoticFunction::ParametrizedDifferentialSystems(ode_solver_vec, stringify!($field), par_values);
                            }
                        )*
                        SelectedChaoticFunction::SingleDifferentialSystem(OdeSystemSolverVec::$variant(vec![$solver::new(val.data.clone())]))
                    }
                }
            }
        )*
    };
}

impl_continuous_variants! {
    Brusselator, OdeSolver, { [a, "a"] , [b, "b"]  },
    VanDerPol, OdeSolver, {  [mu, "μ"] },
    QuadrupTwoOrbit, OdeSolver, { [a, "a"]  , [b, "b"]  , [c, "c"]   },
    Lorenz, OdeSolver, { [sigma, "σ"]  , [beta, "β"]  , [rho, "ρ"]   },
    Rossler, OdeSolver, { [a, "a"]  , [b, "b"]  , [c, "c"]   },
    Chen, OdeSolver, { [a, "a"]  , [b, "b"]  , [c, "c"]   },
    Aizawa, OdeSolver, { [a, "a"]  , [b, "b"]  , [c, "c"]  , [d, "d"]  , [e, "e"]  , [f, "f"]   },
    ChuasCircuit, OdeSolver, { [alpha, "α"]  , [beta, "β"]   },
    RabinovichFabrikant, OdeSolver, { [alpha, "α"]  , [gamma, "γ"]   },
    GenesioTesi, OdeSolver, { [a, "a"] , [b, "b"] , [c, "c"]  },
    BurkeShaw, OdeSolver, { [s, "s"]  , [v, "v"]  },
    Halvorsen, OdeSolver, { [a, "a"]   },
    ThreeSpeciesLotkaVolterra, OdeSolver, { [b, "b"] , [d1, "d1"] , [d2, "d2"] ,  [a11, "a11"] , [a12, "a12"] , [a13, "a13"] , [a21, "a21"] , [a23, "a23"] , [a31, "a31"] , [a32, "a32"]  },
    Rikitake, OdeSolver, { [a, "a"]  ,  [mu, "μ"]   },
    HindmarshRose, OdeSolver, { [a, "a"] , [b, "b"] , [c, "c"] , [d, "d"] , [r, "r"] , [i, "i"] },
    Ababneh, OdeSolver, { [a, "a"]  , [b, "b"]   },
    WeiWang, OdeSolver, { [a, "a"]  , [b, "b"]  , [c, "c"]  , [d, "d"]  , [k, "k"]   },
    ParticleXY, ParticleXYSystemSolver, { [s, "s"] , [m, "m"] , [l, "l"] },
    ParticleXYZ, ParticleXYZSystemSolver, { [s, "s"] , [m, "m"] , [l, "l"] }
}
