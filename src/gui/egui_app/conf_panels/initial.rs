use super::initial_distribution_view::{
    InitialDistributionView, InitialDistributionViewData, INITIAL_DETERMINISTIC, INITIAL_MESHES,
    INITIAL_PROBABILISTIC,
};
use crate::chaos::data::*;
use crate::chaos::labels::*;
use crate::gui::tooltips::*;
use crate::gui::*;
use egui::Ui;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, IntoStaticStr};

const MAX_NUM_STATE_DIMS: usize = 4;

fn generate_initital_distribution_variant(
    open_initial_distributions: &[InitialDistributionViewSelection],
    all_initial_distributions: &[InitialDistributionViewData],
    i: usize,
) -> InitialDistributionVariant {
    let open = open_initial_distributions[i];
    let initial_distributions = &all_initial_distributions[i];
    initial_distributions.map_initial_distribution_view_to_data(&open.view)
}
fn distribution_selection(
    open: &mut InitialDistributionViewSelection,
    all_initital_distributions: &mut InitialDistributionViewData,
    label: &str,
    show_space_distributions: bool,
    ui: &mut Ui,
    distributions_tooltip: &str,
) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let (groups, tooltip): (&[InitialDistributionGroup], &str) =
                    if show_space_distributions {
                        (&DISTRIBUTIONS_ALL, TIP_DISTRIBUTIONS_ALL)
                    } else {
                        (&DISTRIBUTIONS_NO_MESH, TIP_DISTRIBUTIONS_NO_MESH)
                    };
                select_group_filtered(open.group_mut(), ui, groups, tooltip);
            });
            ui.horizontal(|ui| {
                let group_variants: &[InitialDistributionView] = match open.group {
                    InitialDistributionGroup::Probabilistic => &INITIAL_PROBABILISTIC,
                    InitialDistributionGroup::Deterministic => &INITIAL_DETERMINISTIC,
                    InitialDistributionGroup::Mesh => &INITIAL_MESHES,
                };
                let group_with_labels: Vec<(InitialDistributionView, String)> = group_variants
                    .iter()
                    .map(|view| (*view, String::from(*view)))
                    .collect();
                let current_label = String::from(open.view);
                combo_box_from_string(
                    label,
                    (open.view_mut(), current_label),
                    ui,
                    group_with_labels,
                    distributions_tooltip,
                );
                all_initital_distributions.view_ui(open.view, ui);
            });
        });
    });
}

#[derive(PartialEq, Clone, Copy, Default, EnumIter, IntoStaticStr, Deserialize, Serialize)]
enum InitialMode {
    #[default]
    States,
    Particle,
    Fractals,
}

#[derive(PartialEq, Deserialize, Serialize)]
#[serde(default)]
struct InitialStateData {
    open_initial_distributions: [InitialDistributionViewSelection; MAX_NUM_STATE_DIMS],
    all_initital_distributions: [InitialDistributionViewData; MAX_NUM_STATE_DIMS],
}

impl Default for InitialStateData {
    fn default() -> Self {
        let open: [InitialDistributionViewSelection; MAX_NUM_STATE_DIMS] = [
            InitialDistributionViewSelection::default(),
            InitialDistributionViewSelection::default(),
            InitialDistributionViewSelection::default(),
            InitialDistributionViewSelection::new(
                InitialDistributionView::Fixed,
                InitialDistributionGroup::Deterministic,
            ),
        ];
        let initial_distributions: [InitialDistributionViewData; MAX_NUM_STATE_DIMS] =
            std::array::from_fn(|_| InitialDistributionViewData::default());
        Self {
            open_initial_distributions: open,
            all_initital_distributions: initial_distributions,
        }
    }
}

