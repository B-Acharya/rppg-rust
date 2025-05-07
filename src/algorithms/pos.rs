use super::traits::RppgAlgorithm;

pub struct Pos;

impl RppgAlgorithm for Pos {
    fn name(&self) -> &'static str {
        "CHROM"
    }

    fn process(&self, frames: &Vec<opencv::core::Mat>, buffer: &mut Vec<f64>) {
        // Dummy logic
        vec![1.0; frames.len()];
        buffer.push(0.0);
    }
}
