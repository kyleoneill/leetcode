use std::time::Instant;

mod p_2;

pub fn run() {
    let start = Instant::now();
    let solution = p_2::solve();
    if solution {
        println!("Found solution in {} microseconds", start.elapsed().as_micros());
    }
    else {
        println!("Solution is incorrect")
    }
}