impl InitialStateData {
    fn open_selections(&self, num_state_dims: usize) -> &[InitialDistributionViewSelection] {
        self.open_initial_distributions.split_at(num_state_dims).0
    }
    fn initial_distributions(&self, num_state_dims: usize) -> InitialDistributionConfig {
        InitialDistributionConfig::States(
            (0..num_state_dims)
                .map(|i| {
                    generate_initital_distribution_variant(
                        &self.open_initial_distributions,
                        &self.all_initital_distributions,
                        i,
                    )
                })
                .collect(),
        )
    }
    fn selection_ui(&mut self, num_state_dims: usize, ui: &mut Ui) {
        for i in 0..num_state_dims {
            let label = format!("S{}", i + 1);
            let tooltip = format!(" Select initital distribution for state {} ", i + 1);
            let open = &mut self.open_initial_distributions[i];
            let all_initial_distributions = &mut self.all_initital_distributions[i];
            distribution_selection(
                open,
                all_initial_distributions,
                label.as_str(),
                true,
                ui,
                tooltip.as_str(),
            );
        }
    }
}

#[derive(PartialEq, Deserialize, Serialize)]
#[serde(default)]
struct InitialParticleData {
    open_initial_distributions_xy: [InitialDistributionViewSelection; DIMS_INIT_PARTICLEXY],

    all_initital_distributions_xy: [InitialDistributionViewData; DIMS_INIT_PARTICLEXY],

    open_initial_distributions_xyz: [InitialDistributionViewSelection; DIMS_INIT_PARTICLEXYZ],

    all_initital_distributions_xyz: [InitialDistributionViewData; DIMS_INIT_PARTICLEXYZ],
}

impl Default for InitialParticleData {
    fn default() -> Self {
        let open_xy: [InitialDistributionViewSelection; DIMS_INIT_PARTICLEXY] = [
            InitialDistributionViewSelection::new(
                InitialDistributionView::Normal,
                InitialDistributionGroup::Probabilistic,
            ), // "Parity  (collision)",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Normal,
                InitialDistributionGroup::Probabilistic,
            ), // "Mid-range  (charge)",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Exponential,
                InitialDistributionGroup::Probabilistic,
            ), // "Mass  (absolute)",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ), // "Position X",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ), // "Position Y",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Fixed,
                InitialDistributionGroup::Deterministic,
            ), // "Velocity X",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Fixed,
                InitialDistributionGroup::Deterministic,
            ), // "Velocity Y",
        ];
        let mut initial_distributions_xy: [InitialDistributionViewData; DIMS_INIT_PARTICLEXY] =
            std::array::from_fn(|_| InitialDistributionViewData::default());
        initial_distributions_xy[3].uniform.data = Uniform {
            low: -100.0,
            high: 100.0,
        };
        initial_distributions_xy[4].uniform.data = Uniform {
            low: -100.0,
            high: 100.0,
        };
        let open_xyz: [InitialDistributionViewSelection; DIMS_INIT_PARTICLEXYZ] = [
            InitialDistributionViewSelection::new(
                InitialDistributionView::Normal,
                InitialDistributionGroup::Probabilistic,
            ), // "Parity  (collision)",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Normal,
                InitialDistributionGroup::Probabilistic,
            ), // "Mid-range  (charge)",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Exponential,
                InitialDistributionGroup::Probabilistic,
            ), // "Mass  (absolute)",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ), // "Position X",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ), // "Position Y",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ), // "Position Z",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Fixed,
                InitialDistributionGroup::Deterministic,
            ), // "Velocity X",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Fixed,
                InitialDistributionGroup::Deterministic,
            ), // "Velocity Y",
            InitialDistributionViewSelection::new(
                InitialDistributionView::Fixed,
                InitialDistributionGroup::Deterministic,
            ), // "Velocity Z",
        ];
        let mut initial_distributions_xyz: [InitialDistributionViewData; DIMS_INIT_PARTICLEXYZ] =
            std::array::from_fn(|_| InitialDistributionViewData::default());
        initial_distributions_xyz[3].uniform.data = Uniform {
            low: -100.0,
            high: 100.0,
        };
        initial_distributions_xyz[4].uniform.data = Uniform {
            low: -100.0,
            high: 100.0,
        };
        initial_distributions_xyz[5].uniform.data = Uniform {
            low: -100.0,
            high: 100.0,
        };

        Self {
            open_initial_distributions_xy: open_xy,
            all_initital_distributions_xy: initial_distributions_xy,
            open_initial_distributions_xyz: open_xyz,
            all_initital_distributions_xyz: initial_distributions_xyz,
        }
    }
}

