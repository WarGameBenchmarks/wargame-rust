use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use std::thread;

use std::io::{self, Write};

use std::collections::HashMap;

use rand;

use std::time::Instant;

use wg;

const MS: u128 = 1_000_000;
const NS: u128 = 1_000_000_000;

pub fn benchmark(threads: usize, multiplier: f64) {
    // these are communication channels
    let mut terminate_senders = Vec::<Sender<u32>>::new(); // ts_
    let mut termination_receivers = Vec::<Receiver<u32>>::new(); // tr_
    let mut completion_receivers = Vec::<Receiver<u32>>::new(); // c_

    // create threads, and store channel pipes in the respective vectors
    create_threads(
        threads,
        &mut terminate_senders,
        &mut termination_receivers,
        &mut completion_receivers,
    );

    // 1/10 of a second
    const DISPLAY_FREQUENCY: u128 = NS / 10;

    // 1/200 of a second
    const SAMPLE_FREQUENCY: u128 = NS / 200;

    // 10 seconds
    let mut prime_time: u128 = 10000000000;
    // 50 seconds
    let mut sample_time: u128 = 50000000000;

    if multiplier != 1.00 {
        prime_time = (prime_time as f64 * multiplier) as u128;
        sample_time = (sample_time as f64 * multiplier) as u128;
    }

    let end_time: u128 = prime_time + sample_time;

    let sample_size: u128 = sample_time / SAMPLE_FREQUENCY;

    // samples used for statistics calculations
    let mut samples = Vec::with_capacity(sample_size as usize);

    let start_time: Instant = Instant::now();
    let mut current_time: Instant;
    let mut elapsed_time: u128;

    let mut last_display_time: Instant = start_time;
    let mut last_sample_time: Instant = start_time;

    let mut phase: u64 = 1;

    let mut total_games: u64 = 1;

    let mut speed: f64;

    // min and max are forced to be
    // zero here, but they are properly set to the
    // initial speed sample value when phase 1 ends
    let mut maximum_speed: f64 = 0.0;
    let mut minimum_speed: f64 = 0.0;

    'monitor: loop {
        total_games = total_games + get_games(&completion_receivers);

        // time calculations
        current_time = Instant::now();
        elapsed_time = current_time.duration_since(start_time).as_nanos();

        speed = total_games as f64 / elapsed_time as f64;

        if phase == 1 && elapsed_time >= prime_time {
            phase = 2;
            // proper setting of min/max
            maximum_speed = speed;
            minimum_speed = speed;
        } else if phase == 2 {
            if maximum_speed < speed {
                maximum_speed = speed;
            }
            if minimum_speed > speed {
                minimum_speed = speed;
            }

            if elapsed_time >= end_time {
                phase = 3;
            }
        } else if phase == 4 {
            break 'monitor;
        }

        if phase == 2
            && (current_time.duration_since(last_sample_time).as_nanos()) > SAMPLE_FREQUENCY
        {
            last_sample_time = current_time;
            samples.push(speed);
        }

        if (current_time.duration_since(last_display_time).as_nanos()) > DISPLAY_FREQUENCY {
            last_display_time = current_time;

            if phase == 1 {
                print!(
                    "\r{}. priming | et = {}s; g = {}; s = {:.5} g/ms; \t",
                    phase,
                    elapsed_time / NS,
                    total_games,
                    speed * MS as f64
                )
            } else if phase == 2 {
                print!(
                    "\r{}. sampling | et = {}s; g = {}; s = {:.5} g/ms; t = {}; \t",
                    phase,
                    elapsed_time / NS,
                    total_games,
                    speed * MS as f64,
                    samples.len()
                )
            } else if phase == 3 {
                phase = 4;
                // intentionally blank line
                print!(
                    "\r{}. done                                                                 \t",
                    phase
                )
            }

            // rust is line buffered, so force output
            io::stdout().flush().unwrap();
        }
    }

    // Rust requires the threads be manually ended, and the channels specifically closed.
    let _ = stop_threads(threads, &mut terminate_senders, &mut termination_receivers);

    // calculations

    const T_SCORE: f64 = 3.291; // 99.9% t-score
    const ONE_PERCENT: f64 = 0.01; // 1%
    const TEN_PERCENT: f64 = 0.1; // 10%

    let mean: f64 = get_mean(&samples);
    let median: f64 = get_median(&samples);
    let stdev: f64 = get_standard_deviation(&samples, mean);
    let cov: f64 = get_coefficient_of_variation(mean, stdev);

    let mean_median_delta: f64 = (median - mean).abs();
    let mm_lower: f64 = median.min(mean);
    let mm_upper: f64 = median.max(mean);

    let min_max_delta: f64 = maximum_speed - minimum_speed;
    let max_ten_percent: f64 = maximum_speed * TEN_PERCENT;

    let one_sigma_lower: f64 = mean - stdev;
    let one_sigma_upper: f64 = mean + stdev;
    let one_sigma_delta: f64 = one_sigma_upper - one_sigma_lower;

    let ci_lower: f64 = mean - (T_SCORE * (stdev / (samples.len() as f64).sqrt()));
    let ci_upper: f64 = mean + (T_SCORE * (stdev / (samples.len() as f64).sqrt()));
    let ci_delta: f64 = ci_upper - ci_lower;

    let mut criteria = HashMap::new();

    criteria.insert("1", mean_median_delta < stdev);
    criteria.insert("2", min_max_delta < max_ten_percent);
    criteria.insert("3", cov < ONE_PERCENT);
    criteria.insert("4", one_sigma_lower < speed && speed < one_sigma_upper);
    criteria.insert("5", ci_lower < speed && speed < ci_upper);

    // show results
    println!("\n---\n");

    println!("Samples: {:9}", samples.len());
    println!("Mean:\t {:9.5}", toms(mean));
    println!("Median:\t {:9.5}", toms(median));
    println!("S.D.:\t {:9.5}", toms(stdev));
    println!("C.O.V.:\t {:9.5}", cov);

    println!("---");

    println!(
        "Min-Max:\t < {:9.5} - {:9.5} > Δ {:9.5}",
        toms(minimum_speed),
        toms(maximum_speed),
        toms(min_max_delta)
    );

    println!(
        "1-σ:\t\t < {:9.5} - {:9.5} > Δ {:9.5}",
        toms(one_sigma_lower),
        toms(one_sigma_upper),
        toms(one_sigma_delta)
    );

    println!(
        "μ-Median:\t < {:9.5} - {:9.5} > Δ {:9.5}",
        toms(mm_lower),
        toms(mm_upper),
        toms(mean_median_delta)
    );

    println!(
        "99.9%% CI:\t < {:9.5} - {:9.5} > Δ {:9.5}",
        toms(ci_lower),
        toms(ci_upper),
        toms(ci_delta)
    );

    println!("---");

    println!("Threads: {}", threads);
    println!("Multiplier: {:.2}", multiplier);
    println!("Speed: {:.5} g/ms", toms(speed));
    println!("Games: {}", total_games);
    println!("Duration: {:.1}s", (elapsed_time as f64 / NS as f64));

    println!("---");

    println!(
        "Rank: ({}/{}) {}",
        rank_passes(&mut criteria),
        criteria.len(),
        rank_letter(&mut criteria)
    );
    println!("Rank Criteria: {}", rank_reason(&mut criteria));

    println!("---");

    println!("Score: {}", toms(speed).round());
}

