use super::plot_colors::{FromRGB, SeriesColorChoice, SeriesColorer, SeriesColors};
use super::plot_styles::{ColoredStyle, Style};
use super::plot_utils::StateProjection;
use crate::chaos::data::{ChaosData, DistributionDimensions, StateIndex};
use delegate::delegate;
use std::collections::vec_deque::{self, VecDeque};
pub const DEFAULT_MAX_SERIES: usize = 20;

struct PlotSeriesHolder<S> {
    series_collection: VecDeque<S>,
    max_num_series: usize,
}

impl<S> PlotSeriesHolder<S> {
    pub fn set_max_series(&mut self, max_num_series: usize) {
        while self.series_collection.len() > max_num_series {
            self.series_collection.pop_front();
        }
        self.max_num_series = max_num_series;
    }

    pub fn add_series(&mut self, series: S) {
        if self.series_collection.len() == self.max_num_series {
            self.series_collection.pop_front();
        }
        self.series_collection.push_back(series);
    }

    delegate! {
        to self.series_collection{
            pub fn clear(&mut self);
            #[call(len)]
            pub fn num_series(&self) -> usize;
            #[call(back)]
            pub fn latest_series(&self) -> Option<&S>;
            #[call(iter)]
            pub fn series_iter(
                &self,
            ) -> vec_deque::Iter<'_, S>;
        }
    }
}

