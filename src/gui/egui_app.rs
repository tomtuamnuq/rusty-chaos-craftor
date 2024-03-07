mod conf_panels;
mod egui_utils;
mod main_panels;
use self::conf_panels::*;
pub use self::egui_utils::*;
use self::main_panels::*;
use crate::chaos::{benchmark::ChaosInitSchema, *};
use crate::gui::tooltips::*;
use anyhow::{bail, Error};
use egui::{
    style::{Interaction, Visuals},
    Align2, Context, CursorIcon, FontFamily, FontId, TextStyle, Ui,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ChaosApp {
    #[serde(skip)] // open initial panel to save latest chosen function (which is not set)
    open_conf_panel: ConfPanel,
    initial_panel: InitialPanel,
    execute_panel: ExecutionPanel,
    plot_panel: PlotPanel,
    benchmark_panel: BenchmarkPanel,
    open_main_panel: MainPanel,
    #[serde(skip)] // avoid saving ChaosData arrays
    chaos_controller: ChaosExecutionController,
    #[serde(skip)] // start without initiating function
    init_chaotic_function: bool,
    #[serde(skip)] // always start without executing
    executes: bool,
}
impl PartialEq for ChaosApp {
    fn eq(&self, other: &Self) -> bool {
        // only compare options for reset
        self.open_conf_panel == other.open_conf_panel
        && self.open_main_panel == other.open_main_panel
        && self.executes == other.executes // most of the times this will end the comparison to default
        && self.plot_panel == other.plot_panel
        && self.initial_panel == other.initial_panel
        && self.execute_panel == other.execute_panel
        && self.benchmark_panel == other.benchmark_panel
    }
}

impl ChaosApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        ctx.style_mut(|style| {
            style.text_styles = [
                (
                    TextStyle::Heading,
                    FontId::new(18.0, FontFamily::Proportional),
                ),
                (TextStyle::Body, FontId::new(12.0, FontFamily::Proportional)),
                (
                    TextStyle::Monospace,
                    FontId::new(12.0, FontFamily::Monospace),
                ),
                (TextStyle::Button, FontId::new(14.0, FontFamily::Monospace)),
                (TextStyle::Small, FontId::new(8.0, FontFamily::Monospace)),
            ]
            .into();
            style.interaction = Interaction {
                resize_grab_radius_side: 6.0,
                resize_grab_radius_corner: 12.0,
                show_tooltips_only_when_still: true,
                tooltip_delay: 1.0,
                // selectable_labels: false,
                // multi_widget_text_select: false
            };
            style.visuals = Visuals::dark();
            style.visuals.interact_cursor = Some(CursorIcon::PointingHand);
        }); // Disable feathering as it causes artifacts with Plotters backend
        ctx.tessellation_options_mut(|tess_options| {
            tess_options.feathering = false;
        });
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }

    fn add_point_series(&mut self) {
        if let Ok(data) = self.chaos_controller.get_chaos_data() {
            self.plot_panel.add_point_series(data)
        }
    }

    fn chaos_data_loop(&mut self) -> Result<(), Error> {
        if self.plot_panel.generate_new_data {
            self.plot_panel.generate_new_data = false;
            self.generate_initial_chaos_data()?;
        } else if self.init_chaotic_function {
            self.init_chaotic_function = false;
            self.initialize_chaotic_functions()?;
        } else if self.executes && self.plot_panel.check_frame_rate() {
            self.execute_chaotic_function()?;
        };
        Ok(())
    }

    fn benchmark_loop(&mut self) -> Result<(), Error> {
        if self.benchmark_panel.benchmark_toggle() {
            let mut chaos_init = ChaosInitSchema {
                num_samples: self.initial_panel.number_of_samples(),
                num_executions: self.execute_panel.num_executions(),
                init_distr: self.initial_panel.initial_distributions(),
                ..Default::default()
            };
            match self.execute_panel.chosen_chaotic_function() {
                SelectedChaoticFunction::SingleDiscreteMap(map_vec) => {
                    chaos_init.discrete_map_vec = Some(map_vec);
                }
                SelectedChaoticFunction::SingleDifferentialSystem(diff_system_vec) => {
                    chaos_init.diff_system_vec = Some(diff_system_vec);
                }
                SelectedChaoticFunction::ParametrizedDiscreteMaps(map_vec, par, par_values) => {
                    chaos_init.discrete_map_vec = Some(map_vec);
                    chaos_init.pars = (par, par_values);
                }
                SelectedChaoticFunction::ParametrizedDifferentialSystems(
                    diff_system_vec,
                    par,
                    par_values,
                ) => {
                    chaos_init.diff_system_vec = Some(diff_system_vec);
                    chaos_init.pars = (par, par_values);
                }
                SelectedChaoticFunction::Nothing => {
                    bail!("Cannot init chaotic function as it is not set in the execute panel!")
                }
            };

            self.benchmark_panel.chaos_benchmark(chaos_init);
        };
        Ok(())
    }

    fn generate_initial_chaos_data(&mut self) -> Result<(), Error> {
        let init_distr = self.initial_panel.initial_distributions();
        self.executes = self
            .execute_panel
            .check_compatible_chaotic_function(&init_distr.dimensionality())
            && self.executes;
        let chaos_data_gen_result = self
            .chaos_controller
            .generate_initial_chaos_data(self.initial_panel.number_of_samples(), init_distr);
        self.plot_panel.reset_plot_trajectory();
        self.add_point_series();
        chaos_data_gen_result
    }
    fn initialize_chaotic_functions(&mut self) -> Result<(), Error> {
        match self.execute_panel.chosen_chaotic_function() {
            SelectedChaoticFunction::SingleDiscreteMap(map_vec) => {
                self.chaos_controller.set_discrete_mappers(map_vec)?;
                self.plot_panel.set_no_parametrized_plotting();
            }
            SelectedChaoticFunction::SingleDifferentialSystem(diff_system_vec) => {
                self.chaos_controller
                    .set_differential_solvers(diff_system_vec)?;
                self.plot_panel.set_no_parametrized_plotting();
            }
            SelectedChaoticFunction::ParametrizedDiscreteMaps(map_vec, par, par_values) => {
                self.chaos_controller.set_discrete_mappers(map_vec)?;
                self.plot_panel.set_parametrized_plotting(par, par_values);
            }
            SelectedChaoticFunction::ParametrizedDifferentialSystems(
                diff_system_vec,
                par,
                par_values,
            ) => {
                self.chaos_controller
                    .set_differential_solvers(diff_system_vec)?;
                self.plot_panel.set_parametrized_plotting(par, par_values);
            }
            SelectedChaoticFunction::Nothing => {
                bail!("Cannot init chaotic function as it is not set in the execute panel!")
            }
        };
        Ok(())
    }
    fn execute_chaotic_function(&mut self) -> Result<(), Error> {
        self.chaos_controller
            .execute(self.execute_panel.num_executions())?;
        if self.plot_panel.reinit_data() {
            self.chaos_controller.reinit_states()?;
        }
        self.add_point_series();
        Ok(())
    }

    fn add_general_conf(&mut self, ui: &mut Ui) {
        group_horizontal(ui, |ui| {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if clickable_button("Quit", false, true, ui, "Close the application.") {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
            egui::widgets::global_dark_light_mode_buttons(ui);
            if clickable_button(
                LABEL_BUTTON_RESET,
                false,
                true,
                ui,
                TIP_BUTTON_RESET,
            ) {
                // always enabled avoids comparing all initial distribution and chaotic function parameters
                *self = Default::default();
            }
        });
        group_horizontal(ui, |ui| {
            combo_box(
                LABEL_MAIN_MODE,
                &mut self.open_main_panel,
                ui,
                TIP_MAIN_MODE,
            );
            add_checkbox(LABEL_RUN, &mut self.executes, ui, TIP_RUN);
        });
        ui.vertical(|ui| {
            match self.open_main_panel {
                MainPanel::ChaoticPlot => {
                    self.plot_panel.conf_ui(ui);
                }
                MainPanel::Benchmark => {
                    self.benchmark_panel
                        .conf_ui(self.execute_panel.chaotic_function_is_chosen(), ui);
                }
            };
        });
    }

    fn add_chaos_conf(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            select_group(&mut self.open_conf_panel, ui, TIP_INIT_PANEL);
        });
        if self.open_main_panel == MainPanel::ChaoticPlot {
            group_horizontal(ui, |ui| {
                let init_data_button_selected = self.chaos_controller.dimensionality()
                    == self.initial_panel.dimensionality()
                    || self.plot_panel.generate_new_data;
                if clickable_button(
                    LABEL_INIT_DATA,
                    init_data_button_selected,
                    true,
                    ui,
                    TIP_INIT_DATA,
                ) {
                    self.plot_panel.generate_new_data = true;
                }
                let init_fct_button_selected =
                    self.execute_panel.selected_function_was_set || self.init_chaotic_function;
                let init_fct_button_enabled = self.execute_panel.chaotic_function_is_chosen();
                if clickable_button(
                    LABEL_INIT_FUNCTION,
                    init_fct_button_selected,
                    init_fct_button_enabled,
                    ui,
                    TIP_INIT_FUNCTION,
                ) {
                    self.init_chaotic_function = true;
                    self.executes = true;
                };
            });
        }
        match self.open_conf_panel {
            ConfPanel::Initial => {
                self.initial_panel.ui(ui);
            }
            ConfPanel::Execution => {
                let (dims, num_exec_limit) = match self.open_main_panel {
                    MainPanel::ChaoticPlot => (self.chaos_controller.dimensionality(), 100),
                    MainPanel::Benchmark => (self.initial_panel.dimensionality(), 10_000),
                };
                self.execute_panel.ui(ui, dims, num_exec_limit);
            }
        };
    }

    fn main_loop_and_ui(&mut self, mouse_over_main_panel: bool, ui: &mut Ui) {
        match self.open_main_panel {
            MainPanel::ChaoticPlot => {
                let _ = self.chaos_data_loop();
                self.plot_panel.ui(mouse_over_main_panel, ui);
            }
            MainPanel::Benchmark => {
                let _ = self.benchmark_loop();
                self.benchmark_panel.ui(ui);
            }
        }
    }
}

impl eframe::App for ChaosApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut mouse_over_main_panel = true;
        let conf_align = match self.open_main_panel {
            MainPanel::ChaoticPlot => Align2::LEFT_TOP,
            MainPanel::Benchmark => Align2::CENTER_TOP,
        };
        conf_window("Configuration", ctx, conf_align).show(ctx, |ui| {
            let response = ui
                .vertical(|ui| {
                    self.add_general_conf(ui);
                })
                .response;
            if response.hovered() {
                mouse_over_main_panel = false;
            }
        });
        conf_window("Chaos Creation", ctx, Align2::RIGHT_TOP).show(ctx, |ui| {
            let response = ui
                .vertical(|ui| {
                    self.add_chaos_conf(ui);
                })
                .response;
            if response.hovered() {
                mouse_over_main_panel = false;
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_loop_and_ui(mouse_over_main_panel, ui);
        });
    }
}