fn toms(f: f64) -> f64 {
    return f * MS as f64;
}

fn create_threads(
    threads: usize,
    ts: &mut Vec<Sender<u32>>,
    tr: &mut Vec<Receiver<u32>>,
    c: &mut Vec<Receiver<u32>>,
) {
    for i in 0..threads {
        let (c_tx, c_rx): (Sender<u32>, Receiver<u32>) = channel();
        let (ts_tx, ts_rx): (Sender<u32>, Receiver<u32>) = channel();
        let (tr_tx, tr_rx): (Sender<u32>, Receiver<u32>) = channel();

        ts.push(ts_tx);
        tr.push(tr_rx);
        c.push(c_rx);

        thread::spawn(move || {
            let thread_id = i;
            // tight loop

            // make a random generator for this
            // thread only and supply that to each game
            let mut rng = rand::thread_rng();

            loop {
                // the entire point of this: run the wargame
                wg::game(&mut rng);
                // completion gets incremented
                let _ = c_tx.send(1);
                // then the termination signal is checked, and if is available, loop is broken
                let r = ts_rx.try_recv();
                match r {
                    Ok(r) => {
                        if r == 1 {
                            break;
                        }
                    }
                    Err(_) => {}
                }
            }
            // the termination success signal is sent
            let _ = tr_tx.send(thread_id as u32);
        });
    }
}

