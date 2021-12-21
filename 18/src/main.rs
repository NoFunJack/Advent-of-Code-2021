use std::env;
use std::io;
use std::io::prelude::*;

use snailfish::snailfish::Number;

fn main() {
    let args: Vec<String> = env::args().collect();

    let stdin = io::stdin();
    let contents = stdin.lock().lines().next().unwrap().unwrap();

    let n = Number::phase(contents);

    println!("{:#?}", n);
}
