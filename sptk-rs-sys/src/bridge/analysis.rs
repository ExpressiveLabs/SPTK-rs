use autocxx::prelude::*;

//         include!("SPTK/analysis/adaptive_generalized_cepstral_analysis.h");
//         include!("SPTK/analysis/adaptive_mel_cepstral_analysis.h");
//         include!("SPTK/analysis/adaptive_mel_generalized_cepstral_analysis.h");
//         include!("SPTK/analysis/aperiodicity_extraction.h");
//         include!("SPTK/analysis/aperiodicity_extraction_by_tandem.h");
//         include!("SPTK/analysis/aperiodicity_extraction_by_world.h");
//         include!("SPTK/analysis/autocorrelation_analysis.h");
//         include!("SPTK/analysis/fast_fourier_transform_cepstral_analysis.h");
//         include!("SPTK/analysis/mel_cepstral_analysis.h");
//         include!("SPTK/analysis/mel_filter_bank_analysis.h");
//         include!("SPTK/analysis/mel_frequency_cepstral_coefficients_analysis.h");
//         include!("SPTK/analysis/mel_generalized_cepstral_analysis.h");
//         include!("SPTK/analysis/perceptual_linear_predictive_coefficients_analysis.h");
//         include!("SPTK/analysis/pitch_extraction.h");
//         include!("SPTK/analysis/pitch_extraction_by_dio.h");
//         include!("SPTK/analysis/pitch_extraction_by_harvest.h");
//         include!("SPTK/analysis/pitch_extraction_by_rapt.h");
//         include!("SPTK/analysis/pitch_extraction_by_reaper.h");
//         include!("SPTK/analysis/pitch_extraction_by_swipe.h");
//         include!("SPTK/analysis/second_order_all_pass_mel_cepstral_analysis.h");
//         include!("SPTK/analysis/spectrum_extraction.h");
//         include!("SPTK/analysis/spectrum_extraction_by_world.h");
//         include!("SPTK/analysis/zero_crossing_analysis.h");

include_cpp! {
    #include "SPTK/analysis/adaptive_generalized_cepstral_analysis.h"

    safety!(unsafe)
    generate!("sptk::AdaptiveGeneralizedCepstralAnalysis")
    generate_pod!("sptk::AdaptiveGeneralizedCepstralAnalysis::Buffer")

}

use crate::bridge::analysis::ffi::sptk::{
    AdaptiveGeneralizedCepstralAnalysis,
    AdaptiveGeneralizedCepstralAnalysis_Buffer
};

impl std::fmt::Debug for AdaptiveGeneralizedCepstralAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdaptiveGeneralizedCepstralAnalysis")
            .field("order", &self.GetNumOrder())
            .field("min_epsilon", &self.GetMinEpsilon())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_generalized_cepstral_analysis() {
        let agca = AdaptiveGeneralizedCepstralAnalysis::new(c_int(25), c_int(2), 1.0e-8, 0.0, 1.0, 1.0, false).within_box();

        // agca.Run()

        println!("{:?}", agca);
    }
}