impl InitialParticleData {
    fn open_selections(&self, particle_mode: ParticleMode) -> &[InitialDistributionViewSelection] {
        match particle_mode {
            ParticleMode::XY => self.open_initial_distributions_xy.as_slice(),
            ParticleMode::XYZ => self.open_initial_distributions_xyz.as_slice(),
        }
    }
    fn initial_distributions(&self, particle_mode: ParticleMode) -> InitialDistributionConfig {
        match particle_mode {
            ParticleMode::XY => {
                let initial_distributions: [InitialDistributionVariant; DIMS_INIT_PARTICLEXY] =
                    std::array::from_fn(|i| {
                        generate_initital_distribution_variant(
                            &self.open_initial_distributions_xy,
                            &self.all_initital_distributions_xy,
                            i,
                        )
                    });
                InitialDistributionConfig::ParticleXY(initial_distributions)
            }
            ParticleMode::XYZ => {
                let initial_distributions: [InitialDistributionVariant; DIMS_INIT_PARTICLEXYZ] =
                    std::array::from_fn(|i| {
                        generate_initital_distribution_variant(
                            &self.open_initial_distributions_xyz,
                            &self.all_initital_distributions_xyz,
                            i,
                        )
                    });
                InitialDistributionConfig::ParticleXYZ(initial_distributions)
            }
        }
    }

    fn selection_ui(&mut self, particle_mode: ParticleMode, ui: &mut Ui) {
        let labels: Vec<&'static str> = particle_mode.labels().into();
        let meshable: Vec<bool> = particle_mode.meshable_dims().into();
        let tooltips: Vec<&'static str> = particle_mode.tooltips().into();
        labels
            .into_iter()
            .zip(meshable)
            .zip(tooltips)
            .enumerate()
            .for_each(|(i, ((label, show_space_distributions), tooltip))| {
                let (open, all_initial_distributions) = match particle_mode {
                    ParticleMode::XY => (
                        &mut self.open_initial_distributions_xy[i],
                        &mut self.all_initital_distributions_xy[i],
                    ),
                    ParticleMode::XYZ => (
                        &mut self.open_initial_distributions_xyz[i],
                        &mut self.all_initital_distributions_xyz[i],
                    ),
                };
                distribution_selection(
                    open,
                    all_initial_distributions,
                    label,
                    show_space_distributions,
                    ui,
                    tooltip,
                );
            });
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, Clone, Copy, Default, EnumIter, IntoStaticStr, Deserialize, Serialize)]
enum ParticleMode {
    #[default]
    XY,
    XYZ,
}

impl ParticleMode {
    const PARTICLE_2D_LABELS: [&'static str; DIMS_INIT_PARTICLEXY] = [
        LABEL_PARTICLE_PARITY,
        LABEL_PARTICLE_CHARGE,
        LABEL_PARTICLE_MASS,
        LABEL_PARTICLE_PX,
        LABEL_PARTICLE_PY,
        LABEL_PARTICLE_VX,
        LABEL_PARTICLE_VY,
    ];
    const PARTICLE_3D_LABELS: [&'static str; DIMS_INIT_PARTICLEXYZ] = [
        LABEL_PARTICLE_PARITY,
        LABEL_PARTICLE_CHARGE,
        LABEL_PARTICLE_MASS,
        LABEL_PARTICLE_PX,
        LABEL_PARTICLE_PY,
        LABEL_PARTICLE_PZ,
        LABEL_PARTICLE_VX,
        LABEL_PARTICLE_VY,
        LABEL_PARTICLE_VZ,
    ];
    const PARTICLE_2D_MESHABLE: [bool; DIMS_INIT_PARTICLEXY] =
        [false, false, false, true, true, false, false];
    const PARTICLE_3D_MESHABLE: [bool; DIMS_INIT_PARTICLEXYZ] =
        [false, false, false, true, true, true, false, false, false];
    const PARTICLE_2D_TOOLTIPS: [&'static str; DIMS_INIT_PARTICLEXY] = [
        TIP_PARTICLE_PARITY,
        TIP_PARTICLE_CHARGE,
        TIP_PARTICLE_MASS,
        TIP_PARTICLE_PX,
        TIP_PARTICLE_PY,
        TIP_PARTICLE_VX,
        TIP_PARTICLE_VY,
    ];
    const PARTICLE_3D_TOOLTIPS: [&'static str; DIMS_INIT_PARTICLEXYZ] = [
        TIP_PARTICLE_PARITY,
        TIP_PARTICLE_CHARGE,
        TIP_PARTICLE_MASS,
        TIP_PARTICLE_PX,
        TIP_PARTICLE_PY,
        TIP_PARTICLE_PZ,
        TIP_PARTICLE_VX,
        TIP_PARTICLE_VY,
        TIP_PARTICLE_VZ,
    ];
    fn dimensionality(&self) -> DistributionDimensions {
        match self {
            ParticleMode::XY => DIMS_PARTICLEXY,
            ParticleMode::XYZ => DIMS_PARTICLEXYZ,
        }
    }

