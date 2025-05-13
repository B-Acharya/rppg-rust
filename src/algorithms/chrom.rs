use super::traits::RppgAlgorithm;

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
}
