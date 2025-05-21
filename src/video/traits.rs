use ndarray::Array3;

pub trait VideoBackend {
    fn get_frames_fps(&self, path: &str) -> opencv::Result<(Vec<Array3<f64>>, f64), String>;
}
