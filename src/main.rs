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

	let tasks:usize = match args.len() {
		2 => match args[1].trim().parse() {
			Ok(x) => x,
			Err(_) => 1
		},
		_ => 1
	};

	println!("settings: tasks = {}", tasks);

	benchmark::benchmark(tasks);
}
