use rand::{thread_rng, Rng};

use super::initial_distribution::{hyper_mesh_grid, Features, InitialDistributionVariant};
use super::{chaos_states::*, initial_distribution, InitFeatures};

#[derive(Debug, PartialEq, Clone)]
pub struct ChaosData<V> {
    data: Vec<Option<V>>,
}

impl<V> ChaosData<V> {
    pub fn total_num_points(&self) -> usize {
        self.data.len()
    }
    pub fn data(&self) -> &Vec<Option<V>> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<Option<V>> {
        &mut self.data
    }
    pub fn data_filtered(&self) -> Vec<&V> {
        self.data()
            .iter()
            .filter_map(|state| state.as_ref())
            .collect()
    }
}
impl<V: FromStateVec> ChaosData<V> {
    pub fn reinit_states(&mut self, distributions: &[InitialDistributionVariant]) -> Vec<usize> {
        let mut new_state_indices = Vec::new();
        self.data.iter().enumerate().for_each(|(i, state)| {
            if state.is_none() {
                new_state_indices.push(i);
            }
        });
        let num_new_states = new_state_indices.len();
        if num_new_states > 0 {
            let distributions_unmeshed: Vec<InitialDistributionVariant> = distributions
                .iter()
                .map(|init_distr| init_distr.random_space_from_mesh())
                .collect();
            let new_data = Self::states_from_distributions(num_new_states, &distributions_unmeshed);
            new_data
                .into_iter()
                .zip(new_state_indices.iter())
                .for_each(|(new_state, i)| self.data[*i] = new_state);
        }
        new_state_indices
    }

    fn init_meshes<R: Rng>(
        num_init_points: usize,
        distributions: &[InitialDistributionVariant],
        rng: &mut R,
    ) -> (Features, usize) {
        let meshes: InitFeatures = distributions
            .iter()
            .filter_map(|distr| {
                if distr.is_mesh() {
                    let space = distr.space_from_mesh();
                    Some(space.data_generation(num_init_points, rng))
                } else {
                    None
                }
            })
            .collect();
        let meshes = hyper_mesh_grid(meshes);
        let num_points = if meshes.is_empty() {
            num_init_points
        } else {
            num_init_points.pow(meshes.len() as u32)
        };
        (meshes, num_points)
    }
    fn states_from_features(features: Features) -> Vec<Option<V>> {
        if features.is_empty() {
            return Vec::new();
        }
        let num_states = features[0].len();
        let mut iters: Vec<_> = features.into_iter().map(|feat| feat.into_iter()).collect();
        (0..num_states)
            .map(|_| {
                let state = iters
                    .iter_mut()
                    .map(|state_i| state_i.next().unwrap())
                    .collect::<InitState>();
                Some(V::from(state))
            })
            .collect()
    }
    fn states_from_distributions(
        num_init_points: usize,
        distributions: &[InitialDistributionVariant],
    ) -> Vec<Option<V>> {
        let mut rng = thread_rng();
        let (meshes, num_points) = Self::init_meshes(num_init_points, distributions, &mut rng);
        let mut meshes = meshes.into_iter();
        let features = distributions
            .iter()
            .enumerate()
            .map(|(n, distr)| {
                match distr {
                    // handle distributions where the feature (column) depend on the index separately
                    InitialDistributionVariant::Eye(d) => {
                        initial_distribution::eye(num_points, d, n)
                    }
                    // use the previously created mash columns
                    InitialDistributionVariant::Mesh(_) => meshes
                        .next()
                        .expect("Same number of mesh arrays were created!"),
                    _ => distr.data_generation(num_points, &mut rng),
                }
            })
            .collect();
        Self::states_from_features(features)
    }

