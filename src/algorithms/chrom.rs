use super::traits::RppgAlgorithm;
use super::utils::extract_hr_fft;

pub struct Chrom;

impl RppgAlgorithm for Chrom {
    fn name(&self) -> &'static str {
        "CHROM"
    }

    fn process(
        &self,
        frames: &Vec<opencv::core::Mat>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_singal: bool,
    ) {
        // Dummy logic
    }

    fn extract_hr(
        &self,
        frames: &Vec<opencv::core::Mat>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    ) -> f64 {
        self.process(frames, buffer, fps, filter_signal);
        extract_hr_fft(buffer, fps)
    }
}