    fn labels(&self) -> &[&'static str] {
        match self {
            ParticleMode::XY => &Self::PARTICLE_2D_LABELS,
            ParticleMode::XYZ => &Self::PARTICLE_3D_LABELS,
        }
    }

    fn meshable_dims(&self) -> &[bool] {
        match self {
            ParticleMode::XY => &Self::PARTICLE_2D_MESHABLE,
            ParticleMode::XYZ => &Self::PARTICLE_3D_MESHABLE,
        }
    }

    fn tooltips(&self) -> &[&'static str] {
        match self {
            ParticleMode::XY => &Self::PARTICLE_2D_TOOLTIPS,
            ParticleMode::XYZ => &Self::PARTICLE_3D_TOOLTIPS,
        }
    }
}

#[derive(PartialEq, Deserialize, Serialize)]
#[serde(default)]
struct InitialFractalData {
    open_initial_distributions_complex:
        [InitialDistributionViewSelection; DIMS_INIT_FRACTALCOMPLEX],

    all_initital_distributions_complex: [InitialDistributionViewData; DIMS_INIT_FRACTALCOMPLEX],

    open_initial_distributions_dual: [InitialDistributionViewSelection; DIMS_INIT_FRACTALDUAL],

    all_initital_distributions_dual: [InitialDistributionViewData; DIMS_INIT_FRACTALDUAL],

    open_initial_distributions_perplex:
        [InitialDistributionViewSelection; DIMS_INIT_FRACTALPERPLEX],

    all_initital_distributions_perplex: [InitialDistributionViewData; DIMS_INIT_FRACTALPERPLEX],

    open_initial_distributions_quaternion:
        [InitialDistributionViewSelection; DIMS_INIT_FRACTALQUATERNION],

    all_initital_distributions_quaternion:
        [InitialDistributionViewData; DIMS_INIT_FRACTALQUATERNION],
}

