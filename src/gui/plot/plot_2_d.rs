use crate::chaos::data::*;
use crate::gui::{float_slider, group_horizontal, tooltips::*, PARAMETER_MAX, PARAMETER_MIN};
use delegate::delegate;

use egui::Ui;
use egui::{Color32, Shape, Stroke};
use egui_plot::{format_number, log_grid_spacer, Plot, PlotPoint, PlotPoints, PlotUi, Points};
use serde::{Deserialize, Serialize};

use super::plot_colors::{FromRGB, SeriesColorChoice, SeriesColors, RGB};
use super::plot_data::PlotData;
use super::plot_styles::DEFAULT_RADIUS;
use super::plot_utils::{StateProjection, StateProjectionSelection, MAX_NUM_PROJECTIONS};

pub type Point2D = PlotPoint;
pub type Points2D = Vec<Option<Point2D>>;
impl FromRGB for Color32 {
    fn from_rgb(rgb: RGB) -> Self {
        Color32::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

#[derive(PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Plot2D {
    #[serde(skip)] // avoid saving points
    plot_data: PlotData<Point2D, Color32>,
    #[serde(skip)] // projections are set when first series is added
    projection_x: StateProjection,
    #[serde(skip)]
    selection_x: StateProjectionSelection,
    #[serde(skip)]
    projection_y: StateProjection,
    #[serde(skip)]
    selection_y: StateProjectionSelection,
    #[serde(skip)]
    selection_color: StateProjectionSelection,
    #[serde(skip)] // start without data
    mean_number_of_shapes_guess: usize,
    point_size: f64,
}

impl Default for Plot2D {
    fn default() -> Self {
        Self {
            plot_data: Default::default(),
            // chaos app starts without data - projections are set when data is added
            projection_x: Default::default(),
            selection_x: Default::default(),
            projection_y: Default::default(),
            selection_y: Default::default(),
            selection_color: Default::default(),
            mean_number_of_shapes_guess: 100,
            point_size: DEFAULT_RADIUS,
        }
    }
}

impl Plot2D {
    fn parameters_are_shown(&self) -> bool {
        self.selection_x == StateProjectionSelection::Par
    }
    pub fn set_parameter(&mut self, parameter: &'static str, par_values: Vec<f64>) {
        let had_parameter = self.plot_data.with_parameter();
        self.plot_data.set_parameter(parameter, par_values);
        if !had_parameter {
            self.reset_projections();
        } else if self.parameters_are_shown() {
            self.projection_x = StateProjection::Par(parameter)
        }
    }
    pub fn remove_parameter(&mut self) {
        if self.plot_data.with_parameter() {
            self.plot_data.remove_parameter();
            self.reset_projections();
        }
    }

    fn reset_projections(&mut self) {
        let dim = self.number_of_dimensions();
        let projection_color = if let Some(p) = self.plot_data.get_parameter() {
            self.projection_x = StateProjection::Par(p);
            self.projection_y = StateProjection::S(0);
            if dim > 1 {
                StateProjection::S(1)
            } else {
                StateProjection::S(0)
            }
        } else {
            self.projection_x = StateProjection::S(0);
            self.projection_y = StateProjection::S(1);
            if dim > 2 {
                StateProjection::S(2)
            } else {
                StateProjection::S(0)
            }
        };
        self.plot_data.set_projection_color(projection_color);
        self.selection_color = StateProjectionSelection::from(projection_color);
        self.selection_x = StateProjectionSelection::from(self.projection_x);
        self.selection_y = StateProjectionSelection::from(self.projection_y);
    }

    pub fn add_point_series(&mut self, data: ChaosDataVec<'_>) {
        let dimensionality = data.dimensionality();
        if dimensionality != *self.plot_data.dimensionality() {
            self.plot_data.remove_parameter();
            self.plot_data.set_dimensionality(dimensionality);
            self.reset_projections();
        }
        let styles = self.plot_data.create_styles_for_chaos_data(&data);
        let series = if self.parameters_are_shown() {
            self.create_point_series_with_parameters(data)
        } else {
            self.create_point_series_without_parameters(data)
        };
        let extrema = Self::get_extrema_from_series(&series);
        self.plot_data.add_series(series, styles, extrema);
    }

