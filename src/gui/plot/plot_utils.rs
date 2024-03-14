use std::cmp::Ordering;

use crate::{
    chaos::{data::*, labels::*},
    gui::combo_box_from_string,
};
use egui::Ui;
use serde::{Deserialize, Serialize};
pub fn flat_map_data_vec<V: FromStateVec + ValidStateCheck, P>(
    data_vec: Vec<&ChaosData<V>>,
    f: impl Fn(&Vec<Option<V>>) -> Vec<Option<P>>,
) -> Vec<Option<P>> {
    data_vec.iter().flat_map(|data| f(data.data())).collect()
}
pub fn flat_map_data_vec_and_parameter<V: FromStateVec + ValidStateCheck, P>(
    data_vec: Vec<&ChaosData<V>>,
    par_values: &[f64],
    f: impl Fn(&Vec<Option<V>>, &f64) -> Vec<Option<P>>,
) -> Vec<Option<P>> {
    data_vec
        .iter()
        .zip(par_values.iter())
        .flat_map(|(data, par)| f(data.data(), par))
        .collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Deserialize, Serialize)]
pub enum StateProjection {
    Par(&'static str),
    S(usize),
}
impl Default for StateProjection {
    fn default() -> Self {
        StateProjection::S(0)
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum StateProjectionSelection {
    Par,
    #[default]
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
}

impl From<StateProjection> for StateProjectionSelection {
    fn from(value: StateProjection) -> Self {
        match value {
            StateProjection::Par(_) => Self::Par,
            StateProjection::S(s) => match s {
                0 => Self::S0,
                1 => Self::S1,
                2 => Self::S2,
                3 => Self::S3,
                4 => Self::S4,
                5 => Self::S5,
                6 => Self::S6,
                7 => Self::S7,
                8 => Self::S8,
                9 => Self::S9,
                _ => Self::S0,
            },
        }
    }
}
pub const MAX_NUM_PROJECTIONS: usize = 10;
impl StateProjection {
    pub fn index(&self) -> usize {
        match self {
            StateProjection::Par(_) => 0,
            StateProjection::S(s) => *s,
        }
    }
    pub fn add_state_projection_vars(dims: usize, variants: &mut Vec<StateProjection>) {
        let mut i = 0;
        while dims > i {
            variants.push(StateProjection::S(i));
            i += 1;
        }
    }
    pub fn mode_string_choice(&self, dims: &DistributionDimensions) -> String {
        match self {
            Self::Par(p) => format!("Par {p}"),
            Self::S(s) => match dims {
                DistributionDimensions::State(_) => {
                    format!("State {}", *s + 1)
                }
                DistributionDimensions::Particle(cartesian_dims) => match cartesian_dims {
                    2 => LABELS_PARTICLE_2D[*s].into(),
                    3 => LABELS_PARTICLE_3D[*s].into(),
                    _ => String::from("Error"),
                },
                DistributionDimensions::Fractal(fractal_mode) => match fractal_mode {
                    FractalDimensions::Complex => LABELS_COMPLEX[*s].into(),
                    FractalDimensions::Dual => LABELS_DUAL[*s].into(),
                    FractalDimensions::Perplex => LABELS_PERPLEX[*s].into(),
                    FractalDimensions::Quaternion => LABELS_QUATERNION[*s].into(),
                },
            },
        }
    }
    pub fn mode_string_axis(&self, dims: &DistributionDimensions) -> String {
        match self {
            Self::Par(p) => String::from(*p),
            Self::S(s) => match dims {
                DistributionDimensions::State(state) => match s.cmp(state) {
                    Ordering::Less => format!("S{}", *s + 1),
                    Ordering::Equal => String::from("Min"),
                    Ordering::Greater => String::from("Max"),
                },
                DistributionDimensions::Particle(cartesian_dims) => match cartesian_dims {
                    2 => LABELS_SHORT_PARTICLE_2D[*s].into(),
                    3 => LABELS_SHORT_PARTICLE_3D[*s].into(),
                    _ => String::from("Error"),
                },
                DistributionDimensions::Fractal(fractal_mode) => match fractal_mode {
                    FractalDimensions::Complex => LABELS_SHORT_COMPLEX[*s].into(),
                    FractalDimensions::Dual => LABELS_SHORT_DUAL[*s].into(),
                    FractalDimensions::Perplex => LABELS_SHORT_PERPLEX[*s].into(),
                    FractalDimensions::Quaternion => LABELS_SHORT_QUATERNION[*s].into(),
                },
            },
        }
    }
    pub fn state(var: StateProjectionSelection) -> Self {
        let s = match var {
            StateProjectionSelection::Par => 0,
            StateProjectionSelection::S0 => 0,
            StateProjectionSelection::S1 => 1,
            StateProjectionSelection::S2 => 2,
            StateProjectionSelection::S3 => 3,
            StateProjectionSelection::S4 => 4,
            StateProjectionSelection::S5 => 5,
            StateProjectionSelection::S6 => 6,
            StateProjectionSelection::S7 => 7,
            StateProjectionSelection::S8 => 8,
            StateProjectionSelection::S9 => 9,
        };
        Self::S(s)
    }
    pub fn projection_vars_selection(
        label: &'static str,
        var_label: String,
        var_selection: &mut StateProjectionSelection,
        vars: &[StateProjection],
        dims: &DistributionDimensions,
        ui: &mut Ui,
    ) -> bool {
        let projection_with_labels = vars
            .iter()
            .map(|v| {
                (
                    StateProjectionSelection::from(*v),
                    v.mode_string_choice(dims),
                )
            })
            .collect();
        let tooltip = format!("Choose which data feature to project on {}", label);
        combo_box_from_string(
            label,
            (var_selection, var_label),
            ui,
            projection_with_labels,
            tooltip.as_str(),
        )
    }
}
