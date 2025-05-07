use rppg::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problems parsing the input:{err} ");
        process::exit(1);
    });

    if let Err(e) = rppg::run(config) {
        eprintln!("Problems parsing the input:{e} ");
        process::exit(1);
    }
}