    pub fn transform_points_1_d(&self, states: &[Option<State1>]) -> Points2D {
        match self.plot_data.latest_series() {
            None => states
                .iter()
                .map(|v| v.map(|v| PlotPoint::new(0.0, v[0])))
                .collect(),
            Some((last_states, _)) => states
                .iter()
                .zip(last_states.iter())
                .map(|(new_state, last_state)| {
                    new_state.map(|new_state| {
                        let new_y = new_state[0];
                        if let Some(last_state) = last_state {
                            PlotPoint::new(last_state.y, new_y)
                        } else {
                            // new_state was reinitialized
                            PlotPoint::new(0.0, new_y)
                        }
                    })
                })
                .collect(),
        }
    }
    pub fn transform_points_n_d<V: StateIndex>(&self, states: &[Option<V>]) -> Points2D {
        let (ind_x, ind_y) = (self.projection_x.index(), self.projection_y.index());
        states
            .iter()
            .map(|v| {
                v.as_ref()
                    .map(|v| PlotPoint::new(v.ind(ind_x), v.ind(ind_y)))
            })
            .collect()
    }

    pub fn points_with_parameter_n_d<V: StateIndex>(
        &self,
        states: &[Option<V>],
        par: &f64,
    ) -> Points2D {
        let ind_y = self.projection_y.index();
        states
            .iter()
            .map(|v| v.as_ref().map(|v| PlotPoint::new(*par, v.ind(ind_y))))
            .collect()
    }

    const SQRT_3: f32 = 1.732_050_8; // 3_f32.sqrt() = 1.73205080757;
    const FRAC_1_SQRT_2: f32 = std::f32::consts::FRAC_1_SQRT_2; // 1.0 / 2_f32.sqrt();

    fn get_shapes_for_all_states(&self, plot_ui: &mut PlotUi) -> Vec<Shape> {
        let n = self.plot_data.num_series() as f32;
        let point_size = (self.point_size as f32) / n;
        self.plot_data
            .styled_series_iter()
            .enumerate()
            .flat_map(|(i, (points, styles))| {
                let i = (i + 1) as f32;
                let radius = i * point_size;
                styles
                    .iter()
                    .zip(points.iter().filter_map(|p| p.as_ref()))
                    .map(|(s, p)| {
                        let center = plot_ui.screen_from_plot(*p); // in screen coords
                        Shape::circle_filled(center, radius, s.color)
                    })
                    .collect::<Vec<Shape>>()
            })
            .collect()
    }
    fn get_shapes_for_all_particles(
        &mut self,
        default_fill: Color32,
        plot_ui: &mut PlotUi,
    ) -> Vec<Shape> {
        let num_series = self.plot_data.num_series();
        let mut shapes = Vec::with_capacity(num_series * self.mean_number_of_shapes_guess);
        let screen_translate = plot_ui.transform().dpos_dvalue_x() as f32;
        self.plot_data
            .all_styles_and_points_iter()
            .for_each(|(s, p)| {
                let center = plot_ui.screen_from_plot(*p); // in screen coords
                let stroke = Stroke::new(1.0, s.color);
                let radius = (s.radius as f32) * screen_translate; // screen sized
                shapes.push(Shape::circle_stroke(center, radius, stroke));
                // copied from:
                // https://github.com/emilk/egui/blob/a815923717365b2e49b18d238f5dc2b72d023ee0/crates/egui_plot/src/items/mod.rs#L903
                let tf = |dx: f32, dy: f32| -> egui::Pos2 { center + radius * egui::vec2(dx, dy) };
                if s.markers.positive {
                    let points = vec![
                        tf(0.0, -1.0),
                        tf(0.5 * Self::SQRT_3, 0.5),
                        tf(-0.5 * Self::SQRT_3, 0.5),
                    ];
                    shapes.push(Shape::convex_polygon(points, default_fill, stroke));
                }
                if s.markers.negative {
                    let points = vec![
                        tf(0.0, 1.0),
                        tf(-0.5 * Self::SQRT_3, -0.5),
                        tf(0.5 * Self::SQRT_3, -0.5),
                    ];
                    shapes.push(Shape::convex_polygon(points, default_fill, stroke));
                }
                if s.markers.special {
                    let diagonal1 = [
                        tf(-Self::FRAC_1_SQRT_2, -Self::FRAC_1_SQRT_2),
                        tf(Self::FRAC_1_SQRT_2, Self::FRAC_1_SQRT_2),
                    ];
                    let diagonal2 = [
                        tf(Self::FRAC_1_SQRT_2, -Self::FRAC_1_SQRT_2),
                        tf(-Self::FRAC_1_SQRT_2, Self::FRAC_1_SQRT_2),
                    ];
                    shapes.push(Shape::line_segment(diagonal1, stroke));
                    shapes.push(Shape::line_segment(diagonal2, stroke));
                }
            });
        self.mean_number_of_shapes_guess = shapes.len().div_ceil(num_series);
        shapes
    }