    pub fn new(num_points: usize, distributions: &[InitialDistributionVariant]) -> Self {
        Self {
            data: Self::states_from_distributions(num_points, distributions),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chaos::data::*;

    #[test]
    fn test_initial_distributions() {
        let num_points = 3;
        let distr = vec![
            InitialDistributionVariant::default(),
            InitialDistributionVariant::default(),
        ];
        let chaos_data = ChaosData::<State2>::new(num_points, &distr);
        assert_eq!(chaos_data.data().len(), num_points);
        let distr = vec![
            InitialDistributionVariant::Cauchy(Cauchy {
                median: 1.0,
                scale: 1.0,
            }),
            InitialDistributionVariant::Cauchy(Cauchy {
                median: 1.0,
                scale: 1.0,
            }),
        ];
        let chaos_data = ChaosData::<State2>::new(num_points, &distr);
        assert_eq!(chaos_data.data().len(), num_points);
        let distr = vec![
            InitialDistributionVariant::Uniform(Uniform {
                low: 1.0,
                high: 5.0,
            }),
            InitialDistributionVariant::Uniform(Uniform {
                low: 1.0,
                high: 5.0,
            }),
        ];
        let chaos_data = ChaosData::<State2>::new(num_points, &distr);
        assert_eq!(chaos_data.data().len(), num_points);
        let distr = vec![
            InitialDistributionVariant::Fixed(Fixed { value: 1.0 }),
            InitialDistributionVariant::Fixed(Fixed { value: 2.0 }),
        ];
        let chaos_data = ChaosData::<State2>::new(num_points, &distr);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State2::new(1.0, 2.0)),
                Some(State2::new(1.0, 2.0)),
                Some(State2::new(1.0, 2.0))
            ]
        );
        let distr = vec![
            InitialDistributionVariant::Linspace(Linspace {
                low: 0.0,
                high: 1.0,
            }),
            InitialDistributionVariant::Linspace(Linspace {
                low: 0.0,
                high: 1.0,
            }),
        ];
        let chaos_data = ChaosData::<State2>::new(num_points, &distr);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State2::new(0.0, 0.0)),
                Some(State2::new(0.5, 0.5)),
                Some(State2::new(1.0, 1.0))
            ]
        );
    }

    #[test]
    fn test_chaos_data_from_distributions() {
        let distr_fixed = InitialDistributionVariant::Fixed(Fixed { value: 2.0 });
        let distr_lin = InitialDistributionVariant::Linspace(Linspace {
            low: 0.0,
            high: 1.0,
        });
        let distributions = vec![distr_fixed, distr_lin];
        let chaos_data = ChaosData::<State2>::new(3, &distributions);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State2::new(2.0, 0.0)),
                Some(State2::new(2.0, 0.5)),
                Some(State2::new(2.0, 1.0))
            ]
        );
    }

    #[test]
    fn test_mesh_distribution() {
        let num_points = 4;
        let distr = vec![InitialDistributionVariant::Mesh(Mesh {
            start: 0.0,
            end: 1.0,
        })];
        let chaos_data = ChaosData::<State1>::new(num_points, &distr);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State1::new(0.0)),
                Some(State1::new(1.0 / 3.0)),
                Some(State1::new(2.0 / 3.0)),
                Some(State1::new(1.0))
            ]
        );
    }

    #[test]
    fn test_geomspace_distribution() {
        let num_points = 3;
        let distr = vec![InitialDistributionVariant::Geomspace(Geomspace {
            start: 1.0,
            end: 16.0,
        })];
        let chaos_data = ChaosData::<State1>::new(num_points, &distr);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State1::new(1.0)),
                Some(State1::new(4.0)),
                Some(State1::new(15.999999999999998))
            ]
        );
    }

    #[test]
    fn test_eye_distribution() {
        let num_points = 3;
        let distr = vec![
            InitialDistributionVariant::Eye(Eye { value: 1.0 }),
            InitialDistributionVariant::Eye(Eye { value: 2.0 }),
            InitialDistributionVariant::Eye(Eye { value: 3.0 }),
        ];
        let chaos_data = ChaosData::<State3>::new(num_points, &distr);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State3::new(1.0, 0.0, 0.0)),
                Some(State3::new(0.0, 2.0, 0.0)),
                Some(State3::new(0.0, 0.0, 3.0))
            ]
        );
    }

    #[test]
    fn test_logspace_distribution() {
        let num_points = 3;
        let distr = vec![InitialDistributionVariant::Logspace(Logspace {
            start: 0.0,
            end: 3.0,
            base: 2.0,
        })];
        let chaos_data = ChaosData::<State1>::new(num_points, &distr);
        assert_eq!(
            chaos_data.data(),
            &[
                Some(State1::new(1.0)),
                Some(State1::new(2.8284271247461903)),
                Some(State1::new(8.0))
            ]
        );
    }
}
