use super::traits::RppgAlgorithm;
use super::utils::extract_hr_fft;
use super::utils::filterSignal;
use super::utils::plot_signal;
use ndarray::s;
use ndarray::Array3;

pub struct Green;

impl RppgAlgorithm for Green {
    fn name(&self) -> &'static str {
        "green"
    }

    fn process(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_singal: bool,
    ) {
        let dummy_mask = opencv::core::no_array();
        let dummy: Vec<f64> = frames
            .iter()
            .map(|frame| {
                // gets the green channel
                let slice = frame.slice(s![.., .., 2]);
                slice.mean().unwrap()
            })
            .collect();

        buffer.clear();

        if filter_singal {
            let filtered_singal = filterSignal(dummy, fps);
            buffer.extend(filtered_singal);
        } else {
            buffer.extend(dummy);
        }
    }

    fn extract_hr(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    ) -> f64 {
        self.process(frames, buffer, fps, filter_signal);
        let singal_for_plot = buffer.clone();
        let signal_for_plot_32 = singal_for_plot.iter().map(|x| *x as f32).collect();
        plot_signal(&signal_for_plot_32);
        extract_hr_fft(buffer, fps)
    }
}

impl Green {
    fn process_ndarary(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_singal: bool,
    ) {
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
        green_algorithm.process(&frames, &mut buffer, 25.0, true);
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

        let fps = 25.0;

        let frames = vec![frame];
        let mut buffer = vec![0.0; frames.len()]; // Initialize buffer with the correct size

        green_algorithm.process(&frames, &mut buffer, fps, false);

        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer[0], 255.0, "Expected mean green value to be 255.0");
    }
}
