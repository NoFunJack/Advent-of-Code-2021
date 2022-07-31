use std::env;
use std::error::Error;
use std::fs;

mod floor;

use crate::floor::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let floor_str: String = fs::read_to_string(args.next().unwrap())?.parse()?;

    let mut floor = Floor::new(floor_str);
    let mut step_count = 0;

    while floor.step() {
        step_count += 1;
        println!("Step: {step_count}");
    }

    println!("no more movement after {step_count} steps!");

    Ok(())
}
