use delegate::delegate;
use egui::Ui;
use egui_plotter::{Chart, EguiBackend, MouseConfig};

use plotters::coord::ranged3d::Cartesian3d;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;

use crate::chaos::data::*;
use crate::gui::tooltips::*;
use crate::gui::*;
use serde::{Deserialize, Serialize};
use std::ops::Range;

use super::plot_backend::PlotBackend;
use super::plot_colors::{FromRGB, SeriesColorChoice, SeriesColors, RGB};
use super::plot_utils::{StateProjection, StateProjectionSelection, MAX_NUM_PROJECTIONS};

pub type Point3D = (ChaosFloat, ChaosFloat, ChaosFloat);
pub type Points3D = Vec<Option<Point3D>>;
type BackendData = PlotBackend<Point3D, RGBColor>;
type Chart3D<'a, 'b> =
    ChartContext<'a, EguiBackend<'b>, Cartesian3d<RangedCoordf64, RangedCoordf64, RangedCoordf64>>;
struct AxisData {
    pub x_label: String,
    pub y_label: String,
    pub z_label: String,
}
impl Default for AxisData {
    fn default() -> Self {
        Self {
            x_label: String::from("x"),
            y_label: String::from("y"),
            z_label: String::from("z"),
        }
    }
}
#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(default)]
struct Options3D {
    pub point_size: f64,
    pub point_opacity: f64,
    pub show_particle_radius: bool,
    pub show_fractal_set: bool,
}
impl Default for Options3D {
    fn default() -> Self {
        Self {
            point_size: 1.0,
            point_opacity: 1.0,
            show_particle_radius: true,
            show_fractal_set: true,
        }
    }
}
impl FromRGB for RGBColor {
    fn from_rgb(rgb: RGB) -> Self {
        RGBColor(rgb.0, rgb.1, rgb.2)
    }
}
struct Chart3DWithData {
    pub chart: Chart<(BackendData, AxisData, Options3D)>,
}
fn configure_axis(chart: &mut Chart3D<'_, '_>, axis_data: &AxisData) {
    let (lx, ly, lz) = (&axis_data.x_label, &axis_data.y_label, &axis_data.z_label);
    let _ = chart
        .configure_axes()
        .label_style(("sans-serif", 12.0).into_font().color(&RED))
        .tick_size(5)
        .x_labels(3)
        .y_labels(3)
        .z_labels(3)
        .max_light_lines(2)
        .axis_panel_style(GREEN.mix(0.10))
        .bold_grid_style(BLACK.mix(0.2))
        .light_grid_style(BLACK.mix(0.10))
        .x_formatter(&|x| format!("{lx}={x}"))
        .y_formatter(&|y| format!("{ly}={y}"))
        .z_formatter(&|z| format!("{lz}={z}"))
        .draw();
}

fn plot_chaotic_states(
    mut chart: Chart3D<'_, '_>,
    series_holder: &BackendData,
    options: &Options3D,
) {
    let _ = chart.draw_series(series_holder.all_styles_and_points_iter().map(|(s, p)| {
        Circle::new(
            *p,
            options.point_size,
            ShapeStyle::from(s.color.mix(options.point_opacity)).filled(),
        )
    }));
}

