use super::traits::RppgAlgorithm;

pub struct Green;

impl RppgAlgorithm for Green {
    fn name(&self) -> &'static str {
        "green"
    }

    fn process(&self, frames: &Vec<opencv::core::Mat>, buffer: &mut Vec<f32>) {
        // Dummy logic

        buffer[0] = 1.0;
    }
}