impl Default for InitialFractalData {
    fn default() -> Self {
        let open_complex: [InitialDistributionViewSelection; DIMS_INIT_FRACTALCOMPLEX] = [
            InitialDistributionViewSelection::new(
                InitialDistributionView::Mesh,
                InitialDistributionGroup::Mesh,
            ),
            InitialDistributionViewSelection::new(
                InitialDistributionView::Mesh,
                InitialDistributionGroup::Mesh,
            ),
        ];
        let initial_distributions_complex: [InitialDistributionViewData; DIMS_INIT_FRACTALCOMPLEX] =
            std::array::from_fn(|_| InitialDistributionViewData::default());
        let open_dual: [InitialDistributionViewSelection; DIMS_INIT_FRACTALDUAL] = [
            InitialDistributionViewSelection::new(
                InitialDistributionView::Mesh,
                InitialDistributionGroup::Mesh,
            ),
            InitialDistributionViewSelection::new(
                InitialDistributionView::Mesh,
                InitialDistributionGroup::Mesh,
            ),
        ];
        let initial_distributions_dual: [InitialDistributionViewData; DIMS_INIT_FRACTALDUAL] =
            std::array::from_fn(|_| InitialDistributionViewData::default());
        let open_perplex: [InitialDistributionViewSelection; DIMS_INIT_FRACTALPERPLEX] = [
            InitialDistributionViewSelection::new(
                InitialDistributionView::Mesh,
                InitialDistributionGroup::Mesh,
            ),
            InitialDistributionViewSelection::new(
                InitialDistributionView::Mesh,
                InitialDistributionGroup::Mesh,
            ),
        ];
        let initial_distributions_perplex: [InitialDistributionViewData; DIMS_INIT_FRACTALPERPLEX] =
            std::array::from_fn(|_| InitialDistributionViewData::default());
        let open_quaternion: [InitialDistributionViewSelection; DIMS_INIT_FRACTALQUATERNION] = [
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ),
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ),
            InitialDistributionViewSelection::new(
                InitialDistributionView::Uniform,
                InitialDistributionGroup::Probabilistic,
            ),
            InitialDistributionViewSelection::new(
                InitialDistributionView::Fixed,
                InitialDistributionGroup::Deterministic,
            ),
        ];
        let initial_distributions_quaternion: [InitialDistributionViewData;
            DIMS_INIT_FRACTALQUATERNION] =
            std::array::from_fn(|_| InitialDistributionViewData::default());
        Self {
            open_initial_distributions_complex: open_complex,
            all_initital_distributions_complex: initial_distributions_complex,
            open_initial_distributions_dual: open_dual,
            all_initital_distributions_dual: initial_distributions_dual,
            open_initial_distributions_perplex: open_perplex,
            all_initital_distributions_perplex: initial_distributions_perplex,
            open_initial_distributions_quaternion: open_quaternion,
            all_initital_distributions_quaternion: initial_distributions_quaternion,
        }
    }
}

impl InitialFractalData {
    fn open_selections(&self, fractal_mode: FractalMode) -> &[InitialDistributionViewSelection] {
        match fractal_mode {
            FractalMode::Complex => self.open_initial_distributions_complex.as_slice(),
            FractalMode::Dual => self.open_initial_distributions_dual.as_slice(),
            FractalMode::Perplex => self.open_initial_distributions_perplex.as_slice(),
            FractalMode::Quaternion => self.open_initial_distributions_quaternion.as_slice(),
        }
    }
    fn initial_distributions(&self, fractal_mode: FractalMode) -> InitialDistributionConfig {
        match fractal_mode {
            FractalMode::Complex => {
                let initial_distributions: [InitialDistributionVariant; DIMS_INIT_FRACTALCOMPLEX] =
                    std::array::from_fn(|i| {
                        generate_initital_distribution_variant(
                            &self.open_initial_distributions_complex,
                            &self.all_initital_distributions_complex,
                            i,
                        )
                    });
                InitialDistributionConfig::FractalComplex(initial_distributions)
            }
            FractalMode::Dual => {
                let initial_distributions: [InitialDistributionVariant; DIMS_INIT_FRACTALDUAL] =
                    std::array::from_fn(|i| {
                        generate_initital_distribution_variant(
                            &self.open_initial_distributions_dual,
                            &self.all_initital_distributions_dual,
                            i,
                        )
                    });
                InitialDistributionConfig::FractalDual(initial_distributions)
            }
            FractalMode::Perplex => {
                let initial_distributions: [InitialDistributionVariant; DIMS_INIT_FRACTALPERPLEX] =
                    std::array::from_fn(|i| {
                        generate_initital_distribution_variant(
                            &self.open_initial_distributions_perplex,
                            &self.all_initital_distributions_perplex,
                            i,
                        )
                    });
                InitialDistributionConfig::FractalPerplex(initial_distributions)
            }
            FractalMode::Quaternion => {
                let initial_distributions: [InitialDistributionVariant;
                    DIMS_INIT_FRACTALQUATERNION] = std::array::from_fn(|i| {
                    generate_initital_distribution_variant(
                        &self.open_initial_distributions_quaternion,
                        &self.all_initital_distributions_quaternion,
                        i,
                    )
                });
                InitialDistributionConfig::FractalQuaternion(initial_distributions)
            }
        }
    }