fn plot_particles(mut chart: Chart3D<'_, '_>, series_holder: &BackendData, options: &Options3D) {
    let particle_size = 2.0 * options.point_size;
    let particle_stroke = 1;
    let particle_opacity = options.point_opacity;
    if options.show_particle_radius {
        let _ = chart.draw_series(series_holder.all_styles_and_points_iter().map(|(s, p)| {
            let (x, y, z) = *p;
            let r = s.radius;
            Rectangle::new(
                [(x - r, y - r, z - r), (x + r, y + r, z + r)],
                ShapeStyle::from(s.color.mix(particle_opacity)).stroke_width(particle_stroke),
            )
        }));
    }
    let guest_coord_zero = (0, 0);
    let particle_marker_shift = {
        let half_size = (particle_size / 2.0).round() as i32;
        (-half_size, -half_size)
    };
    let phantom_size = 0.3;
    let _ = chart.draw_series(series_holder.all_styles_and_points_iter().map(|(s, p)| {
        let color = ShapeStyle::from(s.color.mix(particle_opacity)).stroke_width(particle_stroke);
        let positive_marker = if s.markers.positive {
            Text::new(
                "P",
                particle_marker_shift,
                ("sans-serif", particle_size).into_font(),
            )
        } else {
            Text::new(
                "",
                guest_coord_zero,
                ("sans-serif", phantom_size).into_font(),
            )
        };
        let negative_marker = if s.markers.negative {
            Text::new(
                "N",
                particle_marker_shift,
                ("sans-serif", particle_size).into_font(),
            )
        } else {
            Text::new(
                "",
                guest_coord_zero,
                ("sans-serif", phantom_size).into_font(),
            )
        };
        let special_marker = if s.markers.special {
            Cross::new(guest_coord_zero, particle_size / 2.0, color)
        } else {
            Cross::new(guest_coord_zero, phantom_size, BLACK)
        };
        EmptyElement::at(*p) + positive_marker + negative_marker + special_marker
    }));
}
fn plot_fractal(mut chart: Chart3D<'_, '_>, series_holder: &BackendData, options: &Options3D) {
    let fractal_size = options.point_size;
    let fractal_stroke = 2;
    let fractal_opacity = options.point_opacity;
    let (positive_size, positive_color) = if options.show_fractal_set {
        (
            fractal_size,
            ShapeStyle::from(series_holder.positive_color().mix(fractal_opacity))
                .stroke_width(fractal_stroke),
        )
    } else {
        (0.01, ShapeStyle::from(BLACK.mix(0.01)))
    };
    let special_color = ShapeStyle::from(series_holder.special_color().mix(fractal_opacity))
        .stroke_width(fractal_stroke);

    let _ = chart.draw_series(series_holder.all_styles_and_points_iter().map(|(s, p)| {
        if s.markers.special {
            Circle::new(*p, fractal_size, special_color)
        } else if s.markers.positive {
            Circle::new(*p, positive_size, positive_color)
        } else if s.markers.negative {
            Circle::new(
                *p,
                fractal_size,
                s.color
                    .mix(fractal_opacity / 2.0)
                    .stroke_width(fractal_stroke / 2),
            )
        } else {
            Circle::new(
                *p,
                fractal_size,
                s.color.mix(fractal_opacity).stroke_width(fractal_stroke),
            )
        }
    }));
}
fn plot_data(chart: Chart3D<'_, '_>, series_holder: &BackendData, options: &Options3D) {
    match series_holder.dimensionality() {
        DistributionDimensions::State(_) => plot_chaotic_states(chart, series_holder, options),
        DistributionDimensions::Particle(_) => plot_particles(chart, series_holder, options),
        DistributionDimensions::Fractal(_) => plot_fractal(chart, series_holder, options),
    };
}

fn get_ranges_from_extrema(
    plot_backend: &BackendData,
) -> (Range<ChaosFloat>, Range<ChaosFloat>, Range<ChaosFloat>) {
    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) = (
        VALID_MAX, VALID_MIN, VALID_MAX, VALID_MIN, VALID_MAX, VALID_MIN,
    );
    plot_backend.extrema_iter().for_each(|(p_min, p_max)| {
        x_min = x_min.min(p_min.0);
        x_max = x_max.max(p_max.0);
        y_min = y_min.min(p_min.1);
        y_max = y_max.max(p_max.1);
        z_min = z_min.min(p_min.2);
        z_max = z_max.max(p_max.2);
    });
    (
        Range {
            start: x_min,
            end: x_max,
        },
        Range {
            start: y_min,
            end: y_max,
        },
        Range {
            start: z_min,
            end: z_max,
        },
    )
}

