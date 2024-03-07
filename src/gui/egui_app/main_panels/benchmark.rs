use crate::chaos::benchmark::*;
use crate::gui::tooltips::*;
use crate::gui::*;
use egui::Ui;
use egui_plot::{HLine, Plot, PlotPoint, PlotPoints, Points, VLine};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct BenchmarkPanel {
    #[serde(skip)] // start without running benchmark
    run_benchmark: bool,
    #[serde(skip)] // benchmark must be reinitiated manually
    benchmark_result: ChaosBenchmarkResult,
    #[serde(skip)] // benchmark must be reinitiated manually
    bench_config: ChaosInitSchema,
    use_warmup: bool,
    num_iterations: usize,
    num_warmups: usize,
}
impl Default for BenchmarkPanel {
    fn default() -> Self {
        Self {
            run_benchmark: false,
            benchmark_result: Default::default(),
            bench_config: Default::default(),
            use_warmup: true,
            num_iterations: 50,
            num_warmups: 2,
        }
    }
}

impl PartialEq for BenchmarkPanel {
    fn eq(&self, other: &Self) -> bool {
        // only compare options for reset
        self.run_benchmark == other.run_benchmark
            && self.use_warmup == other.use_warmup
            && self.num_iterations == other.num_iterations
            && self.num_warmups == other.num_warmups
    }
}

impl BenchmarkPanel {
    pub fn benchmark_toggle(&mut self) -> bool {
        if self.run_benchmark {
            self.run_benchmark = false;
            true
        } else {
            false
        }
    }

    pub fn chaos_benchmark(&mut self, bench_config: ChaosInitSchema) {
        self.bench_config = bench_config;
        let num_warmups = if self.use_warmup { self.num_warmups } else { 0 };
        self.benchmark_result =
            chaos_benchmark(&self.bench_config, self.num_iterations, num_warmups);
    }

    fn valid_runtimes(&self) -> (Vec<f64>, usize) {
        let num_warmups = self.benchmark_result.number_of_warmups();
        let mut failed_warmups = 0;
        let runtimes = self
            .benchmark_result
            .runs()
            .iter()
            .enumerate()
            .filter_map(|(i, res)| match res {
                Ok(run) => Some(run.runtime_nanos() as f64),
                Err(_) => {
                    if i < num_warmups {
                        failed_warmups += 1;
                    }
                    None
                }
            })
            .collect();
        let valid_warmups = num_warmups - failed_warmups;
        (runtimes, valid_warmups)
    }
    pub fn conf_ui(&mut self, is_ready: bool, ui: &mut Ui) {
        ui.heading("Benchmark Configuration");
        group_horizontal(ui, |ui| {
            add_checkbox(LABEL_WARMUP, &mut self.use_warmup, ui, TIP_WARMUP);
            if self.use_warmup {
                integer_slider(
                    LABEL_NUM_WARMUPS,
                    &mut self.num_warmups,
                    50,
                    ui,
                    TIP_NUM_WARMUPS,
                );
            }
        });
        group_horizontal(ui, |ui| {
            integer_slider(
                LABEL_NUM_ITERATIONS,
                &mut self.num_iterations,
                200,
                ui,
                TIP_NUM_ITERATIONS,
            );
            if clickable_button(LABEL_BENCHMARK, false, is_ready, ui, TIP_BENCHMARK) {
                self.run_benchmark = true;
            };
        });
    }
    fn show_summary(&self, ui: &mut Ui) {
        let ChaosInitSchema {
            num_samples,
            num_executions,
            init_distr,
            discrete_map_vec,
            diff_system_vec,
            pars,
        } = &self.bench_config;
        let (parameter, par_values) = pars;
        group_vertical(ui, |ui| {
            ui.heading("Latest Benchmark Summary");
            group_horizontal(ui, |ui| {
                ui.label(format!("Number of Samples: {num_samples}",));
                ui.label(format!("Number of Executions: {num_executions}"));
            });
            group_horizontal(ui, |ui| {
                if let Some(map_vec) = discrete_map_vec {
                    let map_str: &'static str = map_vec.into();
                    ui.label(format!("Discrete Map: {map_str}"));
                };
                if let Some(diff_system_vec) = diff_system_vec {
                    let diff_system_str: &'static str = diff_system_vec.into();
                    ui.label(format!("Differential System: {diff_system_str}"));
                };
                if !par_values.is_empty() {
                    ui.label(format!("{} Parameter {parameter} values", par_values.len()));
                };
            });
            ui.label(format!("Distributions: {}", String::from(init_distr)));
        });
    }

    fn show_results(&self, ui: &mut Ui) {
        let num_warmups = self.benchmark_result.number_of_warmups();
        ui.collapsing("Results", |ui| {
            let result_labels: Vec<String> = self
                .benchmark_result
                .runs()
                .iter()
                .enumerate()
                .map(|(i, res)| {
                    let warmup_tag = if i < num_warmups { "!" } else { "" };
                    match res {
                        Ok(run) => {
                            let (runtime, label) = {
                                let runtime = run.runtime_millis();
                                if runtime < 10 {
                                    (run.runtime_nanos() as usize, "ns")
                                } else {
                                    (runtime, "ms")
                                }
                            };
                            let num_states = run.num_valid_end_states();
                            format!(
                                "{i} {warmup_tag} States: {num_states} Runtime: {runtime} {label}"
                            )
                        }
                        Err(e) => format!("{i} {warmup_tag} Error: {e}"),
                    }
                })
                .collect();
            egui::ScrollArea::vertical().show(ui, |ui| {
                group_vertical(ui, |ui| {
                    result_labels.into_iter().for_each(|l| {
                        ui.label(l);
                    })
                });
            });
        });

        let (runtimes, num_valid_warmups) = self.valid_runtimes();
        if !runtimes.is_empty() {
            let runtime_mean = {
                let (_, valid_runtimes) = runtimes.split_at(num_valid_warmups);
                let num_valid_runtimes = valid_runtimes.len();
                if num_valid_runtimes == 0 {
                    0.0
                } else {
                    let mut runtime_sum = 0.0;
                    for runtime in valid_runtimes {
                        runtime_sum += *runtime;
                    }
                    runtime_sum / (num_valid_runtimes as f64)
                }
            };
            group_horizontal(ui, |ui| {
                ui.label(format!(
                    "Mean: {:.2} ns Valid warmups: {}",
                    runtime_mean, num_valid_warmups
                ));
            });
            let plot = Plot::new("bench_plot")
                .set_margin_fraction(egui::Vec2::new(0.01, 0.01))
                .legend(Default::default());
            plot.show(ui, |plot_ui| {
                let points = runtimes
                    .iter()
                    .enumerate()
                    .map(|(i, y)| PlotPoint::new(i as f64, *y))
                    .collect();
                plot_ui.points(Points::new(PlotPoints::Owned(points)).name("Runtime ns"));
                if num_valid_warmups > 0 {
                    plot_ui.vline(VLine::new((num_valid_warmups - 1) as f64).name("Warm-Ups"));
                };
                plot_ui.hline(HLine::new(runtime_mean).name("Mean"));
            });
        }
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        self.show_summary(ui);
        self.show_results(ui);
    }
}
