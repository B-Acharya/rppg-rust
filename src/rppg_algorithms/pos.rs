use super::traits::RppgAlgorithm;
use super::utils::extract_hr_fft;
use super::utils::{average, filterSignal, mean_rgb, plot_signal, std_deviation};

use ndarray::Array3;
pub struct Pos;

//impl RppgAlgorithm for Pos {
//    fn name(&self) -> &'static str {
//        "CHROM"
//    }
//
//    fn process(
//        &self,
//        frames: &Vec<Array3<f64>>,
//        buffer: &mut Vec<f64>,
//        fps: f64,
//        filter_singal: bool,
//    ) {
//        let mut rbg = Vec::new();
//
//        let dummy_mask = opencv::core::no_array();
//
//        //spatial averaging
//        for frame in frames.iter() {
//            let mean = opencv::core::mean(&frame, &dummy_mask).unwrap();
//            let b = mean[0];
//            let r = mean[1];
//            let g = mean[2];
//            rbg.push((r, g, b));
//        }
//
//        const WIN_SEC: f64 = 1.6;
//
//        let n = rbg.len();
//
//        let mut H: Vec<f64> = vec![0.0; n];
//        let l = (WIN_SEC * fps).ceil() as usize;
//
//        for n_i in 0..n {
//            if n_i < l {
//                continue;
//            }
//            let m = n_i - l;
//
//            let rbg_slice = &&rbg[m..n_i];
//            let temporal_mean_channels = mean_rgb(rbg_slice);
//
//            let mut cn = Vec::new();
//            for x in rbg_slice.iter() {
//                let r = x.0 / temporal_mean_channels.0;
//                let g = x.1 / temporal_mean_channels.1;
//                let b = x.2 / temporal_mean_channels.2;
//
//                cn.push((r, g, b))
//            }
//
//            let s1 = cn.iter().map(|x| x.1 - 1.0 * x.2).collect();
//            let s2 = cn.iter().map(|x| -2.0 * x.0 + x.1 + x.2).collect();
//
//            let s1_std = std_deviation(&s1).unwrap();
//            let s2_std = std_deviation(&s2).unwrap();
//
//            let ratio = s1_std as f64 / s2_std as f64;
//
//            //let s1_s2_iter = std::iter::zip(s1, s2);
//
//            //let h = s1_s2_iter.map(|(s1_v, s2_v)| s1_v + ratio * s2_v).collect();
//
//            // is the above line equal to this ?
//            let mut h = Vec::new();
//            for index in 0..s1.len() {
//                let value = s1[index] + ratio * s2[index];
//                h.push(value);
//            }
//
//            let h_mean = average(&h).unwrap();
//
//            for (i, val) in h.iter().enumerate() {
//                let index = i + m;
//                let value = H[index] + (val - h_mean);
//                H[index] = value;
//                if index > n_i {
//                    println!("brr theres something wrong here")
//                }
//            }
//        }
//        buffer.clear();
//
//        if filter_singal {
//            let filtered_singal = filterSignal(H, fps);
//            buffer.extend(filtered_singal);
//        } else {
//            buffer.extend(H);
//        }
//
//        //TODO: Add detred
//        //TODO Add testcases
//        //TODO move the mean and median to utils.rs
//    }
//
//    fn extract_hr(
//        &self,
//        frames: &Vec<Array3<f64>>,
//        buffer: &mut Vec<f64>,
//        fps: f64,
//        filter_signal: bool,
//    ) -> f64 {
//        self.process(frames, buffer, fps, filter_signal);
//        let singal_for_plot = buffer.clone();
//        let signal_for_plot_32 = singal_for_plot.iter().map(|x| *x as f64).collect();
//        plot_signal(&signal_for_plot_32);
//        extract_hr_fft(buffer, fps)
//    }
//}
//
//impl Pos {
//    fn process_ndarary(
//        &self,
//        frames: &Vec<Array3<f64>>,
//        buffer: &mut Vec<f64>,
//        fps: f64,
//        filter_singal: bool,
//    ) {
//        let mut rbg = Vec::new();
//
//        let dummy_mask = opencv::core::no_array();
//
//        //spatial averaging
//        for frame in frames.iter() {
//            let mean = opencv::core::mean(&frame, &dummy_mask).unwrap();
//            let b = mean[0];
//            let r = mean[1];
//            let g = mean[2];
//            rbg.push((r, g, b));
//        }
//
//        const WIN_SEC: f64 = 1.6;
//
//        let n = rbg.len();
//
//        let mut H: Vec<f64> = vec![0.0; n];
//        let l = (WIN_SEC * fps).ceil() as usize;
//
//        for n_i in 0..n {
//            if n_i < l {
//                continue;
//            }
//            let m = n_i - l;
//
//            let rbg_slice = &&rbg[m..n_i];
//            let temporal_mean_channels = mean_rgb(rbg_slice);
//
//            let mut cn = Vec::new();
//            for x in rbg_slice.iter() {
//                let r = x.0 / temporal_mean_channels.0;
//                let g = x.1 / temporal_mean_channels.1;
//                let b = x.2 / temporal_mean_channels.2;
//
//                cn.push((r, g, b))
//            }
//
//            let s1 = cn.iter().map(|x| x.1 - 1.0 * x.2).collect();
//            let s2 = cn.iter().map(|x| -2.0 * x.0 + x.1 + x.2).collect();
//
//            let s1_std = std_deviation(&s1).unwrap();
//            let s2_std = std_deviation(&s2).unwrap();
//
//            let ratio = s1_std as f64 / s2_std as f64;
//
//            //let s1_s2_iter = std::iter::zip(s1, s2);
//
//            //let h = s1_s2_iter.map(|(s1_v, s2_v)| s1_v + ratio * s2_v).collect();
//
//            // is the above line equal to this ?
//            let mut h = Vec::new();
//            for index in 0..s1.len() {
//                let value = s1[index] + ratio * s2[index];
//                h.push(value);
//            }
//
//            let h_mean = average(&h).unwrap();
//
//            for (i, val) in h.iter().enumerate() {
//                let index = i + m;
//                let value = H[index] + (val - h_mean);
//                H[index] = value;
//                if index > n_i {
//                    println!("brr theres something wrong here")
//                }
//            }
//        }
//        buffer.clear();
//
//        if filter_singal {
//            let filtered_singal = filterSignal(H, fps);
//            buffer.extend(filtered_singal);
//        } else {
//            buffer.extend(H);
//        }
//
//        //TODO: Add detred
//        //TODO Add testcases
//        //TODO move the mean and median to utils.rs
//    }
//}
#[cfg(test)]
mod tests {
    use super::*; // Bring the items from the outer module into the scope

