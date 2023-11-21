#![allow(deprecated)]
use anyhow;

use example::settings::*;

fn main() -> anyhow::Result<()> {
    let settings = Settings::new();
    println!("setings: {:?}", settings);
    Ok(())
}