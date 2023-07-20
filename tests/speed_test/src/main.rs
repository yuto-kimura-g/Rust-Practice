#![allow(unused_imports)]
use std::time::{Duration, Instant};

fn main() {
    println!("> rust speed test");
    let start_time = Instant::now();

    let mut sum: i64 = 0;
    const N: i64 = 1_000_000_000;
    const MOD: i64 = 101;
    for i in 1..N {
        sum += i;
        sum %= MOD;
    }
    println!("sum[0:{}]={}", N, sum);

    let end_time = start_time.elapsed();
    println!(
        "elapsed: {}.{:03} [sec]",
        end_time.as_secs(),
        end_time.as_millis() / 1_000_000
    );
}