impl Default for Chart3DWithData {
    fn default() -> Self {
        let chart = Chart::new((
            BackendData::default(),
            AxisData::default(),
            Options3D::default(),
        ))
        .yaw(0.5)
        .pitch(0.15)
        .scale(0.9)
        .builder_cb(Box::new(|area, transform, data| {
            let (x_range, y_range, z_range) = get_ranges_from_extrema(&data.0);
            let chart_build_res = ChartBuilder::on(area)
                .margin(10)
                .build_cartesian_3d(x_range, y_range, z_range);
            match chart_build_res {
                Err(_) => (),
                Ok(mut chart) => {
                    chart.with_projection(|mut pb| {
                        pb.yaw = transform.yaw;
                        pb.pitch = transform.pitch;
                        pb.scale = transform.scale;
                        pb.into_matrix()
                    });
                    configure_axis(&mut chart, &data.1);
                    plot_data(chart, &data.0, &data.2);
                }
            };
        }));
        Self { chart }
    }
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Plot3D {
    #[serde(skip)]
    chart_with_data: Chart3DWithData,
    options: Options3D,
    #[serde(skip)] // projections are set when first series is added
    projection_x: StateProjection,
    #[serde(skip)]
    selection_x: StateProjectionSelection,
    #[serde(skip)]
    projection_y: StateProjection,
    #[serde(skip)]
    selection_y: StateProjectionSelection,
    #[serde(skip)]
    projection_z: StateProjection,
    #[serde(skip)]
    selection_z: StateProjectionSelection,
    #[serde(skip)]
    selection_color: StateProjectionSelection,
}

impl PartialEq for Plot3D {
    // TODO all fields should be considered
    fn eq(&self, other: &Self) -> bool {
        let self_data = self.series_holder();
        let other_data = other.series_holder();
        self.projection_x == other.projection_x
            && self.projection_y == other.projection_y
            && self.projection_z == other.projection_z
            && self_data.get_parameter() == other_data.get_parameter()
            && self_data.get_parameter_values() == other_data.get_parameter_values()
    }
}

impl Eq for Plot3D {}

impl Plot3D {
    fn series_holder(&self) -> &BackendData {
        &self.chart_with_data.chart.get_data().0
    }
    fn series_holder_mut(&mut self) -> &mut BackendData {
        &mut self.chart_with_data.chart.get_data_mut().0
    }
    fn axis_data_mut(&mut self) -> &mut AxisData {
        &mut self.chart_with_data.chart.get_data_mut().1
    }
    fn options_mut(&mut self) -> &mut Options3D {
        &mut self.options
    }

    fn set_options_in_chart(&mut self) {
        self.chart_with_data.chart.get_data_mut().2 = self.options
    }

    fn set_x_label(&mut self, x_label: impl Into<String>) {
        self.axis_data_mut().x_label = x_label.into();
    }
    fn set_y_label(&mut self, y_label: impl Into<String>) {
        self.axis_data_mut().y_label = y_label.into();
    }
    fn set_z_label(&mut self, z_label: impl Into<String>) {
        self.axis_data_mut().z_label = z_label.into();
    }

    fn get_extrema_from_series(points: &Points3D) -> (Point3D, Point3D) {
        let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) = (
            VALID_MAX, VALID_MIN, VALID_MAX, VALID_MIN, VALID_MAX, VALID_MIN,
        );
        points.iter().for_each(|p| {
            if let Some(p) = p {
                x_min = x_min.min(p.0);
                x_max = x_max.max(p.0);
                y_min = y_min.min(p.1);
                y_max = y_max.max(p.1);
                z_min = z_min.min(p.2);
                z_max = z_max.max(p.2);
            }
        });
        ((x_min, y_min, z_min), (x_max, y_max, z_max))
    }

    pub fn set_parameter(&mut self, parameter: &'static str, par_values: Vec<f64>) {
        let had_parameter = self.with_parameter();
        self.series_holder_mut()
            .set_parameter(parameter, par_values);
        if !had_parameter {
            self.reset_projections();
        } else if self.parameters_are_shown() {
            self.projection_x = StateProjection::Par(parameter);
        }
    }
    pub fn remove_parameter(&mut self) {
        if self.with_parameter() {
            self.series_holder_mut().remove_parameter();
            self.reset_projections();
        }
    }

    fn reset_projections(&mut self) {
        let num_dims = self.number_of_dimensions();
        let projection_color = if let Some(p) = self.get_parameter() {
            self.projection_x = StateProjection::Par(p);
            self.projection_y = StateProjection::S(0);
            self.projection_z = StateProjection::S(1);
            if num_dims > 2 {
                StateProjection::S(2)
            } else if num_dims > 1 {
                StateProjection::S(1)
            } else {
                StateProjection::S(0)
            }
        } else {
            self.projection_x = StateProjection::S(0);
            self.projection_y = StateProjection::S(1);
            self.projection_z = StateProjection::S(2);
            if num_dims > 3 {
                StateProjection::S(3)
            } else if num_dims > 2 {
                StateProjection::S(2)
            } else if num_dims > 1 {
                StateProjection::S(1)
            } else {
                StateProjection::S(0)
            }
        };
        self.set_projection_color(projection_color);
        self.selection_color = StateProjectionSelection::from(projection_color);
        self.selection_x = StateProjectionSelection::from(self.projection_x);
        self.selection_y = StateProjectionSelection::from(self.projection_y);
        self.selection_z = StateProjectionSelection::from(self.projection_z);
    }

