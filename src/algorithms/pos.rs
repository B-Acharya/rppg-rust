use super::traits::RppgAlgorithm;
use faer;

pub struct Pos;

impl RppgAlgorithm for Pos {
    fn name(&self) -> &'static str {
        "CHROM"
    }

    fn process(
        &self,
        frames: &Vec<opencv::core::Mat>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_singal: bool,
    ) {
        let mut rbg = Vec::new();

        let dummy_mask = opencv::core::no_array();

        //spatial averaging
        for frame in frames.iter() {
            let mean = opencv::core::mean(&frame, &dummy_mask).unwrap();
            let b = mean[0];
            let r = mean[0];
            let g = mean[0];
            rbg.push((r, g, b));
        }

        const WIN_SEC: f64 = 1.6;

        let n = rbg.len() as u32;

        let mut H = Vec::new();
        let l = (WIN_SEC * fps).ceil() as u32;

        for i in 0..n {
            let m = n - l;
            if m >= 0 {
                //normalize

                let rbg_slice = rbg[m..n];
                let temporal_mean_channels = mean_rgb(rbg_slice);

                let cn = Vec::new();
                for x in rbg_slice.iter() {
                    let r = x.0 as f64 / temporal_mean_channels.0 as f64;
                    let g = x.1 as f64 / temporal_mean_channels.1 as f64;
                    let b = x.2 as f64 / temporal_mean_channels.2 as f64;

                    cn.push((r, g, b))
                }

                let s1 = cn.iter().map(|x| (0, x.1, -1 * x.2)).collect();
                let s2 = cn.iter().map(|x| (-2 * x.0, x.1, x.2)).collect();

                let s1_std = std(s1).unwrap();
                let s2_std = std(s2).unwrap();

                let ratio = s1_std as f64 / s2_std as f64;

                let h = Vec::new();
                for index in s1.len() {
                    let value = s1[index] + ratio * s2[index];
                    h.push(value);
                }

                let h_mean = average(h).unwrap();
                
                for (i, val) in h.iter().enumerate() {
                    H[i+m] = H[i+m] + (val - h_mean);
                    i = i + 1;
                    if i + m > n {
                        println!("brr theres something wrong here")
                    }
                }
            }
        }
        buffer.clear();

        if filter_singal {
            let filtered_singal = filterSignal(H, fps);
            buffer.extend(filtered_singal);
        } else {
            buffer.extend(dummy);
        }

        //TODO: Add detred 
        //TODO Add testcases
        //TODO move the mean and median to utils.rs
    }
}

fn mean_rgb(rgb: Vec<(f64, f64, f64)>) -> (f64, f64, f64) {
    let mut red = Vec::new();
    let mut green = Vec::new();
    let mut blue = Vec::new();
    for vals in &rgb {
        red.push(vals.0);
        green.push(vals.1);
        blue.push(vals.2);
    }

    let red_mean = average(red).unwrap() ;
    let green_mean = average(green).unwrap();
    let blue_mean = average(blue).unwrap();

    (red_mean, green_mean, blue_mean)
}

/// Yo this this crazy to implement all the stats functions 
///https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
fn average(nums: Vec<f64>) -> Option<f64> {
    let sum: f64 = nums.iter().sum();
    let n = nums.len() ;

    match n {
        positive if positive > 0 => Some(sum / n as f64),
        _  => None, 
    }
}

///https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
fn std_deviation(data: Vec<f64>) -> Option<f64> {
        match (average(data), data.len()) {
                (Some(data_mean), count) if count > 0 => {
                        let variance = data.iter().map(|value| {
                                let diff = data_mean - (*value as f64);

                                diff * diff
                            }).sum::<f64>() / count as f64;

                        Some(variance.sqrt())
                    },
                _ => None
            }
            })
        }
    }
}
