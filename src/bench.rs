extern crate murmurhash3;

#[allow(unstable)]
extern crate test;

use std::time::duration::Duration;
use std::rand;
use std::rand::Rng;

use test::black_box;

use murmurhash3::murmurhash3_x64_128;

#[allow(unstable)]
fn test_iter(bytes: &[u8], seed: u64) -> Duration {
    return Duration::span(|| {
        black_box(murmurhash3_x64_128(bytes, seed));
    });
}

#[allow(unstable)]
fn main() {
    println!("MurmurHash3 x64 128bit Speed Test");

    let mut total_duration = Duration::zero();
    let trials = 1500;
    let mut data = [0u8; 256 * 1024];

    for i in range(0, trials) {
        rand::thread_rng().fill_bytes(&mut data);

        total_duration = total_duration + test_iter(&data, i);
    }

    let ns = total_duration.num_nanoseconds().unwrap() as f64;
    let total_s = ns / 1_000_000_000.0;
    let total_b = (data.len() as u64) * trials;
    let total_mb = (total_b as f64) / (1024.0 * 1024.0);

    println!("Avg MB/s: {}", total_mb / total_s);
}
