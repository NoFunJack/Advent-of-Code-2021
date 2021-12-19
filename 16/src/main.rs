use std::env;
use std::io;
use std::io::prelude::*;

use buoyancy_interchange_transmission_system::bitsreader::BitsStream;
use buoyancy_interchange_transmission_system::packet::Packet;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let stdin = io::stdin();
    let contents = stdin.lock().lines().next().unwrap().unwrap();

    let p = Packet::new(&mut BitsStream::new(contents.to_string()), &mut None);

    println!("{:#?}", p);
    println!("Sum of Versions: {}", p.get_version_sum());
    println!("Packet eval: {}", p.eval());
}
