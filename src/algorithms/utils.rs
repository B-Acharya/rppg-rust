use biquad::*;
use rustfft::num_traits::real::Real;

pub fn filterSignal(Signal: Vec<f64>, f0: f64) -> Vec<f64> {
    // Cutoff and sampling frequencies
    let f_low = 0.6;
    let f_high = 3.0;

    // Is this even the right way to to this ?
    let f_center = (f_low * f_high).sqrt(); // Geometric mean
                                            // 1.341640
    let bandwidth = f_high - f_low; // Hz
    let q = f_center / bandwidth;

    println!("{}", f0);

    // Create coefficients for the biquads
    //let coeffs = Coefficients::<f64>::from_params(Type::BandPass, f0.hz(), f_center.hz(), q).unwrap();

    //// Create two different biquads
    //let mut biquad1 = DirectForm2Transposed::<f64>::new(coeffs);

    let coeffs1 =
        Coefficients::<f64>::from_params(Type::BandPass, f0.hz(), f_center.hz(), q).unwrap();

    let mut stage1 = DirectForm1::<f64>::new(coeffs1);

    // Second biquad (another 2nd order with same Q and f0)
    // You might tweak Q or split the order across slightly different Qs for better stability
    let coeffs2 =
        Coefficients::<f64>::from_params(Type::BandPass, f0.hz(), f_center.hz(), q).unwrap();

    let mut stage2 = DirectForm1::<f64>::new(coeffs2);

    //let mut filtered_singal = Vec::new();

    // Run for all the inputs
    Signal
        .iter()
        .map(|element| stage1.run(*element))
        .map(|element| stage2.run(element))
        .collect()
    // for elem in Signal {
    //     filtered_singal.push(biquad1.run(elem));
    // }

    // filtered_singal
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
    fn test_bandpass_sine_response_1_34hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 0.9;
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
