use std::error::Error;
use std::process;

pub mod algorithms;
pub mod video;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // TODO: Implement algorithms
    let rppgalgorithm = algorithms::get_algorithm(&config.algorithm);

    let frames = video::get_frames(&config.video_path).unwrap_or_else(|err| {
        eprintln!("Problems parsing the input:{err} ");
        process::exit(1);
    });

    let frame_length = frames.len();
    let mut signal_buffer: Vec<f64> = vec![0.0; frame_length];

    // how should you design this function ? It should pass botha mutalable reference so that
    // there's less data that is transfered to the funciton, but then what should be returend ?
    rppgalgorithm.process(&frames, &mut signal_buffer);

    println! {"{}",frame_length};
    println! {"{:?}",signal_buffer};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}