    fn get_shapes_for_fractal(&self, plot_ui: &mut PlotUi) -> Vec<Shape> {
        let (special_color, positive_color) = (
            self.plot_data.special_color(),
            self.plot_data.positive_color(),
        );
        let fractal_size = self.point_size as f32;
        self.plot_data
            .all_styles_and_points_iter()
            .map(|(s, p)| {
                let center = plot_ui.screen_from_plot(*p); // in screen coords
                if s.markers.special {
                    Shape::circle_filled(center, fractal_size / 2.0, special_color)
                } else if s.markers.positive {
                    Shape::circle_filled(center, fractal_size / 2.0, positive_color)
                } else if s.markers.negative {
                    let stroke = Stroke::new(fractal_size / 4.0, s.color);
                    Shape::circle_stroke(center, fractal_size / 2.0, stroke)
                } else {
                    Shape::circle_filled(center, fractal_size, s.color)
                }
            })
            .collect::<Vec<Shape>>()
    }

    fn get_extrema_from_series(points: &Points2D) -> (Point2D, Point2D) {
        let (mut x_min, mut x_max, mut y_min, mut y_max) =
            (VALID_MAX, VALID_MIN, VALID_MAX, VALID_MIN);
        points.iter().for_each(|p| {
            if let Some(p) = p {
                x_min = x_min.min(p.x);
                x_max = x_max.max(p.x);
                y_min = y_min.min(p.y);
                y_max = y_max.max(p.y);
            }
        });
        (Point2D::new(x_min, y_min), Point2D::new(x_max, y_max))
    }

    fn set_bounds_from_points(&self, plot_ui: &mut PlotUi) {
        let mut extrema = Vec::with_capacity(self.plot_data.num_series() * 2);
        self.plot_data.extrema_iter().for_each(|(p_min, p_max)| {
            extrema.push(*p_min);
            extrema.push(*p_max);
        });
        let bounds = PlotPoints::Owned(extrema);
        plot_ui.points(Points::new(bounds).radius(0.01));
    }

    pub fn explanation(&self, ui: &mut Ui) {
        if self.plot_data.with_parameter() {
            let param_select_label = if self.parameters_are_shown() {
                LABEL_PARAMS_SHOWN
            } else {
                LABEL_PARAMS_NOT_SHOWN
            };
            ui.label(param_select_label);
        };
        let distribution_label = if self.parameters_are_shown() {
            match self.plot_data.dimensionality() {
                DistributionDimensions::State(n) => {
                    if *n == 1 {
                        LABEL_PLOT2D_PAR_STATE_1
                    } else {
                        LABEL_PLOT2D_PAR_STATE_N
                    }
                }
                DistributionDimensions::Particle(_) => LABEL_PLOT_PAR_PARTICLE,
                DistributionDimensions::Fractal(_) => LABEL_PLOT2D_PAR_FRACTAL,
            }
        } else {
            match self.plot_data.dimensionality() {
                DistributionDimensions::State(n) => {
                    if *n == 1 {
                        LABEL_PLOT2D_STATE_1
                    } else {
                        LABEL_PLOT_STATE_N
                    }
                }
                DistributionDimensions::Particle(_) => LABEL_PLOT2D_PARTICLE,
                DistributionDimensions::Fractal(_) => LABEL_PLOT2D_FRACTAL,
            }
        };
        ui.label(distribution_label);
    }

