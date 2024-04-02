use anyhow::{bail, Error};

use crate::chaos::{
    data::*,
    execution::executor_variants::*,
    execution::{continuous_exec::ContinuousVecExecutor, discrete_exec::DiscreteVecExecutor},
};

macro_rules! try_init_from_chaos_data {
    ($self:ident, $constructor:ident, $data:ident, $data_variant:ident) => {
        match $data {
            $data_variant::State1(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [Logistic, Tent, Gauss, Circle],
                    []
                );
            }
            $data_variant::State2(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [
                        Chirikov,
                        Henon,
                        ArnoldsCat,
                        Bogdanov,
                        Chialvo,
                        DeJongRing,
                        Duffing,
                        Tinkerbell,
                        Baker,
                        Clifford,
                        Ikeda,
                        Gingerbreadman,
                        KaplanYorke,
                        Rulkov,
                        Zaslavskii
                    ],
                    [Brusselator, VanDerPol, QuadrupTwoOrbit]
                );
            }
            $data_variant::State3(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [Shah, Memristive],
                    [
                        Lorenz,
                        Rossler,
                        Chen,
                        Aizawa,
                        ChuasCircuit,
                        RabinovichFabrikant,
                        GenesioTesi,
                        BurkeShaw,
                        Halvorsen,
                        ThreeSpeciesLotkaVolterra,
                        Rikitake,
                        HindmarshRose
                    ]
                );
            }
            $data_variant::State4(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [Sfsimm],
                    [Ababneh, WeiWang]
                );
            }
            $data_variant::ParticleXY(initial_chaos_data) => {
                map_executor_variant!($self, $constructor, initial_chaos_data, [], [ParticleXY]);
            }
            $data_variant::ParticleXYZ(initial_chaos_data) => {
                map_executor_variant!($self, $constructor, initial_chaos_data, [], [ParticleXYZ]);
            }
            $data_variant::FractalComplex(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [
                        MandelbrotPowerComplex,
                        MandelbrotTranscendentalComplex,
                        MandelbrotSinusComplex,
                        MandelbrotSinhComplex,
                        MandelbrotZubietaComplex,
                        MandelbrotPicardComplex,
                        MandelbrotBiomorphComplex,
                        JuliaPowerComplex,
                        JuliaTranscendentalComplex,
                        JuliaSinusComplex,
                        JuliaSinhComplex,
                        JuliaZubietaComplex,
                        JuliaPicardComplex,
                        JuliaBiomorphComplex
                    ],
                    []
                );
            }
            $data_variant::FractalDual(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [
                        MandelbrotPowerDual,
                        MandelbrotTranscendentalDual,
                        MandelbrotSinusDual,
                        MandelbrotSinhDual,
                        MandelbrotZubietaDual,
                        MandelbrotPicardDual,
                        MandelbrotBiomorphDual,
                        JuliaPowerDual,
                        JuliaTranscendentalDual,
                        JuliaSinusDual,
                        JuliaSinhDual,
                        JuliaZubietaDual,
                        JuliaPicardDual,
                        JuliaBiomorphDual
                    ],
                    []
                );
            }
            $data_variant::FractalPerplex(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [
                        MandelbrotPowerPerplex,
                        MandelbrotTranscendentalPerplex,
                        MandelbrotSinusPerplex,
                        MandelbrotSinhPerplex,
                        MandelbrotZubietaPerplex,
                        MandelbrotPicardPerplex,
                        MandelbrotBiomorphPerplex,
                        JuliaPowerPerplex,
                        JuliaTranscendentalPerplex,
                        JuliaSinusPerplex,
                        JuliaSinhPerplex,
                        JuliaZubietaPerplex,
                        JuliaPicardPerplex,
                        JuliaBiomorphPerplex
                    ],
                    []
                );
            }
            $data_variant::FractalQuaternion(initial_chaos_data) => {
                map_executor_variant!(
                    $self,
                    $constructor,
                    initial_chaos_data,
                    [
                        MandelbrotPowerQuaternion,
                        MandelbrotTranscendentalQuaternion,
                        MandelbrotSinusQuaternion,
                        MandelbrotSinhQuaternion,
                        MandelbrotZubietaQuaternion,
                        MandelbrotPicardQuaternion,
                        MandelbrotBiomorphQuaternion,
                        JuliaPowerQuaternion,
                        JuliaTranscendentalQuaternion,
                        JuliaSinusQuaternion,
                        JuliaSinhQuaternion,
                        JuliaZubietaQuaternion,
                        JuliaPicardQuaternion,
                        JuliaBiomorphQuaternion
                    ],
                    []
                );
            }
        }
    };
}

