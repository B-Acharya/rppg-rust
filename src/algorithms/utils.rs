use biquad::*;

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

    let coeffs1 = Coefficients::<f64>::band_0db_from_cutting_frequencies(
        Type::BandPass,
        f_low_normalize,
        f_high_normalize,
    )
    .unwrap();

    println!("{:?}", coeffs1);

    let mut stage1 = DirectForm1::<f64>::new(coeffs1);

    signal.iter().map(|element| stage1.run(*element)).collect()
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
}
