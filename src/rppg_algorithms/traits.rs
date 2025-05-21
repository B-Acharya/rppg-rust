use biquad::Biquad;
use biquad::{Coefficients, DirectForm1, Type};
use ndarray::Array3;
use rustfft::{num_complex::Complex, FftPlanner};

pub trait RppgAlgorithm {
    fn name(&self) -> &'static str;
    fn process(&self, frames: &Vec<Array3<f64>>, buffer: &mut Vec<f64>);

    //TODO: Maybe moving this out into a seperate trait will help to have a default implimenation  ?
    fn extract_hr(
        &self,
        frames: &Vec<Array3<f64>>,
        buffer: &mut Vec<f64>,
        fps: f64,
        filter_signal: bool,
    ) -> f64;

    fn process_filter(&self, frames: &Vec<Array3<f64>>, buffer: &mut Vec<f64>, fps: f64);

    fn filter_signal(&self, signal: Vec<f64>, f0: f64) -> Vec<f64> {
        // Cutoff and sampling frequencies
        let f_low: f64 = 0.6;
        let f_high: f64 = 4.0;

        //need normalized cutoff frequcncies for this to work
        let f_low_normalize: f64 = f_low / (f0 * 0.5);
        let f_high_normalize: f64 = f_high / (f0 * 0.5);

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
}
