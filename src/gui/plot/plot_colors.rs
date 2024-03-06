use crate::chaos::data::*;

use plotters::style::colors::colormaps::*;
use plotters::style::{Color, HSLColor, RGBColor};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::plot_backend::DEFAULT_MAX_SERIES;
#[allow(clippy::upper_case_acronyms)]
pub type RGB = (u8, u8, u8);
pub trait FromRGB {
    fn from_rgb(rgb: RGB) -> Self;
}

impl FromRGB for RGB {
    fn from_rgb(rgb: RGB) -> Self {
        rgb
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter, Deserialize, Serialize)]
pub enum SeriesColors {
    BlackWhite,
    Bone,
    Copper,
    // TODO DerivedColorMap(Vec<(f64, RGBColor)>),
    #[default]
    MandelbrotHSL,
    ViridisRGB,
    VulcanoHSL,
}

impl From<SeriesColors> for &'static str {
    fn from(val: SeriesColors) -> Self {
        match val {
            SeriesColors::BlackWhite => "Black-White",
            SeriesColors::Bone => "Bone",
            SeriesColors::Copper => "Copper",
            SeriesColors::MandelbrotHSL => "Mandelbrot",
            SeriesColors::ViridisRGB => "Viridis",
            SeriesColors::VulcanoHSL => "Vulcano",
        }
    }
}

impl SeriesColors {
    const BLACKWHITE: BlackWhite = BlackWhite {};
    const BONE: Bone = Bone {};
    const COPPER: Copper = Copper {};
    const MANDELBROT: MandelbrotHSL = MandelbrotHSL {};
    const VIRIDIS: ViridisRGB = ViridisRGB {};
    const VULCANO: VulcanoHSL = VulcanoHSL {};
    pub fn special_color(&self) -> RGB {
        match self {
            SeriesColors::BlackWhite | SeriesColors::Bone | SeriesColors::Copper => {
                SeriesColors::MANDELBROT.get_color(0.5).rgb()
            }
            _ => SeriesColors::COPPER.get_color(1.0).rgb(),
        }
    }
    pub fn positive_color(&self) -> RGB {
        match self {
            SeriesColors::BlackWhite | SeriesColors::Bone | SeriesColors::Copper => {
                SeriesColors::MANDELBROT.get_color(0.0).rgb()
            }
            _ => SeriesColors::COPPER.get_color(0.0).rgb(),
        }
    }
    pub fn color(&self, h: f32) -> RGB {
        match self {
            SeriesColors::BlackWhite => SeriesColors::BLACKWHITE.get_color(h).rgb(),
            SeriesColors::Bone => SeriesColors::BONE.get_color(h).rgb(),
            SeriesColors::Copper => SeriesColors::COPPER.get_color(h).rgb(),
            SeriesColors::MandelbrotHSL => SeriesColors::MANDELBROT.get_color(h).rgb(),
            SeriesColors::ViridisRGB => SeriesColors::VIRIDIS.get_color(h).rgb(),
            SeriesColors::VulcanoHSL => SeriesColors::VULCANO.get_color(h).rgb(),
        }
    }
    pub fn color_vec(&self, h_vec: Vec<f32>) -> Vec<RGB> {
        match self {
            SeriesColors::BlackWhite => {
                SeriesColors::color_vec_trait::<RGBColor>(SeriesColors::BLACKWHITE, h_vec)
            }
            SeriesColors::Bone => {
                SeriesColors::color_vec_trait::<RGBColor>(SeriesColors::BONE, h_vec)
            }
            SeriesColors::Copper => {
                SeriesColors::color_vec_trait::<RGBColor>(SeriesColors::COPPER, h_vec)
            }
            SeriesColors::MandelbrotHSL => {
                SeriesColors::color_vec_trait::<HSLColor>(SeriesColors::MANDELBROT, h_vec)
            }
            SeriesColors::ViridisRGB => {
                SeriesColors::color_vec_trait::<RGBColor>(SeriesColors::VIRIDIS, h_vec)
            }
            SeriesColors::VulcanoHSL => {
                SeriesColors::color_vec_trait::<HSLColor>(SeriesColors::VULCANO, h_vec)
            }
        }
    }
    fn color_vec_trait<C: Color>(colormap: impl ColorMap<C, f32>, h_vec: Vec<f32>) -> Vec<RGB> {
        h_vec
            .into_iter()
            .map(|h| colormap.get_color(h).rgb())
            .collect()
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter)]
pub enum SeriesColorChoice {
    Same,
    #[default]
    PerSeries,
    PerPoint,
    StateProjection,
}
impl From<SeriesColorChoice> for &'static str {
    fn from(val: SeriesColorChoice) -> Self {
        match val {
            SeriesColorChoice::Same => "Same",
            SeriesColorChoice::PerSeries => "Series",
            SeriesColorChoice::PerPoint => "Point",
            SeriesColorChoice::StateProjection => "State",
        }
    }
}

