use std::env;
use std::error::Error;
use std::fs;

mod alu;
use crate::alu::*;

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

struct ModelNumbers {
    // most significant number on the right
    number: [u8; 14],
}

impl ModelNumbers {
    fn new() -> ModelNumbers {
        ModelNumbers { number: [9; 14] }
    }

    fn to_str(&mut self) -> String {
        self.number
            .iter()
            .rev()
            .map(|x| x.to_string())
            .collect::<String>()
    }
}

impl Iterator for ModelNumbers {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.number.iter().all(|&x| x == 0) {
            return None;
        } else {
            for (i, l) in self.number.clone().iter().enumerate() {
                match l {
                    2..=9 => {
                        self.number[i] = l.checked_sub(1).unwrap();
                        return Some(self.to_str());
                    }
                    1 => self.number[i] = 9,
                    _ => panic!("bad number {}", l),
                }
            }
            return None;
        }
    }
}
