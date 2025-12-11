# RustQueueSim

RustQueueSim is my final project for **CS-423/523 (Advanced Programming in Rust)**.
I built a small, instrumented queueing simulation that shows how jobs arrive, wait in a FIFO queue, and get processed by a single server over time. The goal was to create a clean Rust project that feels like a real simulation system, with good metrics and modular code.

## What the simulation does

Very simple idea:

* Time moves forward one step at a time
* Jobs show up randomly based on a probability
* Each job needs a certain amount of service time
* If the server is free, it works on the next job
* Otherwise the job waits in line
* I track metrics the whole time

The program records:

* How many jobs arrived and finished
* How long jobs waited
* Total time jobs spent in the system
* Server utilization (how often it's busy)
* Max queue length (peak congestion)

## Project structure

I kept the project organized into small modules so it’s easy to understand:

* `job.rs` — what a job is (arrival time, service time, etc.)
* `queue.rs` — simple FIFO queue wrapper
* `server.rs` — server that can be idle or busy on one job
* `metrics.rs` — all the stats I track
* `sim.rs` — the actual simulation engine
* `main.rs` — CLI runner

## Running it

Build:

```bash
cargo build
```

Run with defaults:

```bash
cargo run
```

Try custom settings:

```bash
cargo run -- --time=5000 --arrival=0.25 --min_service=1 --max_service=4 --seed=123
```

## Testing & linting

I wrote unit tests for:

* job timing logic
* FIFO queue behavior
* server state transitions
* basic metric calculations

Commands I used:

```bash
cargo test
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```



Just tell me!
