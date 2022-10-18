use std::io::prelude::*;
use std::{env, io};

use crate::lib::check;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines() {
        let result = check(line.as_ref().unwrap().clone());
        println!("input {} score {}", line.unwrap(), result);
        sum += result;
    }

    println!("Final sum: {}", sum);
}

