pub trait RppgAlgorithm {
    fn name(&self) -> &'static str;
    fn process(
        &self,
        frames: &Vec<opencv::core::Mat>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    );
}
