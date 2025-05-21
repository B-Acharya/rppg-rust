use super::traits::VideoBackend;
pub struct VideoRsBackend;
use ndarray::{Array3, ArrayView3};
use video_rs::Decoder;
use video_rs::Frame;
use video_rs::Location;

impl VideoBackend for VideoRsBackend {
    fn get_frames_fps(&self, path: &str) -> Result<(Vec<Array3<f64>>, f64), String> {
        video_rs::init().unwrap();
        //TODO: Convert video_rs_frame to Array3 struct
        let mut frames = Vec::new();

        let location = Location::File(path.into());
        let mut decoder = Decoder::new(location).expect("failed to create decoder");

        //decoder.decode_iter().map(|frame| frames.push(frame.unwrap().1)).collect();
        for frame in decoder.decode_iter() {
            if let Ok((time, frame)) = frame {
                let array_f64 = frame.mapv(|x| x as f64 / 255.0);
                frames.push(array_f64.to_owned());
            } else {
                break;
            }
        }

        let fps = decoder.frame_rate();

        Ok((frames, fps.into()))
    }
}
