use std::env;
use std::error::Error;
use std::fs;

mod alu;
mod modelnumber;

use crate::alu::*;
use crate::modelnumber::*;

fn main() -> Result<(), Box<dyn Error>> {
    // input program
    let mut args = env::args();
    args.next();
    let program: String = fs::read_to_string(args.next().unwrap())?.parse()?;
    // println!("Intructions:\n\n{}", program);
    let mut alu = ALU::new(program);

    // try model numbers
    for nr in ModelNumbers::new() {
        let result = alu.input(nr.clone());
        println!("input {} result: {:?}", nr, result);
        if result.z == 0 {
            println!("found valid model! {}", nr);
        }
    }
    Ok(())
}
