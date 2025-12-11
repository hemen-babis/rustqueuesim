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

Here is a clean, simple **example output section** you can paste directly into your README.
It shows what a typical run of your simulation looks like — readable, professional, and totally accurate.

---

# Example Output

Below is an example of the simulation running with the default settings:

```
$ cargo run

RustQueueSim — CS-423/523 Queue Simulation
------------------------------------------------
Total time steps : 1000
Arrival prob     : 0.300
Service time     : 1 to 5 steps
RNG seed         : 42
------------------------------------------------

Simulation finished.
Time steps          : 1000
Jobs arrived        : 296
Jobs completed      : 292
Max queue length    : 9
Average wait time   : 8.199
Avg system time     : 11.175
Server utilization  : 0.577
```

And here is an example using custom parameters:

```
$ cargo run -- --time=5000 --arrival=0.25 --min_service=1 --max_service=4 --seed=123

RustQueueSim — CS-423/523 Queue Simulation
------------------------------------------------
Total time steps : 5000
Arrival prob     : 0.250
Service time     : 1 to 4 steps
RNG seed         : 123
------------------------------------------------

Simulation finished.
Time steps          : 5000
Jobs arrived        : 1295
Jobs completed      : 1295
Max queue length    : 7
Average wait time   : 1.958
Avg system time     : 4.453
Server utilization  : 0.387
```

These outputs show how changing the arrival rate, service time range, and simulation length affects queue length, server utilization, and job wait times in the system.

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

## AI Use Disclosure

I used ChatGPT to help me with a few non-coding parts of this project. Specifically, I used it to:

- brainstorm the initial module layout after I wrote my proposal

- help me rewrite a few comments in clearer language

- double-check my Rust formatting, clippy expectations, and general project organization

I wrote all the actual Rust code myself and made sure I fully understood the logic in each file.
All final code, testing, and debugging decisions were my own.
