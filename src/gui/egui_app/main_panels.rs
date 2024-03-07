mod benchmark;
mod chaotic_plot;

pub use benchmark::BenchmarkPanel;
pub use chaotic_plot::PlotPanel;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter, Deserialize, Serialize)]
pub enum MainPanel {
    #[default]
    ChaoticPlot,
    Benchmark,
}

impl From<MainPanel> for &'static str {
    fn from(val: MainPanel) -> Self {
        match val {
            MainPanel::ChaoticPlot => "Plot",
            MainPanel::Benchmark => "Benchmark",
        }
    }
}