    fn selection_ui(&mut self, fractal_mode: FractalMode, ui: &mut Ui) {
        let labels: Vec<&'static str> = fractal_mode.labels().into();
        let tooltips: Vec<&'static str> = fractal_mode.tooltips().into();
        labels
            .into_iter()
            .zip(tooltips)
            .enumerate()
            .for_each(|(i, (label, tooltip))| {
                let (open, all_initial_distributions) = match fractal_mode {
                    FractalMode::Complex => (
                        &mut self.open_initial_distributions_complex[i],
                        &mut self.all_initital_distributions_complex[i],
                    ),
                    FractalMode::Dual => (
                        &mut self.open_initial_distributions_dual[i],
                        &mut self.all_initital_distributions_dual[i],
                    ),
                    FractalMode::Perplex => (
                        &mut self.open_initial_distributions_perplex[i],
                        &mut self.all_initital_distributions_perplex[i],
                    ),
                    FractalMode::Quaternion => (
                        &mut self.open_initial_distributions_quaternion[i],
                        &mut self.all_initital_distributions_quaternion[i],
                    ),
                };
                distribution_selection(open, all_initial_distributions, label, true, ui, tooltip);
            });
    }
}

#[derive(PartialEq, Clone, Copy, Default, EnumIter, IntoStaticStr, Deserialize, Serialize)]
enum FractalMode {
    #[default]
    Complex,
    Dual,
    Perplex,
    Quaternion,
}

