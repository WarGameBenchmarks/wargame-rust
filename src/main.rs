#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;
extern crate rand;

pub mod wg;
pub mod benchmark;

use std::env;

fn main() {
	env_logger::init();

	/*
		Grab the optional cli argument.
	*/
	let args: Vec<String> = env::args().collect();

	let threads:usize;
	let multiplier:f64;

	let _ = match args.len() {
		3 => {
			threads = match args[1].trim().parse() {
				Ok(x) => x,
				Err(_) => 1
			};
			multiplier = match args[2].trim().parse() {
				Ok(x) => x,
				Err(_) => 1.00
			};
		},
		2 => {
			threads = match args[1].trim().parse() {
				Ok(x) => x,
				Err(_) => 1
			};
			multiplier = 1.00;
		},
		_ => {
			threads = 1;
			multiplier = 1.00;
		}
	};

	println!("WarGame Rust");

	println!("settings: threads = {}; multiplier = {:.2}\n", threads, multiplier);

	benchmark::benchmark(threads, multiplier.abs());
}
