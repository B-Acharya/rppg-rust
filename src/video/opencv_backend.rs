use super::traits::VideoBackend;
pub struct opencvCvBackend;
use crate::types::Frame;
use ndarray::ArrayView3;
use opencv::{
    core::{MatTraitConst, MatTraitConstManual},
    videoio::{VideoCapture, CAP_ANY},
};

impl VideoBackend for opencvCvBackend {
    pub fn get_frames(path: &str) -> Result<(Vec<Frame>, f64), String> {
        let mut frames: Vec<Frame> = Vec::new();

        //TODO: Convert opencvframe to Frame struct

        let mut cam = VideoCapture::from_file(path, CAP_ANY).unwrap();

        if !cam.is_opened().unwrap() {
            panic!("Unable to open default camera!");
        }

        loop {
            let mut frame = opencv::core::Mat::default();
            match cam.read(&mut frame) {
                Ok(true) => {
                    if frame.empty() {
                        eprintln!("Frame not found");
                        break;
                    }
                    if !frame.is_continuous() {
                        eprintln!("Frame not stored in continous memmory");
                        break;
                    }

                    let mut frame_f32 = opencv::core::Mat::defualt();
                    frame
                        .convert_to(frame_f32, opencv::core::CV_32FC3, 1.0 / 255.0, 0.0)
                        .unwrap();

                    let shape = frame_f32.size().unwrap();

                    let h = size.height as usize;
                    let w = size.width as usize;
                    let channels = mat.channels().unwrap() as usize;

                    let data = frame.data_bytes();
                    let temp_array = ArrayView3::from_shape((h, w, channels), data);
                    frames.push(temp_array);
                }
                Ok(false) => {
                    println!("No more frames!");
                    break;
                }

                Err(e) => {
                    eprintln!("Failed to read from videos {}", e);
                    break;
                }
            }
        }

        let fps = cam.get(opencv::videoio::CAP_PROP_FPS).unwrap();
        Ok((frames, fps))
    }
}
