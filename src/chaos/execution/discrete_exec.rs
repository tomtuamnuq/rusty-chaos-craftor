use crate::chaos::{
    data::{ChaosData, FromStateVec, InitialDistributionVariant, Time, ValidStateCheck},
    functions::DiscreteMap,
};

pub struct DiscreteVecExecutor<V, D>
where
    V: FromStateVec + ValidStateCheck + Clone,
    D: DiscreteMap<State = V> + Clone,
{
    pairs: Vec<(ChaosData<V>, D)>,
}

impl<V, D> DiscreteVecExecutor<V, D>
where
    V: FromStateVec + ValidStateCheck + Clone,
    D: DiscreteMap<State = V> + Clone,
{
    pub fn reinit_states_vec(&mut self, distributions: &[InitialDistributionVariant]) {
        self.pairs.iter_mut().for_each(|(data, _)| {
            data.reinit_states(distributions);
        });
    }

    pub fn execute_vec(&mut self, num_executions: usize, t0: &Time) {
        let timesteps: Vec<Time> = (0..num_executions).map(|i| t0 + (i as Time)).collect();
        self.pairs.iter_mut().for_each(|(data, map)| {
            data.data_mut().iter_mut().for_each(|state| {
                if let Some(y) = state.as_mut() {
                    for t in &timesteps {
                        map.execute(y, t);
                        if !y.is_valid() {
                            *state = None;
                            break;
                        }
                    }
                }
            });
        });
    }

    pub fn get_chaos_data_refs(&self) -> Vec<&ChaosData<V>> {
        self.pairs.iter().map(|(data, _)| data).collect()
    }

    pub fn new(chaos_data: Vec<&ChaosData<V>>, maps: &[D]) -> Self {
        let pairs = chaos_data
            .into_iter()
            .cycle()
            .zip(maps.iter().cloned())
            .map(|(data, map)| (data.clone(), map))
            .collect();

        Self { pairs }
    }

    pub fn new_single(data: &ChaosData<V>, maps: &[D]) -> Self {
        let pairs = maps
            .iter()
            .cloned()
            .map(|map| (data.clone(), map))
            .collect();

        Self { pairs }
    }
}
