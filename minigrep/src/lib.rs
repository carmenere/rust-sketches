use std::{path::Path, error::Error};
use std::fs::{self, File};
use std::io::{self, BufReader, BufRead};

pub struct Config {
    pub pattern: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(argv: Vec<String>) -> Result<Config, &'static str> {
        if argv.len() != 3 {
            return Err("Incorrect number of arguments.")
        }

        // Ok(Config {
        //     pattern: std::mem::take(&mut argv[1]),
        //     path: std::mem::take(&mut argv[2]),
        // })

        Ok(Config {
            path: argv[1].clone(),
            pattern: argv[2].clone(),
            ignore_case: std::env::var("IGNORE_CASE").is_ok(), // is_ok() return `true` if var is set
        })
    }
}

pub fn run (conf: Config) -> Result<(), Box<dyn Error>> {
    let s = fs::read_to_string(&conf.path)?;
    match conf.ignore_case {
        true => {
            for line in search_ignore_case(&s, &conf.pattern) {
                println!("HIT: {line}.")
            }
        },
        false => {
            for line in search(&s, &conf.pattern) {
                println!("HIT: {line}.")
            }
        },
    }


    Ok(())
}

pub fn search_ignore_case<'a>(s: &'a String, pattern: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut result: Vec<&'a str> = Vec::with_capacity(16);
    for line in s.lines() {
        if line.to_lowercase().contains(&pattern) {
            result.push(&line)
        }
    }

    result
}

pub fn search<'a>(s: &'a String, pattern: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::with_capacity(16);
    for line in s.lines() {
        if line.contains(pattern) {
            result.push(&line)
        }
    }

    result
}

fn buf_read<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>
{
        let file = File::open(filename)?;
        let lines: io::Lines<BufReader<File>> = BufReader::new(file).lines();
        Ok(lines)
}