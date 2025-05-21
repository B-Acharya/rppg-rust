use std::error::Error;
use std::process;

use video::traits::VideoBackend;

pub mod pipeline;
pub mod rppg_algorithms;
pub mod rppg_types;
pub mod video;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let rppgalgorithm = rppg_algorithms::get_algorithm(&config.algorithm);

    let backend = video::opencv_backend::OpencvCvBackend;
    let frames_fps = backend
        .get_frames_fps(&config.video_path)
        .unwrap_or_else(|err| {
            eprintln!("Problems parsing the input:{err} ");
            process::exit(1);
        });

    let frame_length = frames_fps.0.len();
    let mut signal_buffer: Vec<f64> = vec![0.0; frame_length];
    let filter_signal = true;

    //// how should you design this function ? It should pass botha mutalable reference so that
    //// there's less data that is transfered to the funciton, but then what should be returend ?
    //rppgalgorithm.process(&frames.0, &mut signal_buffer, frames.1, filter_signal);
    //rppgalgorithm.(&frames.0, &mut signal_buffer, frames.1, filter_signal);
    let hr = rppgalgorithm.extract_hr(
        &frames_fps.0,
        &mut signal_buffer,
        frames_fps.1,
        filter_signal,
    );

    //println! {"{}",frame_length};
    ////println! {"{:?}",signal_buffer};
    //println! {"{:?}",hr };

    Ok(())
}

pub struct Config {
    pub video_path: String,
    pub algorithm: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough parameters");
        }

        let video_path = args[1].clone();
        let algorithm = args[2].clone();

        Ok(Config {
            video_path,
            algorithm,
        })
    }
}