fn stop_threads(threads: usize, ts: &mut Vec<Sender<u32>>, tr: &mut Vec<Receiver<u32>>) -> usize {
    for s in ts.iter() {
        let _ = s.send(1);
    }
    let mut end_collection: usize = 0;
    'end: loop {
        for r in tr.iter() {
            let rv: usize = match r.recv() {
                Ok(_) => 1,
                Err(_) => 0,
            };
            end_collection = end_collection + rv;
        }
        if end_collection == threads {
            break 'end;
        }
    }
    return end_collection;
}

fn get_games(crx: &[Receiver<u32>]) -> u64 {
    let mut total = 0;
    for i in crx.iter() {
        let r = match i.try_recv() {
            Ok(x) => x,
            Err(_) => 0,
        };
        total = total + r as u64;
    }
    return total;
}

fn rank_passes(criteria: &mut HashMap<&str, bool>) -> usize {
    let mut n: usize = 0;
    for (_, &b) in criteria.iter() {
        if b {
            n = n + 1
        }
    }
    return n;
}

fn rank_letter(criteria: &mut HashMap<&str, bool>) -> String {
    let n: usize = rank_passes(criteria);
    let str = match n {
        5 => "A+",
        4 => "A",
        3 => "B",
        2 => "C",
        1 => "D",
        _ => "F",
    };
    return str.to_string();
}

fn rank_reason(criteria: &mut HashMap<&str, bool>) -> String {
    let reason;
    let passes = rank_passes(criteria);
    if passes == 0 {
        reason = "none".to_string();
    } else {
        let mut ss = Vec::new();
        for (&k, &v) in criteria.iter() {
            if v {
                ss.push(k.clone());
            }
        }
        let joined = ss.join(" | ");
        reason = joined.to_string();
    }
    return reason;
}

fn get_coefficient_of_variation(mean: f64, stdev: f64) -> f64 {
    return stdev / mean;
}

fn get_standard_deviation(samples: &[f64], mean: f64) -> f64 {
    let mut total_stdev = 0f64;
    for s in samples.iter() {
        total_stdev = total_stdev + (s - mean).powi(2);
    }
    let stdev = (total_stdev / (samples.len() - 1) as f64).sqrt();
    return stdev;
}

fn get_median(samples: &[f64]) -> f64 {
    let mut s = samples.to_vec();

    // this is required because core::cmp::Ord is not
    // implemented for f64, which means
    // we need to sort by hand here.
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let length: usize = s.len();
    let median: f64;
    if length % 2 == 0 {
        let a = s[length / 2 - 1];
        let b = s[length / 2 + 2];
        median = (a + b) / 2 as f64;
    } else {
        median = s[length / 2];
    }
    return median;
}

fn get_mean(samples: &[f64]) -> f64 {
    let mut total_mean: f64 = 0f64;
    for s in samples.iter() {
        total_mean = total_mean + s;
    }
    let mean = total_mean / samples.len() as f64;
    return mean;
}
