use crate::chaos::{
    data::*,
    fractal::*,
    particle::{ParticleXY, ParticleXYZ},
};
use anyhow::bail;
use anyhow::Error;
use paste::paste;

macro_rules! create_and_impl_data_variants {
    ($($variant:ident),*) => {
        paste!{
            pub enum ChaosDataVariant {
                $(
                    $variant(ChaosData<$variant>),
                )*
            }

            pub enum ChaosDataVec<'a> {
                $(
                    $variant(Vec<&'a ChaosData<$variant>>),
                )*
            }

            impl<'a> ChaosDataVec<'a> {
                pub fn from_chaos_data(chaos_data_variant: &'a ChaosDataVariant) -> Self {
                    match chaos_data_variant {
                        $(
                            ChaosDataVariant::$variant(chaos_data) => ChaosDataVec::$variant(vec![chaos_data]),
                        )*
                    }
                }
                pub fn dimensionality(&self) -> DistributionDimensions {
                    match self {
                        $(
                            ChaosDataVec::$variant(_) => [<DIMS_ $variant:upper>],
                        )*
                    }
                }
            }
        }
    };
}

create_and_impl_data_variants! {
    State1, State2, State3, State4, ParticleXY, ParticleXYZ, FractalComplex, FractalDual, FractalPerplex, FractalQuaternion
}

#[derive(Clone)]
pub enum InitialDistributionConfig {
    States(Vec<InitialDistributionVariant>),
    ParticleXY([InitialDistributionVariant; DIMS_INIT_PARTICLEXY]),
    ParticleXYZ([InitialDistributionVariant; DIMS_INIT_PARTICLEXYZ]),
    FractalComplex([InitialDistributionVariant; DIMS_INIT_FRACTALCOMPLEX]),
    FractalDual([InitialDistributionVariant; DIMS_INIT_FRACTALDUAL]),
    FractalPerplex([InitialDistributionVariant; DIMS_INIT_FRACTALPERPLEX]),
    FractalQuaternion([InitialDistributionVariant; DIMS_INIT_FRACTALQUATERNION]),
}

impl InitialDistributionConfig {
    pub fn dimensionality(&self) -> DistributionDimensions {
        match self {
            Self::States(v) => DistributionDimensions::State(v.len()),
            Self::ParticleXY(_) => DIMS_PARTICLEXY,
            Self::ParticleXYZ(_) => DIMS_PARTICLEXYZ,
            Self::FractalComplex(_) => DIMS_FRACTALCOMPLEX,
            Self::FractalDual(_) => DIMS_FRACTALDUAL,
            Self::FractalPerplex(_) => DIMS_FRACTALPERPLEX,
            Self::FractalQuaternion(_) => DIMS_FRACTALQUATERNION,
        }
    }
}

impl Default for InitialDistributionConfig {
    fn default() -> Self {
        Self::States(Vec::new())
    }
}

impl From<&InitialDistributionConfig> for String {
    fn from(init_distr: &InitialDistributionConfig) -> Self {
        match init_distr {
            InitialDistributionConfig::States(vars) => {
                if vars.is_empty() {
                    "empty".to_string()
                } else {
                    let distr_labels: Vec<String> = vars.iter().map(String::from).collect();
                    distr_labels.join(" ")
                }
            }
            InitialDistributionConfig::ParticleXY(_) => "Particles 3D".into(),
            InitialDistributionConfig::ParticleXYZ(_) => "Particles 2D".into(),
            InitialDistributionConfig::FractalComplex(_) => "Fractal Complex".into(),
            InitialDistributionConfig::FractalDual(_) => "Fractal Dual Numbers".into(),
            InitialDistributionConfig::FractalPerplex(_) => "Fractal Perplex Numbers".into(),
            InitialDistributionConfig::FractalQuaternion(_) => "Fractal Quaternion".into(),
        }
    }
}

impl ChaosDataVariant {
    pub fn generate_initial_chaos_data(
        num_samples: usize,
        config: &InitialDistributionConfig,
    ) -> Result<Self, Error> {
        match config {
            InitialDistributionConfig::States(init_distr) => match init_distr.len() {
                1 => Ok(ChaosDataVariant::State1(ChaosData::new(
                    num_samples,
                    init_distr,
                ))),
                2 => Ok(ChaosDataVariant::State2(ChaosData::new(
                    num_samples,
                    init_distr,
                ))),
                3 => Ok(ChaosDataVariant::State3(ChaosData::new(
                    num_samples,
                    init_distr,
                ))),
                4 => Ok(ChaosDataVariant::State4(ChaosData::new(
                    num_samples,
                    init_distr,
                ))),
                _ => {
                    bail!("More than 4 dimensions are not implemented!");
                }
            },
            InitialDistributionConfig::ParticleXY(init_distr) => Ok(ChaosDataVariant::ParticleXY(
                ChaosData::new(num_samples, init_distr),
            )),

            InitialDistributionConfig::ParticleXYZ(init_distr) => Ok(
                ChaosDataVariant::ParticleXYZ(ChaosData::new(num_samples, init_distr)),
            ),
            InitialDistributionConfig::FractalComplex(init_distr) => Ok(
                ChaosDataVariant::FractalComplex(ChaosData::new(num_samples, init_distr)),
            ),
            InitialDistributionConfig::FractalDual(init_distr) => Ok(
                ChaosDataVariant::FractalDual(ChaosData::new(num_samples, init_distr)),
            ),
            InitialDistributionConfig::FractalPerplex(init_distr) => Ok(
                ChaosDataVariant::FractalPerplex(ChaosData::new(num_samples, init_distr)),
            ),
            InitialDistributionConfig::FractalQuaternion(init_distr) => Ok(
                ChaosDataVariant::FractalQuaternion(ChaosData::new(num_samples, init_distr)),
            ),
        }
    }
}
