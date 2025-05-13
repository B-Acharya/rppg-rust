use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};

pub fn get_frames(path: &str) -> opencv::Result<(Vec<Mat>, f64), String> {
    let mut frames: Vec<Mat> = Vec::new();

    //TODO: Change the capture methods if no path is provided and use the webcam as an input to
    //process the values
    let mut cam = VideoCapture::from_file(path, CAP_ANY).unwrap();

    if !cam.is_opened().unwrap() {
        panic!("Unable to open default camera!");
    }

    loop {
        let mut frame = Mat::default();
        match cam.read(&mut frame) {
            Ok(true) => {
                if frame.empty() {
                    break;
                }
                frames.push(frame);
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
