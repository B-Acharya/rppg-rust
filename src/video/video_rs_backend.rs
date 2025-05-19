use super::traits::VideoBackend;
pub struct VideoRsBackend;

impl VideoBackend for VideoRsBackend {
    pub fn get_frames(path: &str) -> Result<(Vec<Frame>, f64), String> {
        video_rs::init().unwrap();
        //TODO: Convert video_rs_frame to Frame struct
        let mut frames = Vec::new();
        let mut timestamp: Vec<Time> = Vec::new();

        let location = Location::File(path.into());
        let mut decoder = Decoder::new(location).expect("failed to create decoder");

        //decoder.decode_iter().map(|frame| frames.push(frame.unwrap().1)).collect();
        for frame in decoder.decode_iter() {
            if let Ok((time, frame)) = frame {
                frames.push(frame);
            } else {
                break;
            }
        }

        let fps = decoder.frame_rate();

        Ok((frames, fps))
    }
}
