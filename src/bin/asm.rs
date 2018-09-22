extern crate myopic;

use myopic::parse_tr_unit;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: asm FILE");
        exit(2);
    }

    let mut f = File::open(&args[1]).unwrap();
    let input = {
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();
        input
    };

    parse_tr_unit(&input).unwrap();
}