    pub fn add_point_series(&mut self, data: ChaosDataVec<'_>) {
        let dimensionality = data.dimensionality();
        if dimensionality != *self.series_holder().dimensionality() {
            self.remove_parameter();
            self.set_dimensionality(dimensionality);
            self.reset_projections();
        }
        let styles = self.series_holder_mut().create_styles_for_chaos_data(&data);
        let series = if self.parameters_are_shown() {
            self.set_x_label(
                self.get_parameter()
                    .expect("Parameter exists if projection is Par"),
            );
            self.create_point_series_with_parameters(data)
        } else {
            self.create_point_series_without_parameters(data)
        };
        let extrema = Self::get_extrema_from_series(&series);
        self.series_holder_mut().add_series(series, styles, extrema);
    }

    pub fn transform_points_1_d(&self, states: &[Option<State1>]) -> Points3D {
        match self.series_holder().latest_series() {
            None => states.iter().map(|v| v.map(|v| (0.0, 0.0, v[0]))).collect(),
            Some((last_states, _)) => states
                .iter()
                .zip(last_states.iter())
                .map(|(new_state, last_state)| {
                    if let (Some(new_state), Some(last_state)) = (new_state, last_state) {
                        Some((last_state.1, last_state.2, new_state[0]))
                    } else {
                        None
                    }
                })
                .collect(), // x=S1'', y=S1', z=S1
        }
    }
    pub fn transform_points_2_d(&self, states: &[Option<State2>]) -> Points3D {
        let t = self.series_holder().num_series();
        states
            .iter()
            .map(|v| v.map(|v| (t as ChaosFloat, v[0], v[1])))
            .collect() // x=t, y=S1', z=S1
    }

    pub fn transform_points_n_d<V: StateIndex>(&self, states: &[Option<V>]) -> Points3D {
        let (i_x, i_y, i_z) = (
            self.projection_x.index(),
            self.projection_y.index(),
            self.projection_z.index(),
        );
        states
            .iter()
            .map(|v| v.as_ref().map(|v| (v.ind(i_x), v.ind(i_y), v.ind(i_z))))
            .collect()
    }

    fn parameters_are_shown(&self) -> bool {
        self.selection_x == StateProjectionSelection::Par
    }

    pub fn points_with_parameter_1_d(&self, states: &[Option<State1>], par: &f64) -> Points3D {
        let t = self.series_holder().num_series();
        states
            .iter()
            .map(|v| v.map(|v| (*par, t as ChaosFloat, v[0])))
            .collect() // x=par, y=t, z=S1
    }

    pub fn points_with_parameter_n_d<V: StateIndex>(
        &self,
        states: &[Option<V>],
        par: &f64,
    ) -> Points3D {
        let (ind_y, ind_z) = (self.projection_y.index(), self.projection_z.index());
        states
            .iter()
            .map(|v| v.as_ref().map(|v| (*par, v.ind(ind_y), v.ind(ind_z))))
            .collect()
    }

    fn set_axis_labels(&mut self) {
        let dims = self.dimensionality().clone();
        let num_dims = dims.number_of_dimensions();
        if self.parameters_are_shown() {
            self.set_x_label(
                self.get_parameter()
                    .expect("Parameter exists if projection is Par"),
            );
            if num_dims == 1 {
                self.set_y_label("t");
                self.set_z_label("S1");
            } else {
                self.set_y_label(self.projection_y.mode_string_axis(&dims));
                self.set_z_label(self.projection_z.mode_string_axis(&dims));
            }
        } else if num_dims == 1 {
            self.set_x_label("S''");
            self.set_y_label("S'");
            self.set_z_label("S");
        } else if num_dims == 2 {
            self.set_x_label("t");
            self.set_y_label("S1");
            self.set_z_label("S2");
        } else {
            self.set_x_label(self.projection_x.mode_string_axis(&dims));
            self.set_y_label(self.projection_y.mode_string_axis(&dims));
            self.set_z_label(self.projection_z.mode_string_axis(&dims));
        };
    }

    pub fn explanation(&self, ui: &mut Ui) {
        if self.with_parameter() {
            let param_select_label = if self.parameters_are_shown() {
                LABEL_PARAMS_SHOWN
            } else {
                LABEL_PARAMS_NOT_SHOWN
            };
            ui.label(param_select_label);
        };
        let distribution_label = if self.parameters_are_shown() {
            match self.dimensionality() {
                DistributionDimensions::State(n) => {
                    if *n == 1 {
                        LABEL_PLOT3D_PAR_STATE_1
                    } else {
                        LABEL_PLOT3D_PAR_STATE_N
                    }
                }
                DistributionDimensions::Particle(_) => LABEL_PLOT_PAR_PARTICLE,
                DistributionDimensions::Fractal(_) => LABEL_PLOT3D_PAR_FRACTAL,
            }
        } else {
            match self.dimensionality() {
                DistributionDimensions::State(n) => match *n {
                    1 => LABEL_PLOT3D_STATE_1,
                    2 => LABEL_PLOT3D_STATE_2,
                    _ => LABEL_PLOT_STATE_N,
                },
                DistributionDimensions::Particle(_) => LABEL_PLOT3D_PARTICLE,
                DistributionDimensions::Fractal(_) => LABEL_PLOT3D_FRACTAL,
            }
        };
        ui.label(distribution_label);
    }