macro_rules! map_executor_variant {
    ($self:ident, $constructor:ident, $data:ident, [$( $discrete_variant:ident),*], [$( $continuous_variant:ident),*] ) => {
        if let Some(ref discrete_maps_variant) = $self.discrete_map_vec {
            $self.executor = match discrete_maps_variant{
                $(DiscreteMapVec::$discrete_variant(ref maps) => Some(ExecutorVariant::$discrete_variant(DiscreteVecExecutor::$constructor(
                   $data, maps))),)*
                _ => None,
            };
        } else if let Some(ref ode_solvers_variant) = $self.ode_solver_vec {
            $self.executor = match ode_solvers_variant{
                $(OdeSystemSolverVec::$continuous_variant(ref solvers) => Some(ExecutorVariant::$continuous_variant(ContinuousVecExecutor::$constructor(
                   $data, solvers))),)*
                _ => None,
            };
        }
    };
}
pub struct ChaosExecutionController {
    initial_chaos_data: Option<ChaosDataVariant>,
    discrete_map_vec: Option<DiscreteMapVec>,
    ode_solver_vec: Option<OdeSystemSolverVec>,
    executor: Option<ExecutorVariant>,
    initial_distributions: InitialDistributionConfig,
    time: Time,
}

impl Default for ChaosExecutionController {
    fn default() -> Self {
        Self {
            initial_chaos_data: None,
            discrete_map_vec: None,
            ode_solver_vec: None,
            executor: None,
            initial_distributions: Default::default(),
            time: 0.0,
        }
    }
}

impl ChaosExecutionController {
    fn try_init_executor(&mut self) -> Result<(), Error> {
        if let Some(initial_chaos_data_variant) = &mut self.initial_chaos_data {
            try_init_from_chaos_data!(
                self,
                new_single,
                initial_chaos_data_variant,
                ChaosDataVariant
            );
            if self.executor.is_some() {
                self.initial_chaos_data = None;
                self.time = 0.0;
            } else {
                self.discrete_map_vec = None;
                self.ode_solver_vec = None;
            }
        } else if let Some(ref executor) = self.executor {
            let existing_chaos_data = executor.get_chaos_data_vec();
            try_init_from_chaos_data!(self, new, existing_chaos_data, ChaosDataVec);
            if self.executor.is_none() {
                // map_executor_variant! sets it to None if no valid combi of data and map/solver
                // a valid combi must be set
                self.discrete_map_vec = None;
                self.ode_solver_vec = None;
                self.time = 0.0;
                bail!("Dimension mismatch between data and discrete map / ode-system. Removing the chaotic function!");
            }
        }
        Ok(())
    }
    pub fn generate_initial_chaos_data(
        &mut self,
        num_samples: usize,
        init_distr: InitialDistributionConfig,
    ) -> Result<(), Error> {
        self.initial_chaos_data = Some(ChaosDataVariant::generate_initial_chaos_data(
            num_samples,
            &init_distr,
        )?);
        self.initial_distributions = init_distr;
        self.try_init_executor()?;
        Ok(())
    }

    pub fn dimensionality(&self) -> DistributionDimensions {
        self.initial_distributions.dimensionality()
    }

    pub fn set_discrete_mappers(&mut self, maps: DiscreteMapVec) -> Result<(), Error> {
        self.discrete_map_vec = Some(maps);
        self.ode_solver_vec = None;
        self.try_init_executor()?;
        Ok(())
    }

    pub fn set_differential_solvers(
        &mut self,
        diff_solvers: OdeSystemSolverVec,
    ) -> Result<(), Error> {
        self.discrete_map_vec = None;
        self.ode_solver_vec = Some(diff_solvers);
        self.try_init_executor()?;
        Ok(())
    }

