use std::f64;

use time::precise_time_ns;

use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::thread;

use rand;
use rand::ThreadRng;

use wg;

pub fn benchmark(tasks: usize) {

    let mut terminate_senders = Vec::<Sender<u32>>::new(); // ts_
    let mut termination_receivers = Vec::<Receiver<u32>>::new(); // tr_
    let mut completion_receivers = Vec::<Receiver<u32>>::new(); // c_

    // create threads, and store channel pipes in the respective vectors
    create_threads(tasks, &mut terminate_senders, &mut termination_receivers, &mut completion_receivers);

    // it is easier to keep data for calculations here
    // otherwise they would need to passed by reference
    // into the next function

    // calculations

    // samples used for statistics calculations
    let mut samples = Vec::with_capacity(10000);

    // phase of benchmark
    let mut phase = 1u32;

    let mut total_games = 0u64;
    let start_time = precise_time_ns();
    let mut current_time;
    let mut test_duration:f64;

    // 1 minute in nanoseconds
    let prime_time = 60000000000;
    let maximum_tests = 240u32;

    let display_frequency = 50000000u64;
    let sample_frequency =   5000000u64;

    let mut last_display_time = 0u64;
    let mut last_sample_time = 0u64;

    let ms = 1000000u64;
    let ns = 1000000000u64;

    let mut tests = 0;

    let mut elapsed_time;
    let mut test_time = 0u64;

    let mut speed:f64;
    let mut speed_v:f64;
    let mut rate:f64;

    let mut test_started = false;

    let mut maximum_speed_v:f64 = 0f64;

    let mut mean;
    let mut stdev;
    let mut cov = 0f64;

    // the monitor loop collects the total game count
    // and the elasped time so far
    println!("\n{}. prime time has started", phase);
    'monitor: loop {


        total_games = total_games + get_games(&completion_receivers);

        // time calculations
        current_time = precise_time_ns();
        elapsed_time = current_time - start_time;


        // NANOSECONDS per GAME
        rate = elapsed_time as f64 / total_games as f64;

        // GAMES per NANOSECOND
        speed = 1f64 / rate as f64;
        speed_v = speed * ms as f64;

        if maximum_speed_v < speed_v {
            maximum_speed_v = speed_v;
        }

        if !test_started && elapsed_time >= prime_time {
            // phase 1: priming phase
            test_started = true;
            println!("\n{}. prime time has has ended", phase);
            phase = 2;
            println!("\n{}. stability testing has started", phase);
        } else if test_started && elapsed_time >= test_time {
            // phase 2: stability phase
            // calculate statistics
            mean = get_mean(&samples);
            stdev = get_standard_deviation(&samples, mean);
            cov = get_coefficient_of_variation(mean, stdev);
            // cov is a ratio
            // when cov <= 1, stdev is ~1% of the mean
            if cov <= 1f64 || tests >= maximum_tests {
                println!("\n{}. stability testing has ended", phase);
                break 'monitor;
            } else {
                // each test has 1 second duration
                test_duration = 1f64;
                test_time = elapsed_time + (test_duration * ns as f64) as u64;
            }
            tests = tests + 1;
        }

        // sample: save a sample of the current speed_v
        if (current_time - last_sample_time) > sample_frequency {
            last_sample_time = current_time;
            samples.push(speed_v);
        }

        // display: update the display enough time has elapsed
        if  (current_time - last_display_time) > display_frequency {

            last_display_time = current_time;

            if phase == 1 {
                // during phase 1
                backprint(
                        format!("{}. et = {}s; g = {}; s = {sv:.5} g/ms;\t",
                            phase,
                            elapsed_time / ns,
                            total_games,
                            sv=speed_v
                        )
                    );
            }
            else {
                // during phase 2
                backprint(
                        format!("{}. et = {}s; g = {}; s = {sv:.5} g/ms; t = {t}; v = {cov:.2}%; \t",
                            phase,
                            elapsed_time / ns,
                            total_games,
                            sv=speed_v,
                            t = tests,
                            cov = (1f64/cov)*100 as f64
                        )
                    );
            }

        }

    }


    let threads_stopped = stop_threads(tasks, &mut phase, &mut terminate_senders, &mut termination_receivers);
    println!("\n{}. {} tasks stopped", phase, threads_stopped);

    // calculate final statistics
    let mean = get_mean(&samples);
    let stdev = get_standard_deviation(&samples, mean);
    let coefficient_of_variation = get_coefficient_of_variation(mean, stdev);

    // show results
    println!("\n---\n");

    println!("Samples: {} collected", samples.len());
    println!("Mean: {mean:.5}", mean = mean);
    println!("Standard Deviation: {stdev:.5}", stdev = stdev);
    println!("Coefficient of Variation: {cov:.5}", cov = coefficient_of_variation);
    println!("Coefficient of Variation: {cov:.2}%", cov = ((1f64/cov)*100 as f64));
    println!("Maximum Speed: {sv:.5}", sv = maximum_speed_v);

    println!("\n---\n");

    println!("Threads: {}", tasks);
    println!("Speed: {sv:.5}", sv = speed_v);
    println!("Total Games: {}", total_games);
    println!("Elapsed Time: {} nanoseconds; {} seconds", elapsed_time, elapsed_time / ns);

    println!("\nScore: {}\n", f64::round(speed_v));
}

