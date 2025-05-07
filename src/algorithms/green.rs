use opencv::core::MatTraitConst;

use super::traits::RppgAlgorithm;

pub struct Green;

impl RppgAlgorithm for Green {
    fn name(&self) -> &'static str {
        "green"
    }

    fn process(&self, frames: &Vec<opencv::core::Mat>, buffer: &mut Vec<f64>) {
        let mut i = 0;
        for frame in frames {
            let mut green_frame = opencv::core::Mat::default();

            // BRG extract green channel
            match opencv::core::extract_channel(&frame, &mut green_frame, 1) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Error extracting channel: {}", e);
                    break;
                }
            }

            // Maybe this can directly extract mean for each chnannel and I dont need to extract
            // channel
            let dummy_mask = opencv::core::no_array();

            match opencv::core::mean(&green_frame, &dummy_mask) {
                Ok(mean) => {
                    //is this event the green channel ??
                    buffer[i] = mean[0];
                    i += 1;
                }
                Err(e) => {
                    eprintln!("Failed to calcualte the mean in GREEN {}", e);
                    break;
                }
            }
        }
    }
}
