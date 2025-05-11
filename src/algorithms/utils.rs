use biquad::*;
use rustfft::num_traits::real::Real;

pub fn filterSignal(Signal: Vec<f64>, f0: f64) -> Vec<f64> {
    // Cutoff and sampling frequencies
    let f_low = 0.6;
    let f_high = 3.0;

    // Is this even the right way to to this ?
    let f_center = (f_low * f_high).sqrt(); // Geometric mean
    let bandwidth = f_high - f_low; // Hz
    let q = f_center / bandwidth;

    // Create coefficients for the biquads
    let coeffs =
        Coefficients::<f64>::from_params(Type::BandPass, f_center.hz(), f0.hz(), q).unwrap();

    // Create two different biquads
    let mut biquad1 = DirectForm1::<f64>::new(coeffs);

    let mut filtered_singal = Vec::new();

    // Run for all the inputs
    for elem in Signal {
        filtered_singal.push(biquad1.run(elem));
    }

    filtered_singal
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

    #[test]
    fn test_bandpass_sine_response_1_5hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 1.5;
        let sine = sine_wave(frequency, sample_rate, 1000);
        let output = filterSignal(sine, sample_rate);

        // Simple energy check
        let energy: f64 = output.iter().map(|x| x * x).sum();
        assert!(energy > 1.0); // Arbitrary threshold
    }

    #[test]
    fn test_bandpass_sine_response_0001hz() {
        let sample_rate: f64 = 25.0;
        let frequency: f64 = 0.1;
        let sine = sine_wave(frequency, sample_rate, 1000);
        let output = filterSignal(sine, sample_rate);

        // Simple energy check
        let energy: f64 = output.iter().map(|x| x * x).sum();
        assert!(energy > 1.0); // Arbitrary threshold
    }
}