    //#[test]
    //fn test_green_name() {
    //    let green_algorithm = Green;
    //    assert_eq!(green_algorithm.name(), "green");
    //}

    //#[test]
    //fn test_green_process_empty_frames() {
    //    let green_algorithm = Green;
    //    let frames = vec![];
    //    let mut buffer = vec![];
    //    green_algorithm.process(&frames, &mut buffer, 25.0, true);
    //    assert_eq!(buffer.len(), 0);
    //}

    //#[test]
    //fn test_green_process_solid_green_frame() {
    //    let green_algorithm = Green;
    //    let rows = 10;
    //    let cols = 10;

    //    // Create a dummy image (Mat) with all green pixels (BGR format)
    //    let mut frame = opencv::core::Mat::new_rows_cols_with_default(
    //        rows,
    //        cols,
    //        opencv::core::CV_8UC3,
    //        opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0),
    //    )
    //    .unwrap();

    //    let fps = 25.0;

    //    let frames = vec![frame];
    //    let mut buffer = vec![0.0; frames.len()]; // Initialize buffer with the correct size

    //    green_algorithm.process(&frames, &mut buffer, fps, false);

    //    assert_eq!(buffer.len(), 1);
    //    assert_eq!(buffer[0], 255.0, "Expected mean green value to be 255.0");
    //}
}