impl FractalMode {
    const LABELS_COMPLEX: [&'static str; DIMS_INIT_FRACTALCOMPLEX] = ["a", "b"];
    const LABELS_DUAL: [&'static str; DIMS_INIT_FRACTALDUAL] = ["a", "b"];
    const LABELS_PERPLEX: [&'static str; DIMS_INIT_FRACTALPERPLEX] = ["t", "x"];
    const LABELS_QUATERNION: [&'static str; DIMS_INIT_FRACTALQUATERNION] = ["a", "b", "c", "d"];
    const TIPS_COMPLEX: [&'static str; DIMS_INIT_FRACTALCOMPLEX] =
        [TIP_FRACTAL_COMPLEX_RE, TIP_FRACTAL_COMPLEX_IM];
    const TIPS_DUAL: [&'static str; DIMS_INIT_FRACTALDUAL] =
        [TIP_FRACTAL_DUAL_RE, TIP_FRACTAL_DUAL_IM];
    const TIPS_PERPLEX: [&'static str; DIMS_INIT_FRACTALPERPLEX] =
        [TIP_FRACTAL_PERPLEX_RE, TIP_FRACTAL_PERPLEX_IM];
    const TIPS_QUATERNION: [&'static str; DIMS_INIT_FRACTALQUATERNION] = [
        TIP_FRACTAL_QUATERNION_RE,
        TIP_FRACTAL_QUATERNION_I,
        TIP_FRACTAL_QUATERNION_J,
        TIP_FRACTAL_QUATERNION_K,
    ];
    fn dimensionality(&self) -> DistributionDimensions {
        match self {
            FractalMode::Complex => DIMS_FRACTALCOMPLEX,
            FractalMode::Dual => DIMS_FRACTALDUAL,
            FractalMode::Perplex => DIMS_FRACTALPERPLEX,
            FractalMode::Quaternion => DIMS_FRACTALQUATERNION,
        }
    }
    fn tip_basic(&self) -> &'static str {
        match self {
            FractalMode::Complex => TIP_COMPLEX,
            FractalMode::Dual => TIP_DUAL,
            FractalMode::Perplex => TIP_PERPLEX,
            FractalMode::Quaternion => TIP_QUATERNION,
        }
    }
    fn reference(&self) -> &'static str {
        match self {
            FractalMode::Complex => LINK_COMPLEX,
            FractalMode::Dual => LINK_DUAL,
            FractalMode::Perplex => LINK_PERPLEX,
            FractalMode::Quaternion => LINK_QUATERNION,
        }
    }
    fn element_basis(&self) -> &'static str {
        match self {
            FractalMode::Complex => LABEL_BASIS_COMPLEX,
            FractalMode::Dual => LABEL_BASIS_DUAL,
            FractalMode::Perplex => LABEL_BASIS_PERPLEX,
            FractalMode::Quaternion => LABEL_BASIS_QUATERNION,
        }
    }
    fn labels(&self) -> &[&'static str] {
        match self {
            FractalMode::Complex => &Self::LABELS_COMPLEX,
            FractalMode::Dual => &Self::LABELS_DUAL,
            FractalMode::Perplex => &Self::LABELS_PERPLEX,
            FractalMode::Quaternion => &Self::LABELS_QUATERNION,
        }
    }
    fn tooltips(&self) -> &[&'static str] {
        match self {
            FractalMode::Complex => &Self::TIPS_COMPLEX,
            FractalMode::Dual => &Self::TIPS_DUAL,
            FractalMode::Perplex => &Self::TIPS_PERPLEX,
            FractalMode::Quaternion => &Self::TIPS_QUATERNION,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Default, EnumIter, Deserialize, Serialize)]
enum InitialDistributionGroup {
    #[default]
    Probabilistic,
    Mesh,
    Deterministic,
}

impl From<InitialDistributionGroup> for &'static str {
    fn from(val: InitialDistributionGroup) -> Self {
        match val {
            InitialDistributionGroup::Probabilistic => "🎲",
            InitialDistributionGroup::Deterministic => "🎯",
            InitialDistributionGroup::Mesh => "▦",
        }
    }
}

const DISTRIBUTIONS_ALL: [InitialDistributionGroup; 3] = [
    InitialDistributionGroup::Probabilistic,
    InitialDistributionGroup::Mesh,
    InitialDistributionGroup::Deterministic,
];

const DISTRIBUTIONS_NO_MESH: [InitialDistributionGroup; 2] = [
    InitialDistributionGroup::Probabilistic,
    InitialDistributionGroup::Deterministic,
];

#[derive(PartialEq, Default, Copy, Clone, Deserialize, Serialize)]
#[serde(default)]
struct InitialDistributionViewSelection {
    pub view: InitialDistributionView,
    pub group: InitialDistributionGroup,
}

impl InitialDistributionViewSelection {
    fn new(view: InitialDistributionView, group: InitialDistributionGroup) -> Self {
        Self { view, group }
    }
    fn view_mut(&mut self) -> &mut InitialDistributionView {
        &mut self.view
    }
    fn group_mut(&mut self) -> &mut InitialDistributionGroup {
        &mut self.group
    }
}
#[derive(PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct InitialPanel {
    num_samples: usize,
    init_mode: InitialMode,
    num_state_dims: usize,
    particle_mode: ParticleMode,
    fractal_mode: FractalMode,
    #[cfg_attr(target_arch = "wasm32", serde(skip))] // TODO causes wasm memory bug, works native
    states: InitialStateData,
    #[cfg_attr(target_arch = "wasm32", serde(skip))] // TODO causes wasm memory bug, works native
    particles: InitialParticleData,
    #[cfg_attr(target_arch = "wasm32", serde(skip))] // TODO causes wasm memory bug, works native
    fractals: InitialFractalData,
}
impl Default for InitialPanel {
    fn default() -> Self {
        Self {
            num_samples: 100,
            init_mode: Default::default(),
            num_state_dims: 2,
            particle_mode: Default::default(),
            fractal_mode: Default::default(),
            states: Default::default(),
            particles: Default::default(),
            fractals: Default::default(),
        }
    }
}

impl InitialPanel {
    pub fn number_of_samples(&self) -> usize {
        self.num_samples
    }

    pub fn initial_distributions(&self) -> InitialDistributionConfig {
        match self.init_mode {
            InitialMode::States => self.states.initial_distributions(self.num_state_dims()),
            InitialMode::Particle => self.particles.initial_distributions(self.particle_mode),
            InitialMode::Fractals => self.fractals.initial_distributions(self.fractal_mode),
        }
    }

    fn num_state_dims(&self) -> usize {
        self.num_state_dims
    }

    fn count_open_meshes(&self) -> usize {
        let open_selections: &[InitialDistributionViewSelection] = match self.init_mode {
            InitialMode::States => self.states.open_selections(self.num_state_dims()),
            InitialMode::Particle => self.particles.open_selections(self.particle_mode),
            InitialMode::Fractals => self.fractals.open_selections(self.fractal_mode),
        };
        let mut ct = 0;
        for open in open_selections {
            if INITIAL_MESHES.contains(&open.view) {
                ct += 1;
            }
        }
        ct
    }

    fn max_num_samples(&self) -> usize {
        let mesh_control = match self.count_open_meshes() {
            0 | 1 => 1,
            2 => 10,
            3 => 100,
            4 => 1000,
            _ => {
                return 10;
            }
        };
        let mode_num_samples = match self.init_mode {
            InitialMode::States => 10_000 - self.num_state_dims() * 1000,
            InitialMode::Particle => 4_000,
            InitialMode::Fractals => 10_000,
        };
        mode_num_samples / mesh_control
    }

    pub fn dimensionality(&self) -> DistributionDimensions {
        match self.init_mode {
            InitialMode::States => DistributionDimensions::State(self.num_state_dims()),
            InitialMode::Particle => self.particle_mode.dimensionality(),
            InitialMode::Fractals => self.fractal_mode.dimensionality(),
        }
    }

    fn num_states_selection_ui(&mut self, ui: &mut Ui) {
        let minus_button_activated = self.num_state_dims > 1;
        if clickable_button(
            "-",
            false,
            minus_button_activated,
            ui,
            TIP_BUTTON_DECREASE_NUM_STATES,
        ) {
            self.num_state_dims -= 1;
        }
        add_label(
            format!("Dims: {}", self.num_state_dims).as_str(),
            ui,
            TIP_DIMS,
        );
        let plus_button_activated = self.num_state_dims < MAX_NUM_STATE_DIMS;
        if clickable_button(
            "+",
            false,
            plus_button_activated,
            ui,
            TIP_BUTTON_INCREASE_NUM_STATES,
        ) {
            self.num_state_dims += 1;
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        group_horizontal(ui, |ui| {
            let _ = combo_box(LABEL_INIT_MODE, &mut self.init_mode, ui, TIP_INIT_MODE);
        });
        group_horizontal(ui, |ui| {
            let max_num_samples = self.max_num_samples();
            integer_slider(
                LABEL_NUM_SAMPLES,
                &mut self.num_samples,
                max_num_samples,
                ui,
                TIP_NUM_SAMPLES,
            );
        });

        match self.init_mode {
            InitialMode::States => {
                group_horizontal(ui, |ui| {
                    self.num_states_selection_ui(ui);
                });
                self.states_selection_ui(ui);
            }
            InitialMode::Particle => {
                ui.horizontal(|ui| {
                    let _ = select_group(&mut self.particle_mode, ui, TIP_PARTICLE_MODE);
                });
                self.particle_selection_ui(ui);
            }
            InitialMode::Fractals => {
                ui.horizontal(|ui| {
                    let _ = select_group(&mut self.fractal_mode, ui, TIP_FRACTAL_MODE);
                });
                self.fractal_selection_ui(ui);
            }
        };
    }

    fn states_selection_ui(&mut self, ui: &mut Ui) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.vertical(|ui| {
                self.states.selection_ui(self.num_state_dims(), ui);
            });
        });
    }

    fn particle_selection_ui(&mut self, ui: &mut Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.vertical(|ui| {
                self.particles.selection_ui(self.particle_mode, ui);
            });
        });
    }
    fn fractal_selection_ui(&mut self, ui: &mut Ui) {
        group_horizontal(ui, |ui| {
            ui.heading(self.fractal_mode.element_basis());
            add_hyperlink(
                "Info",
                self.fractal_mode.reference(),
                ui,
                self.fractal_mode.tip_basic(),
            );
        });
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.vertical(|ui| {
                self.fractals.selection_ui(self.fractal_mode, ui);
            });
        });
    }
}
