use super::traits::RppgAlgorithm;
use super::utils::extract_hr_fft;
use super::utils::plot_signal;
use ndarray::s;
use ndarray::{Array, Array3};

pub struct Green;

impl RppgAlgorithm for Green {
    fn name(&self) -> &'static str {
        "green"
    }

    fn process(&self, frames: &Vec<Array3<f64>>, buffer: &mut Vec<f64>) {
        let dummy: Vec<f64> = frames
            .iter()
            .map(|frame| {
                // gets the green channel
                let slice = frame.slice(s![.., .., 2]);
                slice.mean().unwrap()
            })
            .collect();

        buffer.clear();
        buffer.extend(dummy);
    }

    fn extract_hr(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    ) -> f64 {
        self.process(frames, buffer);
        let signal_to_filter = buffer.clone();
        let filtered_signal = self.filter_signal(signal_to_filter, fps);
        let signal_for_plot_32 = filtered_signal.iter().map(|x| *x as f32).collect();
        plot_signal(&signal_for_plot_32);
        let hz = extract_hr_fft(buffer, fps);
        // convert to BPM
        hz * 60.0
    }

    fn process_filter(&self, frames: &Vec<Array3<f64>>, buffer: &mut Vec<f64>, fps: f64) {
        self.process(frames, buffer);
        let signal_to_filter = buffer.clone();
        let filtered_singal = self.filter_signal(signal_to_filter, fps);
        buffer.extend(filtered_singal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_green_process_simple() {
        let algo = Green {};

        // Create a 2x2 RGB frame with blue channel (index 2) = 50.0
        let frame = Array::from_shape_fn((2, 2, 3), |(_, _, c)| match c {
            0 => 0.0,  // Red
            1 => 0.0,  // Blue
            2 => 50.0, // Green
            _ => 0.0,
        });

        let frames = vec![frame.clone(), frame.clone()];
        let mut buffer = vec![];

        algo.process(&frames, &mut buffer);

        assert_eq!(buffer.len(), 2);
        for value in buffer {
            assert_eq!(value, 50.0); // Mean of 2x2 pixels each 50.0 = 50.0
        }
    }
}
