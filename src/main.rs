//! Simple CLI wrapper for running the queue simulation with different
//! parameter values. I just did this so I can tweak settings easily
//! when testing.

use rustqueuesim::{SimConfig, Simulator};
use std::env;

fn parse_arg<T: std::str::FromStr>(name: &str, default: T) -> T {
    let key = format!("--{name}=");
    for arg in env::args().skip(1) {
        if let Some(rest) = arg.strip_prefix(&key) {
            if let Ok(parsed) = rest.parse::<T>() {
                return parsed;
            }
        }
    }
    default
}

fn main() {
    let total_time = parse_arg("time", 1_000_u64);
    let arrival_prob = parse_arg("arrival", 0.3_f64);
    let min_service_time = parse_arg("min_service", 1_u64);
    let max_service_time = parse_arg("max_service", 5_u64);
    let seed = parse_arg("seed", 42_u64);

    let config = SimConfig {
        total_time,
        arrival_prob,
        min_service_time,
        max_service_time,
        seed,
    };

    println!("RustQueueSim — CS-423/523 Queue Simulation");
    println!("------------------------------------------------");
    println!("Total time steps : {}", config.total_time);
    println!("Arrival prob     : {:.3}", config.arrival_prob);
    println!(
        "Service time     : {} to {} steps",
        config.min_service_time, config.max_service_time
    );
    println!("RNG seed         : {}", config.seed);
    println!("------------------------------------------------\n");

    let mut sim = Simulator::new(config);
    sim.run();

    let m = sim.metrics;

    println!("Simulation finished.");
    println!("Time steps          : {}", m.time_steps);
    println!("Jobs arrived        : {}", m.total_jobs_arrived);
    println!("Jobs completed      : {}", m.total_jobs_completed);
    println!("Max queue length    : {}", m.max_queue_len);
    println!("Average wait time   : {:.3}", m.avg_wait_time());
    println!("Avg system time     : {:.3}", m.avg_system_time());
    println!("Server utilization  : {:.3}", m.utilization());
}
