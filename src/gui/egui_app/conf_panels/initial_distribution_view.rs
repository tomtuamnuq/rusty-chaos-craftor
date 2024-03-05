use crate::gui::add_hyperlink;
use crate::chaos::data::{self as chaos_creation}; // avoid name collision
use crate::chaos::ChaosDescription;
use egui::{Response, Ui};
use paste::paste;
use strum_macros::EnumIter;

fn parameter_view(
    par: &mut f64,
    suffix: &str,
    range: std::ops::RangeInclusive<f64>,
    ui: &mut Ui,
) -> Response {
    let response = ui.add(
        egui::DragValue::new(par)
            .speed(0.1)
            .clamp_range(range) // Range inclusive
            .suffix(format!(" {}", suffix)),
    );
    response
}

macro_rules! generate_initial_distribution_views {
    ($($variant:ident { $($field:ident),* }),*) => {
        paste!{

            #[derive(PartialEq, Eq, Default, Copy, Clone, Debug, EnumIter)]
            pub enum InitialDistributionView {
                #[default]
                $(
                    $variant,
                )*
            }
            #[derive(PartialEq, Default)]
            pub struct InitialDistributionViewData {
                $(
                    pub [<$variant:lower>]: $variant,
                )*
            }

            impl InitialDistributionViewData {
                pub fn map_initial_distribution_view_to_data(
                    &self,
                    view: &InitialDistributionView,
                ) -> chaos_creation::InitialDistributionVariant {
                    match view {
                        $(
                            InitialDistributionView::$variant => self.[<$variant:lower>].to_initial_variant(),
                        )*
                    }
                }

                pub fn view_ui(&mut self, view: InitialDistributionView, ui: &mut Ui) {
                    match view {
                        $(
                            InitialDistributionView::$variant => self.[<$variant:lower>].ui(ui),
                        )*
                    }
                }
            }
            $(
                #[derive(PartialEq, Default)]
                pub struct $variant {
                    pub data: chaos_creation::$variant,
                }
                impl $variant {
                    fn to_initial_variant(&self) -> chaos_creation::InitialDistributionVariant {
                        chaos_creation::InitialDistributionVariant::$variant(self.data)
                    }
                    pub fn ui(&mut self, ui: &mut Ui) {
                        $(
                            let range = chaos_creation::$variant::[<RANGE_ $field:upper>];
                            let response = parameter_view(&mut self.data.$field, stringify!($field), range, ui);
                            if response.changed() {
                                self.data.par_range_check();
                            };
                        )*
                        add_hyperlink("Info", self.data.reference(), ui, self.data.description().as_str());
                    }
                }
            )*
        } // paste
    };
}

generate_initial_distribution_views! {
    Normal { mean, std_dev },
    Cauchy { median, scale },
    Uniform { low, high },
    Exponential { lambda },
    LogNormal { mean, std_dev },
    Poisson { mean },
    Pareto { scale, shape },
    StudentT { dof },
    Weibull { lambda, k },
    Gamma { shape, scale },
    Beta { alpha, beta },
    Triangular { low, high, mode },
    ChiSquared { dof },
    Fixed { value },
    Linspace { low, high },
    Mesh { start, end },
    Geomspace { start, end },
    Eye { value },
    Logspace { start, end, base }
}

pub const INITIAL_MESHES: [InitialDistributionView; 1] = [InitialDistributionView::Mesh];
pub const INITIAL_DETERMINISTIC: [InitialDistributionView; 5] = [
    InitialDistributionView::Fixed,
    InitialDistributionView::Linspace,
    InitialDistributionView::Geomspace,
    InitialDistributionView::Eye,
    InitialDistributionView::Logspace,
];
pub const INITIAL_PROBABILISTIC: [InitialDistributionView; 13] = [
    InitialDistributionView::Normal,
    InitialDistributionView::Cauchy,
    InitialDistributionView::Uniform,
    InitialDistributionView::Exponential,
    InitialDistributionView::LogNormal,
    InitialDistributionView::Poisson,
    InitialDistributionView::Pareto,
    InitialDistributionView::StudentT,
    InitialDistributionView::Weibull,
    InitialDistributionView::Gamma,
    InitialDistributionView::Beta,
    InitialDistributionView::Triangular,
    InitialDistributionView::ChiSquared,
];

impl From<InitialDistributionView> for &'static str {
    fn from(val: InitialDistributionView) -> Self {
        match val {
            InitialDistributionView::Normal => "Normal Distribution",
            InitialDistributionView::Cauchy => "Cauchy Distribution",
            InitialDistributionView::Uniform => "Uniform Distribution",
            InitialDistributionView::Exponential => "Exponential Distribution",
            InitialDistributionView::LogNormal => "Log-Normal Distribution",
            InitialDistributionView::Poisson => "Poisson Distribution",
            InitialDistributionView::Pareto => "Pareto Distribution",
            InitialDistributionView::StudentT => "Student's t-Distribution",
            InitialDistributionView::Weibull => "Weibull Distribution",
            InitialDistributionView::Gamma => "Gamma Distribution",
            InitialDistributionView::Beta => "Beta Distribution",
            InitialDistributionView::Triangular => "Triangular Distribution",
            InitialDistributionView::ChiSquared => "Chi-squared Distribution",
            InitialDistributionView::Fixed => "Fixed Value",
            InitialDistributionView::Linspace => "Linspace",
            InitialDistributionView::Mesh => "(Hyper) Mesh",
            InitialDistributionView::Geomspace => "Geometric Space",
            InitialDistributionView::Eye => "Identity Matrix (Eye)",
            InitialDistributionView::Logspace => "Logarithmic Space",
        }
    }
}

impl From<InitialDistributionView> for String {
    fn from(val: InitialDistributionView) -> Self {
        let val_label: &'static str = val.into();
        String::from(val_label) // val.into() does panic ?!
    }
}