fn create_threads(tasks: usize, ts: &mut Vec<Sender<u32>>, tr: &mut Vec<Receiver<u32>>, c: &mut Vec<Receiver<u32>>) {

    for i in 0..tasks {

        let (c_tx, c_rx): (Sender<u32>, Receiver<u32>) = channel();
        let (ts_tx, ts_rx): (Sender<u32>, Receiver<u32>) = channel();
        let (tr_tx, tr_rx): (Sender<u32>, Receiver<u32>) = channel();

        ts.push(ts_tx);
        tr.push(tr_rx);
        c.push(c_rx);

        thread::spawn(move || {
            let task_id = i;
            // tight loop

            let mut rng = rand::thread_rng();

            loop {
                // the entire point of this: run the wargame
                wg::game(&mut rng);
                // completion gets incremented
                let _ = c_tx.send(1);
                // then the termination signal is checked, and if is available, loop is broken
                let r = ts_rx.try_recv();
                match r {
                    Ok(r) => {if r == 1 {break;}}
                    Err(_) => {}
                }
            }
            // the termination success signal is sent
            let _ = tr_tx.send(task_id as u32);
        });
    }
}

fn stop_threads(tasks: usize, phase: &mut u32, ts: &mut Vec<Sender<u32>>, tr: &mut Vec<Receiver<u32>>) -> usize {
    for s in ts.iter() {
        let _ = s.send(1);
    }
    let mut end_collection:usize = 0;
    'end:loop {
        for r in tr.iter() {
            let rv: usize = match r.recv() {
                Ok(_) => 1,
                Err(_) => 0
            };
            end_collection = end_collection + rv;
        }
        if end_collection == tasks {
            *phase = 3;
            break 'end;
        }
    }
    return end_collection;
}

fn backprint(s: String) {
    print!("\r{}", s);
}

fn get_games(crx: &[Receiver<u32>]) -> u64 {
    let mut total = 0;
    for i in crx.iter() {
        let r = match i.try_recv() {
            Ok(x) => x,
            Err(_) => 0
        };
        total = total + r as u64;
    }
    return total;
}

fn get_coefficient_of_variation(mean: f64, stdev: f64) -> f64 {
    return (stdev / mean) * 100 as f64;
}

fn get_standard_deviation(samples: &[f64], mean: f64) -> f64 {
    let mut total_stdev = 0f64;
    for s in samples.iter() {
        total_stdev = total_stdev + (s - mean).powi(2);
    }
    let stdev = (total_stdev / samples.len() as f64).sqrt();
    return stdev;
}

fn get_mean(samples: &[f64]) -> f64 {
    let mut total_mean:f64 = 0f64;
    for s in samples.iter() {
        total_mean = total_mean + s;
    }
    let mean = total_mean / samples.len() as f64;
    return mean;
}
