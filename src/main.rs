use clap::Parser;
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

    let read_start = profiler::read_tsc();
    let input = std::fs::read(args.input).expect("Could not read file");
    let read_end = profiler::read_tsc();
    println!("Input size: {}", input.len());

    let parse_start = profiler::read_tsc();
    let pairs = json::parse_pairs(&input).expect("Could not parse pairs");
    let parse_end = profiler::read_tsc();
    println!("Pair count: {}", pairs.len());

    let sum_start = profiler::read_tsc();
    let distances = pairs.iter().map(haversine::calculate).collect::<Vec<f64>>();
    let sum = distances.iter().sum::<f64>();
    let sum_end = profiler::read_tsc();
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

    let total = profiler::ticks_to_ns(total_end - total_start, tsc_hz);
    let read = profiler::ticks_to_ns(read_end - read_start, tsc_hz);
    let parse = profiler::ticks_to_ns(parse_end - parse_start, tsc_hz);
    let sum = profiler::ticks_to_ns(sum_end - sum_start, tsc_hz);

    println!("");
    println!("CPU frequency: {} MHz", tsc_hz / 1_000_000);
    println!("Measurement overhead: {} ticks ({} ns)", measurment_ticks, measurment_ns);
    println!("Total time: {} ms", total / 1_000_000);
    println!("  Read: {} ms ({:.2}%)", read / 1_000_000, (read as f64 / total as f64) * 100.0);
    println!("  Parse: {} ms ({:.2}%)", parse / 1_000_000, (parse as f64 / total as f64) * 100.0);
    println!("  Sum: {} ms ({:.2}%)", sum / 1_000_000, (sum as f64 / total as f64) * 100.0);
}