    pub fn get_chaos_data(&self) -> Result<ChaosDataVec<'_>, Error> {
        if let Some(executor) = &self.executor {
            Ok(executor.get_chaos_data_vec())
        } else if let Some(chaos_data_variant) = &self.initial_chaos_data {
            Ok(ChaosDataVec::from_chaos_data(chaos_data_variant))
        } else {
            bail!("No data available for plotting: Executor and initial data are not set.")
        }
    }

    pub fn execute(&mut self, num_executions: usize) -> Result<(), Error> {
        if let Some(executor_variant) = &mut self.executor {
            executor_variant.execute_vec(num_executions, &self.time);
            self.time += num_executions as Time;
            Ok(())
        } else {
            bail!("Executor is not set: Cannot execute chaotic functions.");
        }
    }

    pub fn reinit_states(&mut self) -> Result<(), Error> {
        if let Some(executor_variant) = &mut self.executor {
            match &self.initial_distributions {
                InitialDistributionConfig::States(v) => {
                    if v.is_empty() {
                        bail!("No initial distributions set: Cannot reinit states!");
                    };
                    executor_variant.reinit_states_vec(v);
                }
                InitialDistributionConfig::ParticleXY(v) => {
                    executor_variant.reinit_states_vec(v);
                }
                InitialDistributionConfig::ParticleXYZ(v) => {
                    executor_variant.reinit_states_vec(v);
                }
                _ => (), // not necessary for fractals since they do not overflow
            }
            Ok(())
        } else {
            bail!("Executor is not set: Cannot reinit states!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chaos::functions::*;
    #[test]
    fn test_controller_init() -> Result<(), Error> {
        let num_samples = 2;
        let (x, y, z) = (1.0, 2.0, 3.0);
        let init_distr = vec![
            InitialDistributionVariant::Fixed(Fixed { value: x }),
            InitialDistributionVariant::Fixed(Fixed { value: y }),
            InitialDistributionVariant::Fixed(Fixed { value: z }),
        ];
        let test_state = State3::new(x, y, z);
        let mut controller = ChaosExecutionController::default();
        assert!(
            controller.get_chaos_data().is_err(),
            "Controller has no initial data!"
        );
        controller.generate_initial_chaos_data(
            num_samples,
            InitialDistributionConfig::States(init_distr.clone()),
        )?;
        assert!(controller.get_chaos_data().is_ok());
        assert_eq!(controller.dimensionality(), DIMS_STATE3);
        let system = OdeSystemSolverVec::Lorenz(vec![OdeSolver::new(Lorenz::default())]);
        controller.set_differential_solvers(system)?;
        if let ChaosDataVec::State3(chaos_data_vec) = controller.get_chaos_data()? {
            assert_eq!(
                chaos_data_vec.len(),
                1,
                "There must be exactly one ChaosData instance!"
            );
            let chaos_data = chaos_data_vec[0];
            assert_eq!(
                chaos_data.data().len(),
                2,
                "There must be two generated samples in one ChaosData instance!"
            );
        } else {
            bail!("Three dimensional data must be State3!");
        };
        let map = SimpleDiscreteMap::new(Shah::default());
        let system_3_d = DiscreteMapVec::Shah(vec![map]);
        controller.set_discrete_mappers(system_3_d)?;
        if let ChaosDataVec::State3(chaos_data_vec) = controller.get_chaos_data()? {
            let chaos_data = chaos_data_vec[0];
            assert_eq!(
                chaos_data.data(),
                &[Some(test_state), Some(test_state)],
                "Data must not change during init!"
            );
        } else {
            bail!("3D Data must still exist since Shah is also 3-dimensional!");
        };
        let map = SimpleDiscreteMap::new(Logistic::default());
        let system_1_d = DiscreteMapVec::Logistic(vec![map]);
        assert!(
            controller.set_discrete_mappers(system_1_d).is_err(),
            "Setting a map with a wrong dimensionality errors!"
        );
        assert!(
            controller.get_chaos_data().is_err(),
            "Data must have been removed if a system with different dimension was tried to get set!"
        );
        controller.generate_initial_chaos_data(
            num_samples,
            InitialDistributionConfig::States(init_distr),
        )?;
        let map = SimpleDiscreteMap::new(Shah::default());
        let system_3_d = DiscreteMapVec::Shah(vec![map]);
        controller.set_discrete_mappers(system_3_d)?;
        let init_distr = vec![InitialDistributionVariant::Fixed(Fixed { value: x })];
        controller.generate_initial_chaos_data(
            num_samples,
            InitialDistributionConfig::States(init_distr),
        )?;
        let new_chaos_data = controller.get_chaos_data().expect("New Chaos Data exists");
        if let ChaosDataVec::State1(chaos_data_vec) = new_chaos_data {
            let chaos_data = chaos_data_vec[0];
            assert_eq!(
                chaos_data.data(),
                &[Some(State1::new(x)), Some(State1::new(x))],
                "New data must have been generated!"
            );
        } else {
            bail!("1D Data must have been generated!");
        };
        Ok(())
    }
    #[test]
    fn test_controller_discrete_with_parametrization() -> Result<(), Error> {
        let num_samples = 2;
        let x = 1.0;
        let init_distr = vec![InitialDistributionVariant::Fixed(Fixed { value: x })];
        let test_state = State1::new(x);
        let mut controller = ChaosExecutionController::default();
        controller.generate_initial_chaos_data(
            num_samples,
            InitialDistributionConfig::States(init_distr),
        )?;
        let maps = DiscreteMapVec::Logistic(vec![
            SimpleDiscreteMap::new(Logistic { r: 0.0 }),
            SimpleDiscreteMap::new(Logistic { r: 1.0 }),
            SimpleDiscreteMap::new(Logistic { r: 2.0 }),
        ]);
        controller.set_discrete_mappers(maps)?;
        let chaos_data_variant = controller.get_chaos_data().expect("Chaos Data exists");
        if let ChaosDataVec::State1(chaos_data_vec) = chaos_data_variant {
            assert_eq!(
                chaos_data_vec.len(),
                3,
                "There must be one chaos data instance per parameter configuration!"
            );
            let chaos_data = chaos_data_vec[0];
            assert_eq!(
                chaos_data.data(),
                &[Some(test_state), Some(test_state)],
                "Single data must have been replicated!"
            );
        } else {
            bail!("1D Data must exist!");
        };
        controller.execute(1)?;
        let maps = DiscreteMapVec::Tent(vec![
            SimpleDiscreteMap::new(Tent { mu: 0.0 }),
            SimpleDiscreteMap::new(Tent { mu: 1.0 }),
            SimpleDiscreteMap::new(Tent { mu: 2.0 }),
            SimpleDiscreteMap::new(Tent { mu: 2.0 }),
        ]);
        controller.set_discrete_mappers(maps)?;
        controller.execute(1)?;
        let chaos_data_variant = controller.get_chaos_data().expect("Chaos Data exists");
        if let ChaosDataVec::State1(chaos_data_vec) = chaos_data_variant {
            assert_eq!(
                chaos_data_vec.len(),
                4,
                "There must be one chaos data instance per parameter!"
            );
            let chaos_data = chaos_data_vec[0];
            assert_ne!(
                chaos_data.data(),
                &[Some(test_state), Some(test_state)],
                "Single data must have changed!"
            );
        } else {
            bail!("1D Data must exist after new map with same dimensionality has been set!");
        };
        Ok(())
    }
    #[test]
    fn test_controller_continuous_with_parametrization() -> Result<(), Error> {
        let num_samples = 2;
        let (x, y) = (1.0, 2.0);
        let init_distr = vec![
            InitialDistributionVariant::Fixed(Fixed { value: x }),
            InitialDistributionVariant::Fixed(Fixed { value: y }),
        ];
        let test_state = State2::new(x, y);
        let mut controller = ChaosExecutionController::default();
        controller.generate_initial_chaos_data(
            num_samples,
            InitialDistributionConfig::States(init_distr),
        )?;
        let maps = OdeSystemSolverVec::VanDerPol(vec![
            OdeSolver::new(VanDerPol { mu: 0.1 }),
            OdeSolver::new(VanDerPol { mu: 0.2 }),
            OdeSolver::new(VanDerPol { mu: 0.3 }),
        ]);
        controller.set_differential_solvers(maps)?;
        if let ChaosDataVec::State2(chaos_data_vec) = controller.get_chaos_data()? {
            assert_eq!(
                chaos_data_vec.len(),
                3,
                "There must be one chaos data instance per parameter configuration!"
            );
            let chaos_data = chaos_data_vec[0];
            assert_eq!(
                chaos_data.data(),
                &[Some(test_state), Some(test_state)],
                "Single data must have been replicated!"
            );
        } else {
            bail!("2D Data must exist!");
        };
        controller.execute(1)?;
        let changed_test_data: Vec<ChaosData<State2>> =
            if let ChaosDataVec::State2(chaos_data_vec) = controller.get_chaos_data()? {
                assert_eq!(
                chaos_data_vec.len(),
                3,
                "There must be one chaos data instance per parameter (even after single execution)!"
            );
                chaos_data_vec.into_iter().map(|cd| (*cd).clone()).collect()
            } else {
                bail!("Data must exist after execution!");
            };
        let maps = DiscreteMapVec::Chirikov(vec![
            SimpleDiscreteMap::new(Chirikov { k: 0.5 }),
            SimpleDiscreteMap::new(Chirikov { k: 1.0 }),
        ]);
        controller.set_discrete_mappers(maps)?;
        if let ChaosDataVec::State2(chaos_data_vec) = controller.get_chaos_data()? {
            assert_eq!(
                chaos_data_vec.len(),
                2,
                "There must be one chaos data instance per parameter (even after new maps/systems with same dimensionality were set)!"
            );
            for i in 0..2 {
                assert_eq!(*(chaos_data_vec[i]), changed_test_data[i], "Data at same positions must still exist for newly setted parameter configurations!")
            }
        } else {
            bail!("Data must exist after setting mappers with same dimensionality!");
        };
        controller.execute(1)?;
        Ok(())
    }
}
