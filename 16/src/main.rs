use std::env;
use std::fs::File;
use std::io::prelude::*;

use buoyancy_interchange_transmission_system::bitsreader::BitsStream;
use buoyancy_interchange_transmission_system::packet::Packet;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut file = File::open(args.get(1).unwrap()).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    println!("{}", contents);
    let p = Packet::new(&mut BitsStream::new(contents.trim().to_string()), &mut None);

    println!("{:#?}", p);
    println!("Sum of Versions: {}", p.get_version_sum());
}
