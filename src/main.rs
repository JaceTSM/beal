mod beal;
mod analysis;


// const LIMIT: u64 = u64::MAX;  // u64::MAX -> breaks on conversion to f64
// const LIMIT: u64 = 100_000_000_000_000; // 100T -> 50748 power numbers. 1Q overflows.
const LIMIT: u64 = 100_000_000_000;
// const LIMIT: u64 = 100;

fn main() {
    beal::run_beal_analysis(LIMIT);
    // inspect_100t_limit_anomaly();
}

