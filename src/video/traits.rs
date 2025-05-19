use types::Frame;

pub trait VideoBackend {
    pub fn get_frames(path: &str) -> opencv::Result<(Vec<Frame>, f64), String>;
}
