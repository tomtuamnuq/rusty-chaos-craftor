use crate::chaos::{
    data::{ChaosData, FromStateVec, InitialDistributionVariant, Time, ValidStateCheck},
    functions::OdeSolverTrait,
};

pub struct ContinuousVecExecutor<V, O>
where
    V: FromStateVec + ValidStateCheck + Clone,
    O: OdeSolverTrait<State = V> + Clone,
{
    pairs: Vec<(ChaosData<V>, O)>,
}

impl<V, O> ContinuousVecExecutor<V, O>
where
    V: FromStateVec + ValidStateCheck + Clone,
    O: OdeSolverTrait<State = V> + Clone,
{
    pub fn reinit_states_vec(&mut self, distributions: &[InitialDistributionVariant]) {
        self.pairs.iter_mut().for_each(|(data, ode_solver)| {
            let new_state_indices = data.reinit_states(distributions);
            ode_solver.reinit_states(data.data_mut(), new_state_indices);
        });
    }

    pub fn execute_vec(&mut self, num_executions: usize, _t0: &Time) {
        self.pairs
            .iter_mut()
            .for_each(|(data, ode_solver)| ode_solver.execute(data.data_mut(), num_executions));
    }

    pub fn new(chaos_data: Vec<&ChaosData<V>>, ode_solver_vec: &[O]) -> Self {
        let pairs = chaos_data
            .into_iter()
            .cycle()
            .zip(ode_solver_vec.iter().cloned())
            .map(|(data, mut ode_solver)| {
                let mut data_clone = data.clone();
                ode_solver.initial_states(data_clone.data_mut());
                (data_clone, ode_solver)
            })
            .collect();

        Self { pairs }
    }

    pub fn new_single(data: &ChaosData<V>, ode_solver_vec: &[O]) -> Self {
        let pairs = ode_solver_vec
            .iter()
            .cloned()
            .map(|mut ode_solver| {
                let mut data_clone = data.clone();
                ode_solver.initial_states(data_clone.data_mut());
                (data_clone, ode_solver)
            })
            .collect();

        Self { pairs }
    }

    pub fn get_chaos_data_refs(&self) -> Vec<&ChaosData<V>> {
        self.pairs.iter().map(|(data, _)| data).collect()
    }
}
