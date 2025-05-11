mod chrom;
mod green;
mod pos;
mod traits;
mod utils;

pub use chrom::Chrom;
pub use green::Green;
pub use pos::Pos;
pub use traits::RppgAlgorithm;

pub fn get_algorithm(name: &str) -> Box<dyn RppgAlgorithm> {
    match name.to_lowercase().as_str() {
        "chrom" => Box::new(Chrom),
        "pos" => Box::new(Pos),
        "green" => Box::new(Green),
        _ => panic!("Unknown algorithm: {}", name),
    }
}
