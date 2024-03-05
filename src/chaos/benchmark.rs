use std::{
    sync::{Arc, Mutex},
    thread,
};
// TODO use wasm threads
use super::{
    data::{ChaosData, ChaosDataVec, InitialDistributionConfig, ValidStateCheck},
    execution::*,
};
use anyhow::Error;
use web_time::{Duration, Instant};
pub struct ChaosInitSchema {
    pub num_samples: usize,
    pub num_executions: usize,
    pub init_distr: InitialDistributionConfig,
    pub discrete_map_vec: Option<DiscreteMapVec>,
    pub diff_system_vec: Option<OdeSystemSolverVec>,
    pub pars: (&'static str, Vec<f64>),
}

pub struct ChaosBenchmarkRun {
    runtime: Duration,
    num_valid_end_states: usize,
}

impl ChaosBenchmarkRun {
    pub fn runtime_millis(&self) -> usize {
        self.runtime.as_millis() as usize
    }

    pub fn runtime_nanos(&self) -> u128 {
        self.runtime.as_nanos()
    }

    pub fn num_valid_end_states(&self) -> usize {
        self.num_valid_end_states
    }
}

pub struct ChaosBenchmarkResult {
    num_warmups: usize,
    runs: Vec<Result<ChaosBenchmarkRun, Error>>, // contains warm-up runs as well
}

impl ChaosBenchmarkResult {
    pub fn runs(&self) -> &Vec<Result<ChaosBenchmarkRun, Error>> {
        &self.runs
    }
    pub fn number_of_warmups(&self) -> usize {
        self.num_warmups
    }
}

impl Default for ChaosBenchmarkResult {
    fn default() -> Self {
        let error_run = Err(Error::msg("Default init without runs!"));
        Self {
            num_warmups: 0,
            runs: vec![error_run],
        }
    }
}

impl Default for ChaosInitSchema {
    fn default() -> Self {
        Self {
            num_samples: 0,
            num_executions: 0,
            init_distr: InitialDistributionConfig::States(Vec::new()),
            discrete_map_vec: None,
            diff_system_vec: None,
            pars: Default::default(),
        }
    }
}

pub fn chaos_benchmark(
    chaos_init: &ChaosInitSchema,
    num_iterations: usize,
    num_warmups: usize,
) -> ChaosBenchmarkResult {
    #[cfg(not(target_arch = "wasm32"))]
    let threaded = true;
    #[cfg(target_arch = "wasm32")]
    let threaded = false;
    let benchmark_run_cb: fn(&ChaosInitSchema) -> Result<ChaosBenchmarkRun, Error> = if threaded {
        chaos_benchmark_threaded
    } else {
        chaos_benchmark_single
    };
    let total_num_runs = num_warmups + num_iterations;
    let runs = (0..total_num_runs)
        .map(|_| benchmark_run_cb(chaos_init))
        .collect();
    ChaosBenchmarkResult { num_warmups, runs }
}

fn chaos_benchmark_single(chaos_init: &ChaosInitSchema) -> Result<ChaosBenchmarkRun, Error> {
    let ChaosInitSchema {
        num_samples,
        num_executions,
        init_distr,
        discrete_map_vec,
        diff_system_vec,
        ..
    } = chaos_init;
    chaos_benchmark_run(
        *num_samples,
        *num_executions,
        init_distr.clone(),
        discrete_map_vec.clone(),
        diff_system_vec.clone(),
    )
}

fn chaos_benchmark_run(
    num_samples: usize,
    num_executions: usize,
    init_distr: InitialDistributionConfig,
    discrete_map_vec: Option<DiscreteMapVec>,
    diff_system_vec: Option<OdeSystemSolverVec>,
) -> Result<ChaosBenchmarkRun, Error> {
    let begin = Instant::now();
    let mut controller = ChaosExecutionController::default();
    controller.generate_initial_chaos_data(num_samples, init_distr)?;
    if let Some(discrete_map_vec) = discrete_map_vec {
        controller.set_discrete_mappers(discrete_map_vec)?;
    } else if let Some(diff_system_vec) = diff_system_vec {
        controller.set_differential_solvers(diff_system_vec)?;
    };
    controller.execute(num_executions)?;
    let elapsed = begin.elapsed();
    let chaos_data_vec = controller.get_chaos_data()?;
    let num_valid_states = match chaos_data_vec {
        ChaosDataVec::State1(data) => evaluate_chaos_data(data),
        ChaosDataVec::State2(data) => evaluate_chaos_data(data),
        ChaosDataVec::State3(data) => evaluate_chaos_data(data),
        ChaosDataVec::State4(data) => evaluate_chaos_data(data),
        ChaosDataVec::ParticleXY(data) => evaluate_chaos_data(data),
        ChaosDataVec::ParticleXYZ(data) => evaluate_chaos_data(data),
        ChaosDataVec::FractalComplex(data) => evaluate_chaos_data(data), // TODO specific for fractal - num iterations
        ChaosDataVec::FractalDual(data) => evaluate_chaos_data(data), // TODO specific for fractal - num iterations
        ChaosDataVec::FractalPerplex(data) => evaluate_chaos_data(data), // TODO specific for fractal - num iterations
        ChaosDataVec::FractalQuaternion(data) => evaluate_chaos_data(data), // TODO specific for fractal - num iterations
    };
    Ok(ChaosBenchmarkRun {
        runtime: elapsed,
        num_valid_end_states: num_valid_states,
    })
}

fn evaluate_chaos_data<V: ValidStateCheck>(data: Vec<&ChaosData<V>>) -> usize {
    data.iter()
        .map(|chaos_data| chaos_data.data_filtered().len())
        .sum()
}

fn chaos_benchmark_threaded(chaos_init: &ChaosInitSchema) -> Result<ChaosBenchmarkRun, Error> {
    let ChaosInitSchema {
        num_samples,
        num_executions,
        init_distr,
        discrete_map_vec,
        diff_system_vec,
        ..
    } = chaos_init;
    let begin = Instant::now();
    #[cfg(not(target_arch = "wasm32"))]
    let num_threads = num_cpus::get();
    #[cfg(target_arch = "wasm32")]
    let num_threads = 1; // TODO wasm threads
    log::debug!("Starting benchmark with {:?} threads.", num_threads);
    let (num_samples_per_thread, num_remaining_samples) = (
        num_samples.div_euclid(num_threads),
        num_samples.rem_euclid(num_threads),
    );
    log::debug!("Each thread creates {:?} samples.", num_samples_per_thread);
    let num_valid_states = Arc::new(Mutex::new(Vec::with_capacity(num_threads)));
    thread::scope(|s| {
        for i in 0..num_threads {
            let num_valid_states = Arc::clone(&num_valid_states);
            let num_samples_this_thread = if i == 0 {
                num_samples_per_thread + num_remaining_samples
            } else {
                num_samples_per_thread
            };
            s.spawn(move || {
                log::debug!("Thread {:?} started.", i);
                let begin_thread = Instant::now();
                let result = chaos_benchmark_run(
                    num_samples_this_thread,
                    *num_executions,
                    init_distr.clone(),
                    discrete_map_vec.clone(),
                    diff_system_vec.clone(),
                );
                if let Ok(mut num_valid_states) = num_valid_states.lock() {
                    num_valid_states.push(result);
                }
                log::debug!(
                    "Thread {:?} finished after {:?} ms.",
                    i,
                    begin_thread.elapsed().as_millis()
                );
            });
        }
    });
    let elapsed = begin.elapsed();
    let mut total_num_valid_end_states = 0;
    if let Ok(num_valid_states) = num_valid_states.lock() {
        for res in num_valid_states.iter() {
            match res {
                Ok(res) => {
                    total_num_valid_end_states += res.num_valid_end_states();
                }
                Err(e) => {
                    return Err(Error::msg(format!("A thread caused an error: {e}")));
                }
            }
        }
    };
    Ok(ChaosBenchmarkRun {
        runtime: elapsed,
        num_valid_end_states: total_num_valid_end_states,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chaos::{
        data::InitialDistributionVariant,
        functions::{Logistic, SimpleDiscreteMap},
    };
    #[test]
    fn test_chaos_benchmark() {
        let num_samples = 11;
        let map = SimpleDiscreteMap::new(Logistic::default());
        let chaos_init = ChaosInitSchema {
            num_samples,
            num_executions: 2,
            init_distr: InitialDistributionConfig::States(vec![
                InitialDistributionVariant::default(),
            ]),
            discrete_map_vec: Some(DiscreteMapVec::Logistic(vec![map])),
            diff_system_vec: None,
            pars: Default::default(),
        };
        let benchmark_result = chaos_benchmark_single(&chaos_init);
        assert!(benchmark_result.is_ok());
        if let Ok(res) = benchmark_result {
            assert_eq!(res.num_valid_end_states, num_samples);
            assert!(res.runtime.as_nanos() > 0);
        }
        let benchmark_result = chaos_benchmark_threaded(&chaos_init);
        assert!(benchmark_result.is_ok());
        if let Ok(res) = benchmark_result {
            assert_eq!(res.num_valid_end_states, num_samples);
            assert!(res.runtime.as_nanos() > 0);
        }
    }
}
