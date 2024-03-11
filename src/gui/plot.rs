mod plot_2_d;
mod plot_3_d;
mod plot_colors;
mod plot_data;
mod plot_data_variants;
mod plot_styles;
mod plot_utils;
pub use self::plot_data::DEFAULT_MAX_SERIES;
pub use plot_2_d::Plot2D;
pub use plot_3_d::Plot3D;
pub use plot_colors::SeriesColors;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter, Deserialize, Serialize)]
pub enum PlotBackendVariant {
    #[default]
    EguiPlot2D,
    Plotters,
}
impl From<PlotBackendVariant> for &'static str {
    fn from(val: PlotBackendVariant) -> Self {
        match val {
            PlotBackendVariant::EguiPlot2D => "Egui 2D",
            PlotBackendVariant::Plotters => "Plotters 3D",
        }
    }
}
