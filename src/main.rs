
extern crate time;

use std::os;
use std::fmt;
use std::rand::{task_rng, Rng};
use time::precise_time_ns;
use std::io;
use std::thread::Thread;

pub mod wg;

fn monitor(tasks: uint) {
	// how many tasks should we run
	// i.e. the level of concurrency

	let mut terminate_senders = Vec::<Sender<uint>>::new();
	let mut termination_receivers = Vec::<Receiver<uint>>::new();
	let mut completion_receivers = Vec::<Receiver<uint>>::new();

	for i in range(0, tasks) {

		// bind various channel ends to the arrays above
		// or into the closure of proc() below for
		// each task's use
		let (tx, rx): (Sender<uint>, Receiver<uint>) = channel();
		let (ctx, crx): (Sender<uint>, Receiver<uint>) = channel();
		let (ttx, trx): (Sender<uint>, Receiver<uint>) = channel();
		terminate_senders.push(ctx);
		termination_receivers.push(trx);
		completion_receivers.push(rx);

		// this starts the task,
		// which may or may not be a thread
		let thread_handle = Thread::spawn(move || {
			let task_id = i;

			// infinitely loop the games,
			// second back the iteration count
			loop {

				wg::game(); // simulation of game
				tx.send(1);
				

				let result = crx.try_recv();
				match result {
					Ok(r) => {
						if r == 1 {
							// break out of this loop
							break;
						}
					},
					Err(e) => {
						// there are no errors here
					}
				}

			}
			// send the termination signal
			ttx.send(1);
		});

		thread_handle.detach();

	}

	let mut phase = 1u;

	let mut total_games = 0u64;
	let start_time = precise_time_ns();
	let mut current_time = precise_time_ns();
	let mut test_duration = 0f64;

	// 1 minute in nanoseconds
	let prime_time = 60000000000;
	let maximum_tests = 100u;
	let percent_variation = 0.0001f64;
	let display_frequency = 50000000u64;

	let ms = 1000000u64;
	let ns = 1000000000u64;

	let mut tests = 0;

	let mut elapsed_time = 0;
	let mut last_time = 0u64;
	let mut test_time = 0u64;

	let mut speed = 0f64;
	let mut speed_v = 0f64;
	let mut rate = 0f64;

	let mut rate_low = 0f64;
	let mut rate_high = 0f64;
	let mut percent_rate = 0f64;

	let mut test_started = false;

	// the monitor loop collects the total game count
	// and the elasped time so far
	println!("\n{}. prime time has begun", phase);
	'monitor: loop {
		
		/*
			Query each counter
		*/
		for i in range(0, tasks) {
			let received = match completion_receivers[i].try_recv() {
				Ok(x) => x,
				Err(_) => 0
			};
			total_games = total_games + received as u64;
		}

		// time calculations
		current_time = precise_time_ns();
		elapsed_time = current_time - start_time;

		
		// NANOSECONDS per GAME
		rate = elapsed_time as f64 / total_games as f64;

		// GAMES per NANOSECOND
		speed = 1f64 / rate as f64;
		speed_v = speed * ms as f64;

		// the priming phase
		if !test_started && elapsed_time >= prime_time {
			test_started = true;
			println!("\n{}. prime time has has ended", phase);
			phase = 2;
			println!("\n{}. stability testing has begun", phase);
		} else if test_started && elapsed_time >= test_time {

			// testing phase
			if rate_low < rate && rate < rate_high || tests >= maximum_tests {
				// end the monitor infinite loop
				println!("\n{}. stability testing has ended", phase);
				break 'monitor;
			} else {
				// calculate the details for the next testing phase
				
				// ceil does not need the +1 because 0..1 will always yeild 1
				test_duration = std::num::Float::ceil(speed_v);
				test_time = elapsed_time + (test_duration * ns as f64) as u64;
				
				percent_rate = rate * percent_variation;

				rate_low = rate - percent_rate;
				rate_high = rate + percent_rate;
				tests = tests + 1;
			}

		}

		/*
			If the time between prints is .5s, print out an updated string.
		*/
		if  (current_time - last_time) > display_frequency {
			last_time = current_time;
			if phase == 1 {
				wg::backprint(format!("{}. et = {}s; g = {}; s = {} g/ms;\t", 
					phase, elapsed_time / ns, total_games, speed_v));
			} 
		else {
			wg::backprint(format!("{}. et = {}s; g = {}; s = {} g/ms; t = {}; \t", 
				phase, elapsed_time / ns, total_games, speed_v, format!("{} @ {}s", tests, test_duration)));
		}

		}

	}

	// cleanup after 'monitor has ended
	for i in range(0, tasks) {
		terminate_senders[i].send(1);
	}
	let mut end_collection = 0u;
	'end:loop {
		for i in range(0, tasks) {
			end_collection = end_collection + termination_receivers[i].recv();
		}
		if end_collection == tasks {
			phase = 3;
			println!("\n{}. {} tasks stopped", phase, end_collection);
			break 'end;
		}
	}

	// show results
	println!("Speed: {}", speed_v);
	println!("Total Games: {}", total_games);
	println!("Elapsed Time: {} nanoseconds; {} seconds", elapsed_time, elapsed_time / ns);
	println!("Score: {}", std::num::Float::round(speed_v));


}


fn main() {

	/*
		Grab the optional cli argument.
	*/
	let args = os::args();
	let tasks:uint = match args.len() {
		2 => match args[1].as_slice().trim().parse() {
			Some(x) => x,
			None => 1
		},
		_ => 1
	};

	println!("settings: tasks = {}", tasks);

	monitor(tasks);
}
