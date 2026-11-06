use clap::Parser;
use proc_macros::profile;
use std::path::PathBuf;

mod haversine;
mod json;
mod profiler;

#[derive(Parser)]
#[command(
    version,
    about = "Calculate Haversine distances of pairs of coordinates from a JSON file"
)]
struct Args {
    /// JSON file path with coordinates
    input: PathBuf,
    /// Binary file path with expected Haversine distances
    answers: PathBuf,
}

fn main() {
    let measurement_start = profiler::read_tsc();
    let measurement_end = profiler::read_tsc();

    let total_start = profiler::read_tsc();

    let args = Args::parse();

    let input = read_input(&args);
    println!("Input size: {}", input.len());

    let pairs = parse_json(input);
    println!("Pair count: {}", pairs.len());

    let sum = calc_distances(pairs);
    println!("Haversine sum: {}", sum);

    println!("Validation");
    let ref_sum = std::fs::read(args.answers)
        .expect("Could not read answers file")
        .into_iter()
        .rev()
        .take(8)
        .collect::<Vec<u8>>();
    let ref_sum = f64::from_be_bytes(ref_sum.try_into().expect("Could not convert to f64"));
    println!("Reference sum: {}", ref_sum);
    println!("Difference: {}", sum - ref_sum);

    let total_end = profiler::read_tsc();

    let tsc_hz = profiler::tsc_frequency_calibrated();

    let measurment_ticks = measurement_end - measurement_start;
    let measurment_ns = profiler::ticks_to_ns(measurment_ticks, tsc_hz);

    let total_ticks = total_end - total_start;
    let total_ns = profiler::ticks_to_ns(total_ticks, tsc_hz);

    println!("");
    println!("CPU frequency: {} MHz", tsc_hz / 1_000_000);
    println!(
        "Measurement overhead: {} ticks ({} ns)",
        measurment_ticks, measurment_ns
    );
    println!("Total time: {} ms", total_ns / 1_000_000);

    for (entry, ticks) in profiler::PROFILE_DATA.with(|data| data.borrow().clone()) {
        let ns = profiler::ticks_to_ns(ticks, tsc_hz);
        println!(
            "{}: {} ms ({:.2}%)",
            entry,
            ns / 1_000_000,
            (ticks as f64 / total_ticks as f64) * 100.0
        );
    }
}

#[profile]
fn calc_distances(pairs: Vec<[f64; 4]>) -> f64 {
    let distances = pairs.iter().map(haversine::calculate).collect::<Vec<f64>>();
    let sum = distances.iter().sum::<f64>();
    sum
}

#[profile]
fn parse_json(input: Vec<u8>) -> Vec<[f64; 4]> {
    let pairs = json::parse_pairs(&input).expect("Could not parse pairs");
    pairs
}

#[profile]
fn read_input(args: &Args) -> Vec<u8> {
    std::fs::read(args.input.clone()).expect("Could not read file")
}
