//         include!("SPTK/math/discrete_cosine_transform.h");
//         include!("SPTK/math/discrete_fourier_transform.h");
//         include!("SPTK/math/distance_calculation.h");
//         include!("SPTK/math/durand_kerner_method.h");
//         include!("SPTK/math/dynamic_time_warping.h");
//         include!("SPTK/math/entropy_calculation.h");
//         include!("SPTK/math/fast_fourier_transform.h");
//         include!("SPTK/math/fourier_transform.h");
//         include!("SPTK/math/frequency_transform.h");
//         include!("SPTK/math/gaussian_mixture_model_based_conversion.h");
//         include!("SPTK/math/gaussian_mixture_modeling.h");
//         include!("SPTK/math/histogram_calculation.h");
//         include!("SPTK/math/inverse_discrete_cosine_transform.h");
//         include!("SPTK/math/inverse_discrete_fourier_transform.h");
//         include!("SPTK/math/inverse_fast_fourier_transform.h");
//         include!("SPTK/math/inverse_fourier_transform.h");
//         include!("SPTK/math/levinson_durbin_recursion.h");
//         include!("SPTK/math/matrix.h");
//         include!("SPTK/math/matrix2d.h");
//         include!("SPTK/math/minmax_accumulation.h");
//         include!("SPTK/math/mode_accumulation.h");
//         include!("SPTK/math/principal_component_analysis.h");
//         include!("SPTK/math/product_accumulation.h");
//         include!("SPTK/math/real_valued_fast_fourier_transform.h");
//         include!("SPTK/math/real_valued_inverse_fast_fourier_transform.h");
//         include!("SPTK/math/reverse_levinson_durbin_recursion.h");
//         include!("SPTK/math/scalar_operation.h");
//         include!("SPTK/math/second_order_all_pass_frequency_transform.h");
//         include!("SPTK/math/second_order_all_pass_inverse_frequency_transform.h");
//         include!("SPTK/math/statistics_accumulation.h");
//         include!("SPTK/math/symmetric_matrix.h");
//         include!("SPTK/math/symmetric_system_solver.h");
//         include!("SPTK/math/toeplitz_plus_hankel_system_solver.h");
//         include!("SPTK/math/two_dimensional_fast_fourier_transform.h");
//         include!("SPTK/math/two_dimensional_inverse_fast_fourier_transform.h");
//         include!("SPTK/math/two_dimensional_real_valued_fast_fourier_transform.h");
//         include!("SPTK/math/vandermonde_system_solver.h");

use autocxx::prelude::*;

include_cpp! {
    #include "SPTK/math/matrix.h"
    #include "SPTK/math/matrix2d.h"

    safety!(unsafe)
    generate!("sptk::Matrix")
    generate!("sptk::Matrix2D")
}

use crate::bridge::math::ffi::sptk::{
    Matrix,
    Matrix2D
};

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix")
            .field("cols", &self.GetNumColumn())
            .field("rows", &self.GetNumRow())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut matrix = Matrix::new(10, 10).within_box();

        println!("{:?}", matrix);
    }
}