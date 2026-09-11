mod haversine;
mod json;
use clap::Parser;
use std::path::PathBuf;

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
    let args = Args::parse();
    let input = std::fs::read(args.input).expect("Could not read file");
    println!("Input size: {}", input.len());

    let pairs = json::parse_pairs(&input).expect("Could not parse pairs");
    println!("Pair count: {}", pairs.len());

    let distances = pairs.iter().map(haversine::calculate).collect::<Vec<f64>>();
    let sum = distances.iter().sum::<f64>();
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
}
