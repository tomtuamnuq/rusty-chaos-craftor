mod plot_2_d;
mod plot_3_d;
mod plot_backend;
mod plot_colors;
mod plot_data_variants;
mod plot_styles;
mod plot_utils;
pub use plot_2_d::Plot2D;
pub use plot_3_d::Plot3D;
pub use plot_colors::SeriesColors;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter)]
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
