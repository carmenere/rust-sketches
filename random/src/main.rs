use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::io::{self, Write}; // Write is needed for std::io::stdout().flush()
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // rand::thread_rng() retrieves the lazily-initialized thread-local random number generator, seeded by the system.
    // .gen_range() generates a random value in the given range.
    let rnd_num: u32 = rand::thread_rng().gen_range(1..=10);

    loop {
        print!("Please input number: ");

        // The print!() doesn't flush write buffer to output it to the terminal.
        std::io::stdout().flush().unwrap();

        let mut buf = String::new();

        let r = io::stdin().read_line(&mut buf).unwrap_or_else(|err| {
            eprintln!("Error during read input: {:?}", err);
            process::exit(1)
        });
        
        let usr_num = match buf.trim().parse::<u32>() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Error during parse input: {:?}.", err);
                eprintln!("Try again.", );
                continue;
            },
        };

        match usr_num.cmp(&rnd_num) {
            Ordering::Less => {
                println!("Less! Try again.");
            },
            Ordering::Equal => {
                println!("Equal! You win!.");
                break Ok(());
            },
            Ordering::Greater => {
                println!("Greater! Try again.");
            },
        }

    }
}