impl<S> Default for PlotSeriesHolder<S> {
    fn default() -> Self {
        Self {
            series_collection: VecDeque::new(),
            max_num_series: DEFAULT_MAX_SERIES,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct PlotDimensions {
    dimensions: DistributionDimensions,
    parameter: Option<&'static str>,
    par_values: Vec<f64>,
}

impl Default for PlotDimensions {
    fn default() -> Self {
        Self {
            dimensions: DistributionDimensions::State(0),
            parameter: None,
            par_values: Vec::new(),
        }
    }
}

impl PlotDimensions {
    pub fn set_parameter(&mut self, parameter: &'static str, par_values: Vec<f64>) {
        self.parameter = Some(parameter);
        self.par_values = par_values;
    }
    pub fn remove_parameter(&mut self) {
        self.parameter = None;
        self.par_values.clear();
    }
    pub fn with_parameter(&self) -> bool {
        self.get_parameter().is_some()
    }
    pub fn get_parameter(&self) -> Option<&'static str> {
        self.parameter
    }
    pub fn get_parameter_values(&self) -> &Vec<f64> {
        &self.par_values
    }

    pub fn dimensionality(&self) -> &DistributionDimensions {
        &self.dimensions
    }
    pub fn set_dimensionality(&mut self, dims: DistributionDimensions) {
        self.dimensions = dims;
    }
    pub fn number_of_dimensions(&self) -> usize {
        self.dimensions.number_of_dimensions()
    }
}

type StyledSeries<P, C> = (Vec<Option<P>>, Vec<Style<C>>);
pub struct PlotBackend<P, C> {
    point_holder: PlotSeriesHolder<StyledSeries<P, C>>,
    extrema_holder: PlotSeriesHolder<(P, P)>,
    series_colorer: SeriesColorer,
    plot_dimensions: PlotDimensions,
    projection_color: StateProjection,
    selection_series_color: SeriesColorChoice,
}

impl<P, C> PartialEq for PlotBackend<P, C> {
    fn eq(&self, other: &Self) -> bool {
        // only compare options for reset
        self.series_colorer == other.series_colorer
            && self.plot_dimensions == other.plot_dimensions
            && self.projection_color == other.projection_color
            && self.selection_series_color == other.selection_series_color
    }
}

impl<P, C: FromRGB> Default for PlotBackend<P, C> {
    fn default() -> Self {
        Self {
            point_holder: Default::default(),
            extrema_holder: Default::default(),
            series_colorer: Default::default(),
            plot_dimensions: Default::default(),
            projection_color: Default::default(),
            selection_series_color: Default::default(),
        }
    }
}

impl<P, C> PlotBackend<P, C> {
    pub fn projection_color(&self) -> StateProjection {
        self.projection_color
    }
    pub fn set_projection_color(&mut self, projection_color: StateProjection) {
        self.projection_color = projection_color;
        self.selection_series_color = SeriesColorChoice::StateProjection;
    }
    pub fn series_color_mut(&mut self) -> &mut SeriesColorChoice {
        &mut self.selection_series_color
    }
    pub fn clear(&mut self) {
        self.point_holder.clear();
        self.extrema_holder.clear();
        self.series_colorer.reset();
    }

    pub fn set_max_series(&mut self, max_num_series: usize) {
        self.point_holder.set_max_series(max_num_series);
        self.extrema_holder.set_max_series(max_num_series);
        self.series_colorer.set_max_number_of_colors(max_num_series);
    }
    pub fn add_series(&mut self, series: Vec<Option<P>>, styles: Vec<Style<C>>, extrema: (P, P)) {
        self.point_holder.add_series((series, styles));
        self.extrema_holder.add_series(extrema);
    }

    pub fn all_styles_and_points_iter(&self) -> impl Iterator<Item = (&Style<C>, &P)> {
        self.styled_series_iter().flat_map(|(points, styles)| {
            styles.iter().zip(points.iter().filter_map(|p| p.as_ref()))
        })
    }

    delegate! {
        to self.series_colorer{
            pub fn set_colormap(&mut self, colormap: SeriesColors);
        }
        to self.plot_dimensions{
            pub fn set_parameter(&mut self, parameter: &'static str, par_values: Vec<f64>);
            pub fn remove_parameter(&mut self);
            pub fn with_parameter(&self) -> bool;
            pub fn get_parameter(&self) -> Option<&'static str>;
            pub fn get_parameter_values(&self) -> &Vec<f64>;
            pub fn dimensionality(&self) -> &DistributionDimensions;
            pub fn set_dimensionality(&mut self, dims: DistributionDimensions);
            pub fn number_of_dimensions(&self) -> usize;
        }
        to self.point_holder{
            pub fn num_series(&self) -> usize;
            pub fn latest_series(&self) -> Option<&StyledSeries<P, C>>;
            #[call(series_iter)]
            pub fn styled_series_iter(
                &self,
            ) -> vec_deque::Iter<'_, StyledSeries<P, C>>;
        }
        to self.extrema_holder{
            #[call(series_iter)]
            pub fn extrema_iter(
                &self,
            ) -> vec_deque::Iter<'_, (P, P)>;
        }
    }
}
fn generic_flattened_states<'a, V>(
    chaos_data_vec: &[&'a ChaosData<V>],
) -> (Vec<&'a V>, Vec<usize>) {
    let mut num_valid_states_per_param = Vec::with_capacity(chaos_data_vec.len());
    let flattened_valid_states = chaos_data_vec
        .iter()
        .flat_map(|chaos_data| {
            let valid_states = chaos_data.data_filtered();
            num_valid_states_per_param.push(valid_states.len());
            valid_states
        })
        .collect();
    (flattened_valid_states, num_valid_states_per_param)
}
impl<P, C: FromRGB + Clone> PlotBackend<P, C> {
    delegate! {
        to self.series_colorer{
            pub fn special_color(&self) -> C;
            pub fn positive_color(&self) -> C;
        }
    }

    pub fn create_styles_for_chaos_data_generic<V: StateIndex + ColoredStyle<C>>(
        &mut self,
        chaos_data_vec: &[&ChaosData<V>],
    ) -> Vec<Style<C>> {
        let color_parameters = matches!(self.projection_color, StateProjection::Par(_));
        let (flattened_state_refs, num_valid_states_per_param) =
            generic_flattened_states(chaos_data_vec);
        let num_existing_states = flattened_state_refs.len();
        let color_vec = match self.selection_series_color {
            SeriesColorChoice::Same => self.series_colorer.same_series_color(num_existing_states),
            SeriesColorChoice::PerSeries => {
                self.series_colorer.single_color_series(num_existing_states)
            }
            SeriesColorChoice::PerPoint => {
                self.series_colorer.color_series_by_points(chaos_data_vec)
            }
            SeriesColorChoice::StateProjection => {
                let projected_floats = if color_parameters {
                    num_valid_states_per_param
                        .into_iter()
                        .zip(self.get_parameter_values().iter())
                        .flat_map(|(num_valid_states, par)| vec![*par; num_valid_states])
                        .collect()
                } else {
                    let color_projection_index = self.projection_color.index();
                    flattened_state_refs
                        .iter()
                        .map(|v| v.ind(color_projection_index))
                        .collect()
                };
                self.series_colorer.color_series_projected(projected_floats)
            }
        };
        flattened_state_refs
            .into_iter()
            .zip(color_vec)
            .map(|(v, c)| v.colored_style(c))
            .collect()
    }
}
mod tests {

    use super::*;
    use crate::{chaos::data::*, gui::plot::plot_colors::RGB};
    #[test]
    fn test_state2_style_creation() {
        let num_samples = 2;
        let init_distr_1 = vec![
            InitialDistributionVariant::Fixed(Fixed { value: 1.1 }),
            InitialDistributionVariant::Fixed(Fixed { value: 1.2 }),
        ];
        let init_distr_2 = vec![
            InitialDistributionVariant::Linspace(Linspace {
                low: 0.0,
                high: 0.5,
            }),
            InitialDistributionVariant::Fixed(Fixed { value: 2.2 }),
        ];
        let chaos_data_1: ChaosData<State2> = ChaosData::new(num_samples, &init_distr_1);
        let chaos_data_2: ChaosData<State2> = ChaosData::new(num_samples, &init_distr_2);
        let chaos_data_vec = vec![&chaos_data_1, &chaos_data_2];
        let mut plot_backend: PlotBackend<(ChaosFloat, ChaosFloat), RGB> = Default::default();
        plot_backend.set_colormap(SeriesColors::BlackWhite);
        *plot_backend.series_color_mut() = SeriesColorChoice::Same;
        let styles_same = plot_backend.create_styles_for_chaos_data_generic(&chaos_data_vec);
        assert_eq!(
            styles_same[1], styles_same[2],
            "Styles must have the same color and default values!"
        );
        *plot_backend.series_color_mut() = SeriesColorChoice::PerPoint;
        let styles_point = plot_backend.create_styles_for_chaos_data_generic(&chaos_data_vec);
        assert_eq!(styles_point[0], styles_point[2], "Styles must have same value when they represent the same initial state for two parameter configurations!");
        assert_eq!(styles_point[1], styles_point[3], "Styles must have same value when they represent the same initial state for two parameter configurations");
        assert_ne!(
            styles_point[0], styles_point[1],
            "Styles must be different when represent different states!"
        );
        *plot_backend.series_color_mut() = SeriesColorChoice::PerSeries;
        let styles_series = plot_backend.create_styles_for_chaos_data_generic(&chaos_data_vec);
        assert_eq!(
            styles_series[0], styles_series[1],
            "Styles must have same values since they were plotted in the same series(even though different parameter configurations)!"
        );
        plot_backend.remove_parameter();
        plot_backend.set_projection_color(StateProjection::S(0));
        *plot_backend.series_color_mut() = SeriesColorChoice::StateProjection;
        let styles_projection = plot_backend.create_styles_for_chaos_data_generic(&chaos_data_vec);
        assert_ne!(
            styles_projection[0], styles_projection[2],
            "Styles must be different when color represents the different state values!"
        );
        plot_backend.set_parameter("t", vec![-1.0, 1.0]);
        let styles_projection = plot_backend.create_styles_for_chaos_data_generic(&chaos_data_vec);
        assert_eq!(
            styles_projection[0], styles_projection[1],
            "Styles must be equal when color represents the same parameter value!"
        );
        assert_ne!(
            styles_projection[0], styles_projection[2],
            "Styles must be different when color represents the different parameter value!"
        );
    }
}
