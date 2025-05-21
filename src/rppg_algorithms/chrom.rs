use super::traits::RppgAlgorithm;
use super::utils::extract_hr_fft;

use ndarray::Array3;
pub struct Chrom;

impl RppgAlgorithm for Chrom {
    fn name(&self) -> &'static str {
        "CHROM"
    }

    fn process(&self, frames: &Vec<Array3<f64>>, buffer: &mut Vec<f64>) {
        // Dummy logic
    }

    fn extract_hr(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    ) -> f64 {
        self.process(frames, buffer);
        extract_hr_fft(buffer, fps)
    }

    fn process_filter(&self, frames: &Vec<Array3<f64>>, buffer: &mut Vec<f64>, fps: f64) {
        self.process(frames, buffer);
        let signal_to_filter = buffer.clone();
        let filtered_singal = self.filter_signal(signal_to_filter, fps);
        buffer.extend(filtered_singal);
    }
}
