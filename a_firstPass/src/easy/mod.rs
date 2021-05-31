use std::time::Instant;

mod p_1480;
mod p_771;
mod p_1108;
mod p_1342;

pub fn run() {
    let start = Instant::now();
    let solution = p_1342::solve();
    if solution {
        println!("Found solution in {} microseconds", start.elapsed().as_micros());
    }
    else {
        println!("Solution is incorrect")
    }
}