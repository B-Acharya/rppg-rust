use super::traits::VideoBackend;
use ndarray::{Array3, ArrayView3};
use opencv::{
    core::{MatTraitConst, MatTraitConstManual},
    videoio::{VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY},
};

pub struct OpencvCvBackend;

impl VideoBackend for OpencvCvBackend {
    fn get_frames_fps(&self, path: &str) -> Result<(Vec<Array3<f64>>, f64), String> {
        let mut frames = Vec::new();

        let mut cam = VideoCapture::from_file(path, CAP_ANY).unwrap();

        if !cam.is_opened().unwrap() {
            panic!("Unable to open default camera!");
        }

        loop {
            let mut frame = opencv::core::Mat::default();
            match cam.read(&mut frame) {
                Ok(true) => {
                    if frame.empty() {
                        eprintln!("Array3 not found");
                        break;
                    }
                    if !frame.is_continuous() {
                        eprintln!("Array3 not stored in continous memmory");
                        break;
                    }

                    let mut frame_f64 = opencv::core::Mat::default();
                    frame
                        .convert_to(&mut frame_f64, opencv::core::CV_64FC3, 1.0 / 255.0, 0.0)
                        .unwrap();

                    // Conversion of opencv::core::MAT to ArrayView3
                    let size = frame_f64.size().unwrap();
                    let h = size.height as usize;
                    let w = size.width as usize;
                    let channels = frame_f64.channels() as usize;

                    //How are images stroed that this makes sense ?
                    let data = frame_f64.data_typed::<f64>().unwrap();
                    let temp_array = ArrayView3::<f64>::from_shape((h, w, channels), data).unwrap();
                    frames.push(temp_array.into_owned());
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
        Ok((frames, fps.into()))
    }
}