    pub fn options_ui(&mut self, ui: &mut Ui) {
        let dims = self.plot_data.dimensionality().clone();
        let num_dims = dims.number_of_dimensions();
        let mut projection_vars_to_show = Vec::with_capacity(MAX_NUM_PROJECTIONS);
        let par = self.plot_data.get_parameter();
        if let Some(p) = par {
            projection_vars_to_show.push(StateProjection::Par(p));
        }
        StateProjection::add_state_projection_vars(num_dims, &mut projection_vars_to_show);
        group_horizontal(ui, |ui| {
            let has_x_selected = StateProjection::projection_vars_selection(
                "X",
                self.projection_x.mode_string_choice(&dims),
                &mut self.selection_x,
                &projection_vars_to_show,
                &dims,
                ui,
            );
            if has_x_selected {
                self.projection_x = if self.parameters_are_shown() {
                    StateProjection::Par(par.unwrap())
                } else {
                    StateProjection::state(self.selection_x)
                }
            }
            projection_vars_to_show.clear();
            StateProjection::add_state_projection_vars(num_dims, &mut projection_vars_to_show);
            if num_dims > 1 {
                let has_y_selected = StateProjection::projection_vars_selection(
                    "Y",
                    self.projection_y.mode_string_choice(&dims),
                    &mut self.selection_y,
                    &projection_vars_to_show,
                    &dims,
                    ui,
                );
                if has_y_selected {
                    self.projection_y = StateProjection::state(self.selection_y);
                }
            } else if has_x_selected {
                self.plot_data.clear();
            }
        });
        if let Some(p) = par {
            projection_vars_to_show.push(StateProjection::Par(p));
        }
        group_horizontal(ui, |ui| {
            let has_color_selected = StateProjection::projection_vars_selection(
                "Color",
                self.plot_data.projection_color().mode_string_choice(&dims),
                &mut self.selection_color,
                &projection_vars_to_show,
                &dims,
                ui,
            );
            if has_color_selected {
                let projection_color = if self.selection_color == StateProjectionSelection::Par {
                    StateProjection::Par(par.unwrap())
                } else {
                    StateProjection::state(self.selection_color)
                };
                self.plot_data.set_projection_color(projection_color);
            }
        });
        if let DistributionDimensions::Particle(_) = dims {
        } else {
            group_horizontal(ui, |ui| {
                float_slider(
                    LABEL_POINT_SIZE,
                    &mut self.point_size,
                    10.0,
                    ui,
                    TIP_POINT_SIZE,
                );
            });
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let plot = self.axis_configured_plot();
        let num_series = self.plot_data.num_series();
        if num_series > 0 {
            let default_fill_color = if ui.visuals().dark_mode {
                Color32::DARK_BLUE
            } else {
                Color32::LIGHT_BLUE
            };
            // add all shapes to the Vec and draw at once
            let mut shapes: Vec<Shape> = Vec::new();
            let egui::Response { rect, .. } = plot
                .show(ui, |plot_ui| {
                    // set the auto-bounds functionality in plot_ui since we draw directly to screen
                    self.set_bounds_from_points(plot_ui);
                    shapes = match self.plot_data.dimensionality() {
                        DistributionDimensions::State(_) => self.get_shapes_for_all_states(plot_ui),
                        DistributionDimensions::Particle(_) => {
                            self.get_shapes_for_all_particles(default_fill_color, plot_ui)
                        }
                        DistributionDimensions::Fractal(_) => self.get_shapes_for_fractal(plot_ui),
                    };
                })
                .response;
            // ctx.layer_painter(layer_id).extend(shapes); // avoids the clipping so that points overlay the options etc.
            ui.painter().with_clip_rect(rect).extend(shapes);
        } else {
            credits(ui);
        }
    }

    fn axis_configured_plot(&self) -> Plot {
        let (x_min, x_max) = if self.parameters_are_shown() {
            (4.0 * PARAMETER_MIN, 4.0 * PARAMETER_MAX)
        } else {
            (VALID_MIN, VALID_MAX)
        };
        let (y_min, y_max) = (VALID_MIN, VALID_MAX);
        let dims = self.plot_data.dimensionality();
        let (x_label, y_label) = if let DistributionDimensions::State(1) = dims {
            (String::from("S'"), String::from("S"))
        } else {
            (
                self.projection_x.mode_string_axis(dims),
                self.projection_y.mode_string_axis(dims),
            )
        };

        let mut plot = Plot::new("plot_2_d")
            .set_margin_fraction(egui::Vec2::new(0.01, 0.01))
            .x_grid_spacer(log_grid_spacer(2))
            .x_axis_formatter(move |x, _, range| {
                if *range.start() <= x_min || *range.end() >= x_max {
                    "".to_string()
                } else if x.abs() > 0.1 {
                    format_number(x, 1).to_string()
                } else {
                    format!("{}={}", x_label, format_number(x, 1))
                }
            })
            .y_grid_spacer(log_grid_spacer(2))
            .y_axis_formatter(move |y, _, range| {
                if *range.start() <= y_min || *range.end() >= y_max {
                    "".to_string()
                } else if y.abs() > 0.1 {
                    format_number(y, 1).to_string()
                } else {
                    format!("{}={}", y_label, format_number(y, 1))
                }
            });
        if !self.parameters_are_shown() {
            plot = plot.data_aspect(1.0);
        }
        plot
    }

    delegate! {
        to self.plot_data{
            pub fn series_color_mut(&mut self)-> &mut SeriesColorChoice;
            pub fn number_of_dimensions(&self) -> usize;
            pub fn get_parameter_values(&self) -> &Vec<f64>;
            #[call(clear)]
            pub fn reset_data(&mut self);
            #[call(set_max_series)]
            pub fn set_max_num_series(&mut self, max_num_series: usize);
            #[call(set_colormap)]
            pub fn set_point_colormap(&mut self, colormap: SeriesColors);
        }
    }
}

fn credits(ui: &mut egui::Ui) {
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.hyperlink_to(
                "Source Code",
                "https://github.com/tomtuamnuq/rusty-chaos-craftor",
            );
            egui::warn_if_debug_build(ui);
        });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(", ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(", ");
            ui.hyperlink_to("plotters", "https://github.com/plotters-rs");
            ui.label(" and ");
            ui.hyperlink_to("ode-solvers", "https://github.com/srenevey/ode-solvers");
            ui.label(".");
        });
    });
}
