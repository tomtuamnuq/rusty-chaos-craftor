use delegate::delegate;
use paste::paste;
use strum_macros::IntoStaticStr;

use crate::chaos::{
    data::*,
    execution::{continuous_exec::ContinuousVecExecutor, discrete_exec::DiscreteVecExecutor},
    fractal::*,
    functions::*,
    particle::{ParticleXY, ParticleXYSystemSolver, ParticleXYZ, ParticleXYZSystemSolver},
};

macro_rules! create_and_implement_executor_variants {
    ([$( $discrete_map:ident $discrete_state:expr),*] [$( $fractal_fn:ident),*] [$( $continuous_ode:ident $continuous_state:expr),*] [$( $particle_dim:ident),*]) => {
        paste!{
            pub enum ExecutorVariant {
                $(
                    $discrete_map(DiscreteVecExecutor<[<State $discrete_state>], SimpleDiscreteMap<$discrete_map>>),
                )*
                $(
                    [<Mandelbrot $fractal_fn Complex>](DiscreteVecExecutor<FractalComplex, [<Mandelbrot $fractal_fn>]<Complex>>),
                    [<Mandelbrot $fractal_fn Dual>](DiscreteVecExecutor<FractalDual, [<Mandelbrot $fractal_fn>]<Dual>>),
                    [<Mandelbrot $fractal_fn Perplex>](DiscreteVecExecutor<FractalPerplex, [<Mandelbrot $fractal_fn>]<Perplex>>),
                    [<Mandelbrot $fractal_fn Quaternion>](DiscreteVecExecutor<FractalQuaternion, [<Mandelbrot $fractal_fn>]<Quaternion>>),
                    [<Julia $fractal_fn Complex>](DiscreteVecExecutor<FractalComplex, [<Julia $fractal_fn>]<Complex>>),
                    [<Julia $fractal_fn Dual>](DiscreteVecExecutor<FractalDual, [<Julia $fractal_fn>]<Dual>>),
                    [<Julia $fractal_fn Perplex>](DiscreteVecExecutor<FractalPerplex, [<Julia $fractal_fn>]<Perplex>>),
                    [<Julia $fractal_fn Quaternion>](DiscreteVecExecutor<FractalQuaternion, [<Julia $fractal_fn>]<Quaternion>>),
                )*
                $(
                    $continuous_ode(ContinuousVecExecutor<[<State $continuous_state>], OdeSolver<[<State $continuous_state>], $continuous_ode>>),
                )*
                $(
                    [<Particle $particle_dim>](ContinuousVecExecutor<[<Particle $particle_dim>], [<Particle $particle_dim SystemSolver>]>),
                )*
            }
            impl ExecutorVariant {
                pub fn get_chaos_data_vec(&self) -> ChaosDataVec<'_> {
                    use ExecutorVariant::*;
                    match self {
                        $(
                            $discrete_map(ex) => ChaosDataVec::[<State $discrete_state>](ex.get_chaos_data_refs()),
                        )*
                        $(
                            [<Mandelbrot $fractal_fn Complex>](ex) => ChaosDataVec::FractalComplex(ex.get_chaos_data_refs()),
                            [<Mandelbrot $fractal_fn Dual>](ex) => ChaosDataVec::FractalDual(ex.get_chaos_data_refs()),
                            [<Mandelbrot $fractal_fn Perplex>](ex) => ChaosDataVec::FractalPerplex(ex.get_chaos_data_refs()),
                            [<Mandelbrot $fractal_fn Quaternion>](ex) => ChaosDataVec::FractalQuaternion(ex.get_chaos_data_refs()),
                            [<Julia $fractal_fn Complex>](ex) => ChaosDataVec::FractalComplex(ex.get_chaos_data_refs()),
                            [<Julia $fractal_fn Dual>](ex) => ChaosDataVec::FractalDual(ex.get_chaos_data_refs()),
                            [<Julia $fractal_fn Perplex>](ex) => ChaosDataVec::FractalPerplex(ex.get_chaos_data_refs()),
                            [<Julia $fractal_fn Quaternion>](ex) => ChaosDataVec::FractalQuaternion(ex.get_chaos_data_refs()),
                        )*
                        $(
                            $continuous_ode(ex) => ChaosDataVec::[<State $continuous_state>](ex.get_chaos_data_refs()),
                        )*
                        $(
                            [<Particle $particle_dim>](ex) => ChaosDataVec::[<Particle $particle_dim>](ex.get_chaos_data_refs()),
                        )*
                    }
                }
                delegate! {
                    to match self{
                        $(
                            ExecutorVariant::$discrete_map(ex) => ex,
                        )*
                        $(
                            ExecutorVariant::[<Mandelbrot $fractal_fn Complex>](ex) => ex,
                            ExecutorVariant::[<Mandelbrot $fractal_fn Dual>](ex) => ex,
                            ExecutorVariant::[<Mandelbrot $fractal_fn Perplex>](ex) => ex,
                            ExecutorVariant::[<Mandelbrot $fractal_fn Quaternion>](ex) => ex,
                            ExecutorVariant::[<Julia $fractal_fn Complex>](ex) => ex,
                            ExecutorVariant::[<Julia $fractal_fn Dual>](ex) => ex,
                            ExecutorVariant::[<Julia $fractal_fn Perplex>](ex) => ex,
                            ExecutorVariant::[<Julia $fractal_fn Quaternion>](ex) => ex,
                        )*
                        $(
                            ExecutorVariant::$continuous_ode(ex) => ex,
                        )*
                        $(
                            ExecutorVariant::[<Particle $particle_dim>](ex) => ex,
                        )*
                    }
                    {
                        pub fn execute_vec(&mut self, num_executions: usize, time: &Time);
                        pub fn reinit_states_vec(&mut self, init_distr: &[InitialDistributionVariant]);
                    }
                }
            }


            #[derive(Clone, IntoStaticStr)]
            pub enum DiscreteMapVec {
                $(
                    $discrete_map(Vec<SimpleDiscreteMap<$discrete_map>>),
                )*
                $(
                    [<Mandelbrot $fractal_fn Complex>](Vec<[<Mandelbrot $fractal_fn>]<Complex>>),
                    [<Mandelbrot $fractal_fn Dual>](Vec<[<Mandelbrot $fractal_fn>]<Dual>>),
                    [<Mandelbrot $fractal_fn Perplex>](Vec<[<Mandelbrot $fractal_fn>]<Perplex>>),
                    [<Mandelbrot $fractal_fn Quaternion>](Vec<[<Mandelbrot $fractal_fn>]<Quaternion>>),
                    [<Julia $fractal_fn Complex>](Vec<[<Julia $fractal_fn>]<Complex>>),
                    [<Julia $fractal_fn Dual>](Vec<[<Julia $fractal_fn>]<Dual>>),
                    [<Julia $fractal_fn Perplex>](Vec<[<Julia $fractal_fn>]<Perplex>>),
                    [<Julia $fractal_fn Quaternion>](Vec<[<Julia $fractal_fn>]<Quaternion>>),
                )*
            }
            #[derive(Clone, IntoStaticStr)]
            pub enum OdeSystemSolverVec {
                $(
                    $continuous_ode(Vec<OdeSolver<[<State $continuous_state>], $continuous_ode>>),
                )*
                $(
                    [<Particle $particle_dim>](Vec<[<Particle $particle_dim SystemSolver>]>),
                )*
            }
        } // paste
    };
}
create_and_implement_executor_variants! {
    [
        Logistic 1,
        Tent 1,
        Gauss 1,
        Circle 1,
        Chirikov 2,
        Henon 2,
        ArnoldsCat 2,
        Duffing 2,
        Bogdanov 2,
        Chialvo 2,
        DeJongRing 2,
        Tinkerbell 2,
        Baker 2,
        Clifford 2,
        Ikeda 2,
        Gingerbreadman 2,
        KaplanYorke 2,
        Rulkov 2,
        Zaslavskii 2,
        ReverseProbability 2,
        Shah 3,
        Memristive 3,
        Sfsimm 4
    ]
    [Power, Transcendental, Sinus, Sinh, Zubieta, Picard, Biomorph]
    [
        Brusselator 2,
        VanDerPol 2,
        QuadrupTwoOrbit 2,
        Lorenz 3,
        Rossler 3,
        Chen 3,
        Aizawa 3,
        ChuasCircuit 3,
        RabinovichFabrikant 3,
        GenesioTesi 3,
        BurkeShaw 3,
        Halvorsen 3,
        ThreeSpeciesLotkaVolterra 3,
        Rikitake 3,
        HindmarshRose 3,
        Ababneh 4,
        WeiWang 4
    ]
    [XY, XYZ]
}
