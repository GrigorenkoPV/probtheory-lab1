use std::{f64::consts::PI, ops::Range};

use rand::prelude::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const NUMBER_OF_EXPERIMENTS: usize = 1_000_000_000;

fn experiment() -> bool {
    static SQUARE_SIDE: Range<f64> = 0f64..1f64;
    let rng = &mut thread_rng();
    let ax = rng.gen_range(SQUARE_SIDE.clone());
    let ay = rng.gen_range(SQUARE_SIDE.clone());
    let bx = rng.gen_range(SQUARE_SIDE.clone());
    let by = rng.gen_range(SQUARE_SIDE.clone());
    let x0 = (ax + bx) / 2.;
    let y0 = (ay + by) / 2.;
    let r = ((ax - bx).powi(2) + (ay - by).powi(2)).sqrt() / 2.;
    SQUARE_SIDE.start <= x0 - r
        && x0 + r <= SQUARE_SIDE.end
        && SQUARE_SIDE.start <= y0 - r
        && y0 + r <= SQUARE_SIDE.end
}

fn main() {
    let result = (0..NUMBER_OF_EXPERIMENTS)
        .into_par_iter()
        .filter(|_| experiment())
        .count();
    println!(
        "{} out of {} circles were contained within the square,",
        result, NUMBER_OF_EXPERIMENTS
    );
    println!(
        "Thus, the result is {:.5}%",
        (result as f64 / NUMBER_OF_EXPERIMENTS as f64) * 100.
    );
    println!("And the expected is {:.5}%", (PI / 6.) * 100.);
}
