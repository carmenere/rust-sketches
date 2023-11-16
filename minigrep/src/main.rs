use std::env;
use std::error::Error;
use std::process;

use minigrep;

fn main() -> Result<(), Box<dyn Error>>{
    let argv: Vec<String> = env::args().collect();

    let conf = minigrep::Config::new(argv).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("path: {}, pattern: {}, inore_case: {}", conf.path, conf.pattern, conf.ignore_case);

    Ok(minigrep::run(conf)?)
}