pub struct SeriesColorer {
    colormap: SeriesColors,
    color_counter: usize,
    max_num_colors: usize,
}

impl SeriesColorer {
    pub fn set_colormap(&mut self, colormap: SeriesColors) {
        self.colormap = colormap;
    }

    pub fn reset(&mut self) {
        self.color_counter = 0;
    }

    pub fn set_max_number_of_colors(&mut self, max_num_colors: usize) {
        self.max_num_colors = max_num_colors;
    }

    fn cloned_color_vec<C: FromRGB + Clone>(&self, h: f32, num_colors: usize) -> Vec<C> {
        vec![C::from_rgb(self.colormap.color(h)); num_colors]
    }

    fn convert_h_to_c<C: FromRGB>(&self, h_vec: Vec<f32>) -> Vec<C> {
        self.colormap
            .color_vec(h_vec)
            .into_iter()
            .map(C::from_rgb)
            .collect()
    }

    pub fn same_series_color<C: FromRGB + Clone>(&self, num_colors: usize) -> Vec<C> {
        self.cloned_color_vec(0.5, num_colors)
    }

    pub fn single_color_series<C: FromRGB + Clone>(&mut self, num_colors: usize) -> Vec<C> {
        if self.color_counter > self.max_num_colors {
            self.color_counter = 0;
        } else {
            self.color_counter += 1;
        }
        let h = self.color_counter as f32 / self.max_num_colors as f32;
        self.cloned_color_vec(h, num_colors)
    }
    pub fn color_series_by_points<V, C: FromRGB>(
        &self,
        chaos_data_vec: &[&ChaosData<V>],
    ) -> Vec<C> {
        let total_num_points = chaos_data_vec
            .first()
            .map_or(0.0, |chaos_data| chaos_data.total_num_points() as f32);
        let h_vec = chaos_data_vec
            .iter()
            .flat_map(|chaos_data| {
                let state_colors_per_param: Vec<f32> = chaos_data
                    .data()
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| v.as_ref().map(|_| i as f32 / total_num_points))
                    .collect();
                state_colors_per_param
            })
            .collect();
        self.convert_h_to_c(h_vec)
    }
    pub fn color_series_projected<C: FromRGB + Clone>(
        &self,
        projected_state_vec: Vec<ChaosFloat>,
    ) -> Vec<C> {
        let (mut min, mut max) = (ChaosFloat::INFINITY, ChaosFloat::NEG_INFINITY);
        projected_state_vec.iter().for_each(|x| {
            min = min.min(*x);
            max = max.max(*x);
        });
        let diff = max - min;
        if !min.is_finite() || !max.is_finite() || diff <= ChaosFloat::EPSILON {
            return self.cloned_color_vec(0.5, projected_state_vec.len());
        }
        let h_vec = projected_state_vec
            .into_iter()
            .map(|x| ((x - min) / diff) as f32)
            .collect();
        self.convert_h_to_c(h_vec)
    }
    pub fn special_color<C: FromRGB + Clone>(&self) -> C {
        C::from_rgb(self.colormap.special_color())
    }
    pub fn positive_color<C: FromRGB + Clone>(&self) -> C {
        C::from_rgb(self.colormap.positive_color())
    }
}

impl Default for SeriesColorer {
    fn default() -> Self {
        Self {
            colormap: Default::default(),
            color_counter: 0,
            max_num_colors: DEFAULT_MAX_SERIES,
        }
    }
}
