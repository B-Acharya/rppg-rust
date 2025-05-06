use super::traits::RppgAlgorithm;

pub struct Chrom;

impl RppgAlgorithm for Chrom {
    fn name(&self) -> &'static str {
        "CHROM"
    }

    fn process(&self, frames: &Vec<opencv::core::Mat>, buffer: &mut Vec<f32>) {
        // Dummy logic
    }
}
