use biquad::Biquad;
use biquad::{Coefficients, DirectForm1, Type};
use rustfft::{num_complex::Complex, FftPlanner};

pub fn filterSignal(signal: Vec<f64>, f0: f64) -> Vec<f64> {
    // Cutoff and sampling frequencies
    let f_low: f64 = 0.6;
    let f_high: f64 = 4.0;

    //need normalized cutoff frequcncies for this to work
    let f_low_normalize: f64 = f_low / (f0 * 0.5);
    let f_high_normalize: f64 = f_high / (f0 * 0.5);
    // Is this even the right way to to this ?
    let f_center = (f_low * f_high).sqrt(); // Geometric mean
                                            // 1.341640
    let bandwidth = f_high - f_low; // Hz
    let q = f_center / bandwidth;

    println!("{}", f0);

    //TODO: Can I use a cascaded filter, how ill it affect my singal
    let coeffs1 = Coefficients::<f64>::band_0db_from_cutting_frequencies(
        Type::BandPass,
        f_low_normalize,
        f_high_normalize,
    )
    .unwrap();

    println!("{:?}", coeffs1);

    //TODO: Which form should I use ?
    let mut stage1 = DirectForm1::<f64>::new(coeffs1);

    signal.iter().map(|element| stage1.run(*element)).collect()
}

pub fn extract_hr_fft(signal: Vec<f64>, sampling_rate: f64) -> f64 {
    let n = signal.len();
    let mut signal_complex: Vec<Complex<f64>> =
        signal.iter().map(|&x| Complex::new(x, 0.0)).collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(n);

    fft.process(&mut signal_complex);

    let max_index = signal_complex[..n / 2]
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.norm_sqr()))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap();

    let freq_resolution = sampling_rate / n as f64;
    max_index as f64 * freq_resolution
}

pub fn mean_rgb(rgb: &[(f64, f64, f64)]) -> (f64, f64, f64) {
    let mut red = Vec::new();
    let mut green = Vec::new();
    let mut blue = Vec::new();
    for vals in rgb {
        red.push(vals.0);
        green.push(vals.1);
        blue.push(vals.2);
    }

    let red_mean = average(&red).unwrap();
    let green_mean = average(&green).unwrap();
    let blue_mean = average(&blue).unwrap();

    (red_mean, green_mean, blue_mean)
}

/// Yo this this crazy to implement all the stats functions
///https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
pub fn average(nums: &Vec<f64>) -> Option<f64> {
    let sum: f64 = nums.iter().sum();
    let n = nums.len();

    match n {
        positive if positive > 0 => Some(sum / n as f64),
        _ => None,
    }
}

///https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html
pub fn std_deviation(data: &Vec<f64>) -> Option<f64> {
    match (average(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value);

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Bring the items#[test]
                  //
    fn sine_wave(frequency: f64, sample_rate: f64, samples: usize) -> Vec<f64> {
        (0..samples)
            .map(|n| {
                let t = n as f64 / sample_rate;
                (2.0 * std::f64::consts::PI * frequency * t).sin()
            })
            .collect()
    }

    fn compute_energy(signal: &Vec<f64>) -> f64 {
        signal.iter().map(|x| x * x).sum()
    }

    #[test]
    fn test_bandpass_sine_response_4hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 4.0;
        let sine = sine_wave(frequency, sample_rate, 100);
        let output = filterSignal(sine.clone(), sample_rate);

        // Simple energy check
        let input_energy = compute_energy(&sine);
        let output_energy = compute_energy(&output);
        let energy_ratio = output_energy / input_energy;

        println!("In-band energy ratio: {:.2}", energy_ratio);
        assert!(
            energy_ratio < 0.2,
            "Filter attenuated too much: {:.2}%",
            100.0 * output_energy / input_energy,
        );
    }

    #[test]
    fn test_bandpass_sine_response_2_5hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 2.5;
        let sine = sine_wave(frequency, sample_rate, 100);
        let output = filterSignal(sine.clone(), sample_rate);

        // Simple energy check
        let input_energy = compute_energy(&sine);
        let output_energy = compute_energy(&output);
        let energy_ratio = output_energy / input_energy;

        println!("In-band energy ratio: {:.2}", energy_ratio);
        assert!(
            energy_ratio > 0.4,
            "Filter attenuated too much: {:.2}%",
            100.0 * output_energy / input_energy,
        );
    }

    #[test]
    fn test_fft_sine_response_4hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 4.0;
        let sine = sine_wave(frequency, sample_rate, 100);
        let output = extract_hr_fft(sine, sample_rate);

        // Simple energy check
        println!("Freq {}", output);
        assert!(output == 4.0, "Frequncy not equal to 4.0");
    }

    #[test]
    fn test_bandpass_sine_response_1_5hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 1.5;
        let sine = sine_wave(frequency, sample_rate, 100);
        let output = filterSignal(sine.clone(), sample_rate);

        // Simple energy check
        let input_energy = compute_energy(&sine);
        let output_energy = compute_energy(&output);
        let energy_ratio = output_energy / input_energy;

        println!("In-band energy ratio: {:.2}", energy_ratio);
        assert!(
            energy_ratio > 0.4,
            "Filter attenuated too much: {:.2}%",
            100.0 * output_energy / input_energy,
        );
    }

    #[test]
    fn test_bandpass_sine_response_0001hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 0.1;
        let sine = sine_wave(frequency, sample_rate, 1000);
        let output = filterSignal(sine.clone(), sample_rate);

        // Simple energy check
        let input_energy = compute_energy(&sine);
        let output_energy = compute_energy(&output);
        let energy_ratio = output_energy / input_energy;

        println!("In-band energy ratio: {:.2}", energy_ratio);
        assert!(
            energy_ratio < 0.2,
            "Filter attenuated too much: {:.2}%",
            100.0 * output_energy / input_energy,
        );
    }

    #[test]
    fn test_average_with_multiple_elements() {
        let nums = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(average(&nums), Some(2.5));
    }

    #[test]
    fn test_average_with_single_element() {
        let nums = vec![42.0];
        assert_eq!(average(&nums), Some(42.0));
    }

    #[test]
    fn test_average_with_empty_vector() {
        let nums: Vec<f64> = vec![];
        assert_eq!(average(&nums), None);
    }

    #[test]
    fn test_average_with_negative_numbers() {
        let nums = vec![-1.0, -2.0, -3.0];
        assert_eq!(average(&nums), Some(-2.0));
    }

    #[test]
    fn test_average_with_mixed_sign_numbers() {
        let nums = vec![-1.0, 0.0, 1.0];
        assert_eq!(average(&nums), Some(0.0));
    }

    #[test]
    fn test_average_with_nan() {
        let nums = vec![1.0, f64::NAN, 3.0];
        assert!(average(&nums).unwrap().is_nan());
    }

    #[test]
    fn test_average_with_infinity() {
        let nums = vec![1.0, f64::INFINITY];
        assert_eq!(average(&nums), Some(f64::INFINITY));
    }
}
