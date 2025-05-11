use super::traits::RppgAlgorithm;
use super::utils::filterSignal;

pub struct Green;

impl RppgAlgorithm for Green {
    fn name(&self) -> &'static str {
        "green"
    }

    fn process(&self, frames: &Vec<opencv::core::Mat>, buffer: &mut Vec<f64>) {
        let dummy_mask = opencv::core::no_array();
        let dummy: Vec<f64> = frames
            .iter()
            .map(|frame| match opencv::core::mean(&frame, &dummy_mask) {
                Ok(mean) => {
                    //is this event the green channel BGR format
                    mean[1]
                }
                Err(e) => {
                    eprintln!(
                        "Failed to calcualte the mean GREEN setting green value to zero{}",
                        e
                    );
                    0.0
                }
            })
            .collect();

        buffer.clear();
        buffer.extend(dummy.clone());

        let signaltofilter = dummy.clone();

        //TODO: replace fps with accurate number
        //let filtered_singal = filterSignal(signaltofilter, 25.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the items from the outer module into the scope

    #[test]
    fn test_green_name() {
        let green_algorithm = Green;
        assert_eq!(green_algorithm.name(), "green");
    }

    #[test]
    fn test_green_process_empty_frames() {
        let green_algorithm = Green;
        let frames = vec![];
        let mut buffer = vec![];
        green_algorithm.process(&frames, &mut buffer);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_green_process_solid_green_frame() {
        let green_algorithm = Green;
        let rows = 10;
        let cols = 10;

        // Create a dummy image (Mat) with all green pixels (BGR format)
        let mut frame = opencv::core::Mat::new_rows_cols_with_default(
            rows,
            cols,
            opencv::core::CV_8UC3,
            opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
        )
        .unwrap();

        let frames = vec![frame];
        let mut buffer = vec![0.0; frames.len()]; // Initialize buffer with the correct size

        green_algorithm.process(&frames, &mut buffer);

        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 255.0, "Expected mean green value to be 255.0");
    }
}
