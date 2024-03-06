use crate::{
    chaos::data::{DistributionDimensions, FractalDimensions},
    chaos::{DiscreteMapVec, OdeSystemSolverVec},
    gui::{add_hyperlink, integer_slider, tooltips::*},
};

use super::execute_chaotic_function_view::{
    ChaosFunctionViewData, DifferentialSystemView, DiscreteMapView,
};

use egui::{ScrollArea, Ui};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(PartialEq, Deserialize, Serialize)]
pub struct ExecutionPanel {
    num_executions: usize,
    chaotic_discrete_map: Option<DiscreteMapView>,
    chaotic_diff_system: Option<DifferentialSystemView>,
    #[cfg_attr(target_arch = "wasm32", serde(skip))] // TODO causes wasm memory bug, works native
    view_data: ChaosFunctionViewData,
    #[serde(skip)] // start without an initialized function
    pub selected_function_was_set: bool,
}

impl Default for ExecutionPanel {
    fn default() -> Self {
        Self {
            num_executions: 1,
            chaotic_discrete_map: None,
            chaotic_diff_system: None,
            view_data: Default::default(),
            selected_function_was_set: false,
        }
    }
}
pub enum SelectedChaoticFunction {
    Nothing,
    SingleDiscreteMap(DiscreteMapVec),
    ParametrizedDiscreteMaps(DiscreteMapVec, &'static str, Vec<f64>),
    SingleDifferentialSystem(OdeSystemSolverVec),
    ParametrizedDifferentialSystems(OdeSystemSolverVec, &'static str, Vec<f64>),
}
impl ExecutionPanel {
    pub fn chaotic_function_is_chosen(&self) -> bool {
        self.chaotic_discrete_map.is_some() || self.chaotic_diff_system.is_some()
    }

    pub fn check_compatible_chaotic_function(&mut self, dims: &DistributionDimensions) -> bool {
        self.check_compatible_discrete_map(dims) | self.check_compatible_diff_system(dims)
    }

    fn check_compatible_discrete_map(&mut self, dims: &DistributionDimensions) -> bool {
        if let Some(map) = &self.chaotic_discrete_map {
            let is_compatible = map.dimensionality() == *dims;
            if is_compatible {
                return true;
            } else {
                self.chaotic_discrete_map = None;
            }
        }
        false // None or not compatible
    }

    fn check_compatible_diff_system(&mut self, dims: &DistributionDimensions) -> bool {
        if let Some(system) = &self.chaotic_diff_system {
            let is_compatible = system.dimensionality() == *dims;
            if is_compatible {
                return true;
            } else {
                self.chaotic_diff_system = None;
            }
        }
        false // None or not compatible
    }

    pub fn num_executions(&self) -> usize {
        self.num_executions
    }

    pub fn chosen_chaotic_function(&mut self) -> SelectedChaoticFunction {
        self.selected_function_was_set = true;
        if let Some(view) = self.chaotic_discrete_map.as_ref() {
            return self.view_data.map_discrete_view_to_maps_vec_variant(view);
        };
        if let Some(view) = self.chaotic_diff_system.as_ref() {
            return self
                .view_data
                .map_continuous_view_to_solver_vec_variant(view);
        }
        SelectedChaoticFunction::Nothing
    }

    fn discrete_map_listing(&mut self, ui: &mut Ui, dims: &DistributionDimensions) {
        ui.vertical(|ui| {
            ui.heading("Discrete Map");
            ui.group(|ui| {
                DiscreteMapView::iter().for_each(|view| {
                    if *dims == view.dimensionality() {
                        self.discrete_map_selection(ui, view);
                    }
                });
            });
        });
    }
    fn discrete_map_selection(&mut self, ui: &mut Ui, view: DiscreteMapView) {
        let view_name: &'static str = view.into();
        let on_hover_description = |ui: &mut Ui| {
            let (description, ref_link) = self.view_data.discrete_description(&view);
            ui.vertical(|ui| {
                ui.label(description);
                ui.horizontal(|ui| {
                    ui.label("Reference: ");
                    ui.hyperlink(ref_link);
                });
            });
        };
        if ui
            .selectable_value(&mut self.chaotic_discrete_map, Some(view), view_name)
            .on_hover_ui(on_hover_description)
            .changed()
        {
            self.chaotic_diff_system = None;
            self.selected_function_was_set = false;
        }
    }
    fn fractal_ui(&mut self, ui: &mut Ui, dims: &DistributionDimensions) {
        let all_fractals_with_dims: Vec<DiscreteMapView> = DiscreteMapView::iter()
            .filter(|view| *dims == view.dimensionality())
            .collect();
        ui.vertical(|ui| {
            ui.heading("Mandelbrot");
            ui.group(|ui| {
                all_fractals_with_dims.iter().for_each(|view| {
                    if view.is_mandelbrot() {
                        self.discrete_map_selection(ui, *view);
                    }
                });
            });
        });
        ui.separator();
        ui.vertical(|ui| {
            ui.heading("Julia");
            ui.group(|ui| {
                all_fractals_with_dims.iter().for_each(|view| {
                    if view.is_julia() {
                        self.discrete_map_selection(ui, *view);
                    }
                });
            });
        });
    }
    fn diff_system_ui(&mut self, ui: &mut Ui, dims: &DistributionDimensions) {
        ui.vertical(|ui| {
            ui.heading("Differential System");
            ui.group(|ui| {
                DifferentialSystemView::iter().for_each(|view| {
                    let view_name: &'static str = view.into();
                    let on_hover_description = |ui: &mut Ui| {
                        let (description, ref_link) = self.view_data.continuous_description(&view);
                        ui.vertical(|ui| {
                            ui.label(description);
                            ui.horizontal(|ui| {
                                ui.label("Reference: ");
                                ui.hyperlink(ref_link);
                            });
                        });
                    };
                    if *dims == view.dimensionality()
                        && ui
                            .selectable_value(&mut self.chaotic_diff_system, Some(view), view_name)
                            .on_hover_ui(on_hover_description)
                            .changed()
                    {
                        self.chaotic_discrete_map = None;
                        self.selected_function_was_set = false;
                    }
                })
            });
        });
    }

    fn particle_ui(&mut self, _ui: &mut Ui, num_dims: usize) {
        // since there is only one version implemented for 2 and 3 each we select it
        if !self.chaotic_function_is_chosen() {
            self.chaotic_diff_system = Some(if num_dims == 3 {
                DifferentialSystemView::ParticleXYZ
            } else {
                DifferentialSystemView::ParticleXY
            });
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, dims: DistributionDimensions, num_execution_limit: usize) {
        self.check_compatible_chaotic_function(&dims);
        ui.vertical(|ui| {
            ScrollArea::vertical().show(ui, |ui| {
                match dims {
                    DistributionDimensions::State(_) => {
                        ui.horizontal(|ui| {
                            self.discrete_map_listing(ui, &dims);
                            ui.separator();
                            self.diff_system_ui(ui, &dims);
                        });
                    }
                    DistributionDimensions::Particle(n) => {
                        ui.heading(format!("{n}D Particles"));
                        ui.separator();
                        self.particle_ui(ui, n);
                    }
                    DistributionDimensions::Fractal(ref fractal_ring) => {
                        let ring_type: &'static str = fractal_ring.into();
                        ui.horizontal(|ui| {
                            ui.heading(format!("{ring_type} Fractals "));
                            let (reference, tooltip) = match fractal_ring {
                                FractalDimensions::Complex => (LINK_COMPLEX, TIP_COMPLEX),
                                FractalDimensions::Dual => (LINK_DUAL, TIP_DUAL),
                                FractalDimensions::Perplex => (LINK_PERPLEX, TIP_PERPLEX),
                                FractalDimensions::Quaternion => (LINK_QUATERNION, TIP_QUATERNION),
                            };
                            add_hyperlink("Info", reference, ui, tooltip);
                        });
                        ui.horizontal(|ui| {
                            self.fractal_ui(ui, &dims);
                        });
                    }
                };
            });
        });
        ScrollArea::both().show(ui, |ui| {
            if let Some(open) = &self.chaotic_discrete_map {
                self.view_data.discrete_view_ui(open, ui);
            } else if let Some(open) = &self.chaotic_diff_system {
                self.view_data.continuous_view_ui(open, ui);
            };
            ui.horizontal(|ui| {
                integer_slider(
                    LABEL_NUM_EXECS,
                    &mut self.num_executions,
                    num_execution_limit,
                    ui,
                    TIP_NUM_EXECS,
                );
            });
        });
    }
}
