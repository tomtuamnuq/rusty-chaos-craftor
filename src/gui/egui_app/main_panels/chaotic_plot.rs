use crate::chaos::data::ChaosDataVec;
use crate::gui::plot::*;
use crate::gui::tooltips::*;
use crate::gui::*;
use crate::utils::Timer;
use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct PlotPanel {
    #[serde(skip)] // start without generating data immediately
    pub generate_new_data: bool,
    reinit_data: bool,
    plot_2_d: Plot2D,
    plot_3_d: Plot3D,
    plot_backend: PlotBackendVariant,
    save_trajectory: bool,
    max_num_series: usize,
    last_max_num_series: usize,
    point_colormap: SeriesColors,
    frame_rate: usize,
    timer: Timer,
}

const DEFAULT_NUM_SERIES: usize = 20;
impl Default for PlotPanel {
    fn default() -> Self {
        Self {
            generate_new_data: false,
            reinit_data: false,
            plot_2_d: Default::default(),
            plot_3_d: Default::default(),
            plot_backend: Default::default(),
            save_trajectory: true,
            max_num_series: DEFAULT_NUM_SERIES,
            last_max_num_series: DEFAULT_NUM_SERIES,
            point_colormap: Default::default(),
            frame_rate: 10,
            timer: Default::default(),
        }
    }
}
impl PlotPanel {
    pub fn reinit_data(&self) -> bool {
        self.reinit_data
    }
    pub fn add_point_series(&mut self, data: ChaosDataVec<'_>) {
        match self.plot_backend {
            PlotBackendVariant::EguiPlot2D => {
                self.plot_2_d.set_point_colormap(self.point_colormap);
                self.plot_2_d.set_max_num_series(self.max_num_series);
                self.plot_2_d.add_point_series(data);
            }
            PlotBackendVariant::Plotters => {
                self.plot_3_d.set_point_colormap(self.point_colormap);
                self.plot_3_d.set_max_num_series(self.max_num_series);
                self.plot_3_d.add_point_series(data);
            }
        }
    }

    pub fn reset_plot_trajectory(&mut self) {
        self.plot_2_d.reset_data();
        self.plot_3_d.reset_data();
    }

    pub fn set_no_parametrized_plotting(&mut self) {
        self.plot_2_d.remove_parameter();
        self.plot_3_d.remove_parameter();
    }

    pub fn set_parametrized_plotting(&mut self, par: &'static str, par_values: Vec<f64>) {
        self.plot_2_d.set_parameter(par, par_values.to_owned());
        self.plot_3_d.set_parameter(par, par_values);
    }

    pub fn check_frame_rate(&mut self) -> bool {
        self.timer.check_elapsed()
    }

    fn add_general_plot_options(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if combo_box(
                LABEL_PLOT_BACKEND,
                &mut self.plot_backend,
                ui,
                TIP_PLOT_BACKEND,
            ) {
                self.reset_plot_trajectory();
            };
        });
        ui.horizontal(|ui| {
            if clickable_button(
                LABEL_INIT_DATA,
                self.generate_new_data,
                true,
                ui,
                TIP_INIT_DATA,
            ) {
                self.generate_new_data = true;
            }
            add_checkbox(
                LABEL_REINIT_DATA,
                &mut self.reinit_data,
                ui,
                TIP_REINIT_DATA,
            );
        });
        ui.horizontal(|ui| {
            if integer_slider(
                LABEL_NUM_FRAMES,
                &mut self.frame_rate,
                50,
                ui,
                TIP_NUM_FRAMES,
            ) {
                self.timer.set_frequency(self.frame_rate as f64);
            };
        });
        ui.horizontal(|ui| {
            if add_checkbox(
                LABEL_TRAJECTORY,
                &mut self.save_trajectory,
                ui,
                TIP_TRAJECTORY,
            ) {
                if !self.save_trajectory {
                    self.last_max_num_series = self.max_num_series;
                    self.max_num_series = 1;
                } else {
                    self.max_num_series = self.last_max_num_series;
                }
            }
            if self.save_trajectory {
                integer_slider(
                    LABEL_NUM_SERIES,
                    &mut self.max_num_series,
                    100,
                    ui,
                    TIP_NUM_SERIES,
                );
            }
        });
        ui.horizontal(|ui| {
            combo_box(LABEL_COLORMAP, &mut self.point_colormap, ui, TIP_COLORMAP);
        });
        ui.horizontal(|ui| {
            let color_choice = match self.plot_backend {
                PlotBackendVariant::EguiPlot2D => self.plot_2_d.series_color_mut(),
                PlotBackendVariant::Plotters => self.plot_3_d.series_color_mut(),
            };
            combo_box(LABEL_COLOR_PER_POINT, color_choice, ui, TIP_COLOR_PER_POINT);
        });
    }

    pub fn conf_ui(&mut self, ui: &mut Ui) {
        group_vertical(ui, |ui| {
            ui.heading("Plot Configuration");
            self.add_general_plot_options(ui);
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.collapsing("Plot Visualization Info", |ui| {
                group_vertical(ui, |ui| {
                    self.add_plot_explanation(ui);
                });
            });
            group_vertical(ui, |ui| {
                ui.heading("State Projections");
                self.add_plot_backend_options(ui);
            });
        });
    }

    fn add_plot_backend_options(&mut self, ui: &mut Ui) {
        match self.plot_backend {
            PlotBackendVariant::EguiPlot2D => self.plot_2_d.options_ui(ui),
            PlotBackendVariant::Plotters => self.plot_3_d.options_ui(ui),
        };
    }
    fn add_plot_explanation(&mut self, ui: &mut Ui) {
        match self.plot_backend {
            PlotBackendVariant::EguiPlot2D => self.plot_2_d.explanation(ui),
            PlotBackendVariant::Plotters => self.plot_3_d.explanation(ui),
        };
    }

    pub fn ui(&mut self, mouse_is_over_plot: bool, ui: &mut Ui) {
        ui.ctx().request_repaint(); // animate
        match self.plot_backend {
            PlotBackendVariant::EguiPlot2D => self.plot_2_d.ui(ui),
            PlotBackendVariant::Plotters => self.plot_3_d.ui(mouse_is_over_plot, ui),
        };
    }
}
