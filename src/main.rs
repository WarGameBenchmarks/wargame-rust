
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

	let MS = 1000000u64;
	let NS = 1000000000u64;

	let mut tests = 0;

	let mut elasped_time = 0;
	let mut last_time = 0u64;
	let mut test_time = 0u64;

	let mut speed = 0f64;
	let mut rate = 0f64;

	let mut rate_low = 0f64;
	let mut rate_high = 0f64;
	let mut percent_rate = 0f64;

	let mut test_started = false;

	// the monitor loop collects the total game count
	// and the elasped time so far
	println!("\n{}. prime time has begun", phase); phase = 2;
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
		elasped_time = current_time - start_time;

		
		// NANOSECONDS per GAME
		rate = elasped_time as f64 / total_games as f64;

		// GAMES per NANOSECOND
		speed = 1f64 / rate as f64;
		let speed_v = speed * MS as f64;

		// the priming phase
		if !test_started && elasped_time >= prime_time {
			test_started = true;
			phase = 3;
			println!("\n{}. prime time has is over", phase);
			phase = 4;
		} else if test_started && elasped_time >= test_time {

			// testing phase
			if rate_low < rate && rate < rate_high || tests >= maximum_tests {
				// end the monitor infinite loop
				break 'monitor;
			} else {
				// calculate the details for the next testing phase
				test_duration = speed_v + 1f64;
				test_time = elasped_time + (test_duration * NS as f64) as u64;
				
				percent_rate = rate * percent_variation;

				rate_low = rate - percent_rate;
				rate_high = rate + percent_rate;
				tests = tests + 1;
			}

		}

		if  (current_time - last_time) > 50000000 {
			last_time = current_time;
			wg::backprint(format!("{}. et = {}s, r = {} ms/g; s = {} g/ms; total = {}\t",
			 phase, elasped_time / NS, rate / MS as f64, speed_v, total_games));
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
			phase = 5;
			println!("\n{}. {} tasks stopped", phase, end_collection);
			break 'end;
		}
	}


}

// view the source code on this gist
// https://gist.github.com/ryanmr/46097dc63c1ccf833f52
fn main() {

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
