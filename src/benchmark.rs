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

    let mut phase = 1u32;

    let mut total_games = 0u64;
    let start_time = precise_time_ns();
    let mut current_time;
    let mut test_duration = 0f64;

    // 1 minute in nanoseconds
    let prime_time = 60000000000;
    let maximum_tests = 100u32;
    let percent_variation = 0.0001f64;
    let display_frequency = 50000000u64;

    let ms = 1000000u64;
    let ns = 1000000000u64;

    let mut tests = 0;

    let mut elapsed_time;
    let mut last_time = 0u64;
    let mut test_time = 0u64;

    let mut speed:f64;
    let mut speed_v:f64;
    let mut rate:f64;

    let mut rate_low = 0f64;
    let mut rate_high = 0f64;
    let mut percent_rate:f64;

    let mut test_started = false;

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
                Err(_) => 00
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
            println!("\n{}. stability testing has started", phase);
        } else if test_started && elapsed_time >= test_time {

            // testing phase
            if rate_low < rate && rate < rate_high || tests >= maximum_tests {
                // end the monitor infinite loop
                println!("\n{}. stability testing has ended", phase);
                break 'monitor;
            } else {
                // calculate the details for the next testing phase
                
                // ceil does not need the +1 because 0..1 will always yeild 1
                test_duration = f64::ceil(f64::sqrt(speed_v));
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
            backprint(
                    format!("{}. et = {}s; g = {}; s = {sv:.5} g/ms; t = {t}; \t", 
                        phase,
                        elapsed_time / ns,
                        total_games, 
                        sv=speed_v, 
                        t = format!(
                            "{} @ {}s",
                                tests,
                                test_duration
                            )
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
    println!("Threads: {}", tasks);
    println!("Speed: {sv:.5}", sv = speed_v);
    println!("Total Games: {}", total_games);
    println!("Elapsed Time: {} nanoseconds; {} seconds", elapsed_time, elapsed_time / ns);
    println!("Score: {}", f64::round(speed_v));

}
