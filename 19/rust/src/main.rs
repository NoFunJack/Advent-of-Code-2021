use std::env;
use std::fs;

use beacon_scanner::cloud::Cloud;
use beacon_scanner::scanner::man_dist;
use beacon_scanner::scanner::Beacon;
use beacon_scanner::scanner::Scanner;

fn main() {
    let mut scanner = load_scanner();
    let mut cloud = Cloud::new(scanner.swap_remove(0));
    println!("cloud starts with {} beacons", cloud.beacons.len());

    while !scanner.is_empty() {
        println!("{} scanner unmachted", scanner.len());
        let mut to_remove = None;
        for (i, scan) in scanner.iter().enumerate() {
            if cloud.add_scanner(scan).is_ok() {
                to_remove = Some(i);
                break;
            }
        }

        println!("cloud knows {} beacons", cloud.beacons.len());

        match to_remove {
            Some(i) => {
                scanner.swap_remove(i);
            }
            None => panic!("{} Scanner left, none fit", scanner.len()),
        }
    }

    let pos_copy = cloud.scanner_pos.clone();

    let max_dist = cloud
        .scanner_pos
        .iter()
        .map(|t| pos_copy.iter().map(|c| man_dist(*t, *c)).max())
        .max()
        .flatten()
        .unwrap();

    println!("max scanner dist: {}", max_dist);
}

fn load_scanner() -> Vec<Scanner> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let scanner_str: Vec<&str> = contents.split("\n\n").collect();
    let mut scanner: Vec<Scanner> = Vec::new();

    for scan in scanner_str {
        let beacons: Vec<Beacon> = scan
            .split('\n')
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let nums: Vec<i32> = s.split(',').map(|n| n.parse().unwrap()).collect();
                (nums[0], nums[1], nums[2])
            })
            .map(|tl| Beacon::new(tl.0, tl.1, tl.2))
            .collect();

        scanner.push(Scanner::new(beacons));
    }

    scanner
}
