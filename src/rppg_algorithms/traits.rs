use ndarray::Array3;

pub trait RppgAlgorithm {
    fn name(&self) -> &'static str;
    fn process(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    );

    //TODO: Maybe moving this out into a seperate trait will help to have a default implimenation  ?
    fn extract_hr(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    ) -> f64;
}
