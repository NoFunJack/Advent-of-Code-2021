use std::io::prelude::*;
use std::{env, io};

use crate::lib::check;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let stdin = io::stdin();
    let mut sum_corrupted = 0;
    let mut list_autocomplete = Vec::new();

    for line in stdin.lock().lines() {
        let result = check(line.as_ref().unwrap().clone());
        println!("input {} score {:?}", line.unwrap(), result);
        match result {
            lib::CheckResult::Corrupted(s) => sum_corrupted += s,
            lib::CheckResult::Inclomplete(s, _) => list_autocomplete.push(s),
        }
    }

    list_autocomplete.sort();
    println!("Final sum corrupted: {}", sum_corrupted);
    println!(
        "DEBUG: {}, {} ",
        list_autocomplete.len(),
        list_autocomplete.len() / 2
    );
    println!(
        "Final middle autocomplete: {:#?}\n {}",
        list_autocomplete,
        list_autocomplete.get(list_autocomplete.len() / 2).unwrap()
    );
}
