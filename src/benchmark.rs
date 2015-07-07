use std::f64;

use time::precise_time_ns;

use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::thread;

use wg;

fn backprint(s: String) {
    print!("\r{}", s);
}

pub fn benchmark(tasks: usize) {

    let mut terminate_senders = Vec::<Sender<u32>>::new(); // ts_
    let mut termination_receivers = Vec::<Receiver<u32>>::new(); // tr_
    let mut completion_receivers = Vec::<Receiver<u32>>::new(); // c_


    // TODO: split the thread creation into its own method

    for i in 0..tasks {

        let (c_tx, c_rx): (Sender<u32>, Receiver<u32>) = channel();
        let (ts_tx, ts_rx): (Sender<u32>, Receiver<u32>) = channel();
        let (tr_tx, tr_rx): (Sender<u32>, Receiver<u32>) = channel();

        terminate_senders.push(ts_tx);
        termination_receivers.push(tr_rx);
        completion_receivers.push(c_rx);

        thread::spawn(move || {

            let task_id = i;

            loop {
                wg::game();
                let _ = c_tx.send(1);


                let r = ts_rx.try_recv();
                match r {
                    Ok(r) => {if r == 1 {break;}}
                    Err(_) => {}
                }

            }

            let _ = tr_tx.send(task_id as u32);

        });


    }

    // TODO: split the calculations into its own method

    let mut samples = Vec::with_capacity(10000);

    let mut phase = 1u32;

    let mut total_games = 0u64;
    let start_time = precise_time_ns();
    let mut current_time;
    let mut test_duration:f64;

    // 1 minute in nanoseconds
    // let prime_time = 60000000000;
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

        /*
            Query each counter
        */
        for i in 0..tasks {
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

        if maximum_speed_v < speed_v {
            maximum_speed_v = speed_v;
        }

        // the priming phase
        if !test_started && elapsed_time >= prime_time {

            test_started = true;
            println!("\n{}. prime time has has ended", phase);
            phase = 2;
            println!("\n{}. stability testing has started", phase);

        } else if test_started && elapsed_time >= test_time {

            mean = get_mean(&samples);
            stdev = get_standard_deviation(&samples, mean);
            cov = get_coefficient_of_variation(mean, stdev);

            if cov <= 1f64 || tests >= maximum_tests {
               println!("\n{}. stability testing has ended", phase);
               break 'monitor;
            } else {
               test_duration = 1f64;
               test_time = elapsed_time + (test_duration * ns as f64) as u64;
            }

            tests = tests + 1;
        }

        if (current_time - last_sample_time) > sample_frequency {
            last_sample_time = current_time;
            samples.push(speed_v);
        }

        /*
            If the time between prints is .5s, print out an updated string.
        */
        if  (current_time - last_display_time) > display_frequency {
            last_display_time = current_time;
            if phase == 1 {
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
            // backprint(
            //         format!("{}. et = {}s; g = {}; s = {sv:.5} g/ms; t = {t}; \t",
            //             phase,
            //             elapsed_time / ns,
            //             total_games,
            //             sv=speed_v,
            //             t = format!(
            //                 "{} @ {}s",
            //                     tests,
            //                     test_duration
            //                 )
            //         )
            //     );
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

    // TODO: split the thread cleanup into its own method

    for i in 0..tasks {
        let _ = terminate_senders[i].send(1);
    }
    let mut end_collection:usize = 0;
    'end:loop {
        for i in 0..tasks {
            let recv_value: usize = match termination_receivers[i].recv() {
                Ok(_) => 1,
                Err(_) => 0
            };
            end_collection = end_collection + recv_value;
        }
        if end_collection == tasks {
            phase = 3;
            println!("\n{}. {} tasks stopped", phase, end_collection);
            break 'end;
        }
    }

    // show results
    println!("\n\n\t\t----\n");


    let mean = get_mean(&samples);

    println!("Samples: {} collected", samples.len());
    println!("Mean: {}", mean);

    let stdev = get_standard_deviation(&samples, mean);

    println!("Standard Deviation: {}", stdev);

    let coefficient_of_variation = get_coefficient_of_variation(mean, stdev);

    println!("Coefficient of Variation: {}%", coefficient_of_variation);

    println!("\n\n\t\t----\n");
    println!("Threads: {}", tasks);
    println!("Maximum Speed: {sv:.5}", sv = maximum_speed_v);
    println!("Speed: {sv:.5}", sv = speed_v);
    println!("Total Games: {}", total_games);
    println!("Elapsed Time: {} nanoseconds; {} seconds", elapsed_time, elapsed_time / ns);
    println!("Score: {}", f64::round(speed_v));

}

fn get_coefficient_of_variation(mean: f64, stdev: f64) -> f64 {
    return (stdev / mean) * 100 as f64;
}

fn get_standard_deviation(samples: &[f64], mean: f64) -> f64 {
    let mut total_stdev = 0f64;
    for i in 0..samples.len() {
        total_stdev = total_stdev + (samples[i] - mean).powi(2);
    }
    let stdev = (total_stdev / samples.len() as f64).sqrt();
    return stdev;
}

fn get_mean(samples: &[f64]) -> f64 {
    let mut total_mean:f64 = 0f64;
    for i in 0..samples.len() {
        total_mean = total_mean + samples[i];
    }
    let mean = total_mean / samples.len() as f64;
    return mean;
}