    pub fn options_ui(&mut self, ui: &mut Ui) {
        let dims = self.dimensionality().clone();
        let num_dims = dims.number_of_dimensions();
        let mut projection_vars_to_show = Vec::with_capacity(MAX_NUM_PROJECTIONS);
        let par = self.get_parameter();
        if let Some(p) = par {
            projection_vars_to_show.push(StateProjection::Par(p));
        }
        StateProjection::add_state_projection_vars(num_dims, &mut projection_vars_to_show);
        let mut has_x_selected = false;
        group_horizontal(ui, |ui| {
            has_x_selected = StateProjection::projection_vars_selection(
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
        });
        projection_vars_to_show.clear();
        StateProjection::add_state_projection_vars(num_dims, &mut projection_vars_to_show);
        if num_dims > 2 {
            group_horizontal(ui, |ui| {
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
            });
            group_horizontal(ui, |ui| {
                let has_z_selected = StateProjection::projection_vars_selection(
                    "Z",
                    self.projection_z.mode_string_choice(&dims),
                    &mut self.selection_z,
                    &projection_vars_to_show,
                    &dims,
                    ui,
                );
                if has_z_selected {
                    self.projection_z = StateProjection::state(self.selection_z);
                }
            });
        } else if has_x_selected {
            self.reset_data();
        }
        if let Some(p) = par {
            projection_vars_to_show.push(StateProjection::Par(p));
        }
        group_horizontal(ui, |ui| {
            let has_color_selected = StateProjection::projection_vars_selection(
                "Color",
                self.series_holder()
                    .projection_color()
                    .mode_string_choice(&dims),
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
                self.set_projection_color(projection_color);
            }
        });
        let options = self.options_mut();
        group_horizontal(ui, |ui| {
            float_slider(
                LABEL_POINT_SIZE,
                &mut options.point_size,
                10.0,
                ui,
                TIP_POINT_SIZE,
            );
        });
        group_horizontal(ui, |ui| {
            float_slider(
                LABEL_POINT_OPACITY,
                &mut options.point_opacity,
                1.0,
                ui,
                TIP_POINT_OPACITY,
            );
        });
        match dims {
            DistributionDimensions::State(_) => (),
            DistributionDimensions::Particle(_) => {
                group_horizontal(ui, |ui| {
                    add_checkbox(
                        "Square Radius",
                        &mut options.show_particle_radius,
                        ui,
                        TIP_PARTICLE_RADIUS,
                    );
                });
            }
            DistributionDimensions::Fractal(_) => {
                group_horizontal(ui, |ui| {
                    add_checkbox(
                        "show set",
                        &mut options.show_fractal_set,
                        ui,
                        TIP_FRACTAL_SET,
                    );
                });
            }
        }
    }

    pub fn ui(&mut self, mouse_is_over_plot: bool, ui: &mut Ui) {
        self.set_axis_labels();
        let mouse_config = MouseConfig::default()
            .rotate(mouse_is_over_plot)
            .pitch_scale(0.02); // TODO test drag and zoom
        self.chart_with_data.chart.set_mouse(mouse_config);
        self.set_options_in_chart();
        self.chart_with_data.chart.draw(ui);
    }
    delegate! {
        to self.series_holder(){
            pub fn dimensionality(&self) -> &DistributionDimensions;
            pub fn number_of_dimensions(&self) -> usize;
            pub fn get_parameter(&self) -> Option<&'static str>;
            pub fn get_parameter_values(&self) -> &Vec<f64>;
            pub fn with_parameter(&self)->bool;
        }
        to self.series_holder_mut(){
            pub fn series_color_mut(&mut self)-> &mut SeriesColorChoice;
            pub fn set_dimensionality(&mut self, dims: DistributionDimensions);
            pub fn set_projection_color(&mut self, projection_color: StateProjection);
            #[call(clear)]
            pub fn reset_data(&mut self);
            #[call(set_max_series)]
            pub fn set_max_num_series(&mut self, max_num_series: usize);
            #[call(set_colormap)]
            pub fn set_point_colormap(&mut self, colormap: SeriesColors);
        }
    }
}
