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
    for nr in ModelNumbers::new(vec![
        Step::new(11, 14),
        Step::new(13, 8),
        Step::new(11, 4),
        Step::new(10, 10),
        Step::new(-3, 14),
        Step::new(-4, 10),
        Step::new(12, 4),
        Step::new(-8, 14),
        Step::new(-3, 1),
        Step::new(-12, 6),
        Step::new(14, 0),
        Step::new(-6, 9),
        Step::new(11, 13),
        Step::new(-12, 12),
    ]) {
        let result = alu.input(nr.clone());
        //println!("input {} result: {:?}", nr, result);
        if result.z == 0 {
            println!("found valid model! {}", nr);
            //return Ok(());
        }
    }
    Ok(())
}
