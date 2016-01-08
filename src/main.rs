#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate rand;

pub mod wg;
pub mod benchmark;

use std::env;

fn main() {
	env_logger::init().unwrap();

	/*
		Grab the optional cli argument.
	*/
	let args: Vec<String> = env::args().collect();

	let threads:usize = match args.len() {
		2 => match args[1].trim().parse() {
			Ok(x) => x,
			Err(_) => 1
		},
		_ => 1
	};

	let multiplier:f64 = match args.len() {
		3 => match args[2].trim().parse() {
			Ok(x) => x,
			Err(_) => 1.00
		},
		_ => 1.00
	};

	println!("WarGame Rust");

	println!("settings: threads = {}; multiplier = {:.2}\n", threads, multiplier);

	benchmark::benchmark(threads, multiplier.abs());
}
