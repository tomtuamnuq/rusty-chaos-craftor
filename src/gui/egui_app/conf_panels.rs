mod execute;
mod execute_chaotic_function_view;
mod initial;
mod initial_distribution_view;
pub use self::execute::*;
pub use self::initial::InitialPanel;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Default, Clone, Copy, EnumIter, Deserialize, Serialize)]
pub enum ConfPanel {
    #[default]
    Initial,
    Execution,
}

impl From<ConfPanel> for &'static str {
    fn from(val: ConfPanel) -> Self {
        match val {
            ConfPanel::Initial => "Initial Distribution",
            ConfPanel::Execution => "Chaotic Functions",
        }
    }
}
