use super::plot_2_d::*;
use super::plot_3_d::*;
use super::plot_colors::*;
use super::plot_data::*;
use super::plot_styles::*;
use super::plot_utils::{flat_map_data_vec, flat_map_data_vec_and_parameter};
use crate::chaos::data::*;
use paste::paste;
macro_rules! impl_data_variant_plot {
    ($($variant:ident, $d2:expr, $d2_par:expr, $d3:expr, $d3_par:expr),*) => {
        paste!{
            impl Plot2D {
                pub fn create_point_series_without_parameters(&self, data: ChaosDataVec<'_>) -> Points2D {
                    match data {
                        $(
                            ChaosDataVec::$variant(data_vec) => flat_map_data_vec(data_vec, |x| self.[<transform_points_ $d2 _d>](x)),
                        )*
                    }
                }

                pub fn create_point_series_with_parameters(&mut self, data: ChaosDataVec<'_>) -> Points2D {
                    let par_values = self.get_parameter_values();
                    match data {
                        $(
                            ChaosDataVec::$variant(data_vec) => {
                                flat_map_data_vec_and_parameter(data_vec, par_values, |x, p| {
                                    self.[<points_with_parameter_ $d2_par _d>](x, p)
                                })
                            },
                        )*
                    }
                }
            }

            impl Plot3D {
                pub fn create_point_series_without_parameters(&self, data: ChaosDataVec<'_>) -> Points3D {
                    match data {
                        $(
                            ChaosDataVec::$variant(data_vec) => flat_map_data_vec(data_vec, |x| self.[<transform_points_ $d3 _d>](x)),
                        )*
                    }
                }

                pub fn create_point_series_with_parameters(&mut self, data: ChaosDataVec<'_>) -> Points3D {
                    let par_values = self.get_parameter_values();
                    match data {
                        $(
                            ChaosDataVec::$variant(data_vec) => {
                                flat_map_data_vec_and_parameter(data_vec, par_values, |x, p| {
                                    self.[<points_with_parameter_ $d3_par _d>](x, p)
                                })
                            },
                        )*
                    }
                }
            }

            impl<P, C: FromRGB + Clone> PlotData<P, C> {
                pub fn create_styles_for_chaos_data(
                    &mut self,
                    data: &ChaosDataVec<'_>,
                ) -> Vec<Style<C>> {
                    match data {
                        $(
                            ChaosDataVec::$variant(data_vec) => self.create_styles_for_chaos_data_generic(data_vec),
                        )*
                    }
                }
            }
        }
    };
}

impl_data_variant_plot! {
    State1, 1, n, 1, 1,
    State2, n, n, 2, n,
    State3, n, n, n, n,
    State4, n, n, n, n,
    ParticleXY, n, n, n, n,
    ParticleXYZ, n, n, n, n,
    FractalComplex, n, n, n, n,
    FractalDual, n, n, n, n,
    FractalPerplex, n, n, n, n,
    FractalQuaternion, n, n, n, n
}
