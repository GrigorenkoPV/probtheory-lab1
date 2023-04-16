use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const N: usize = 99;

fn experiment() -> usize {
    const PROGENITOR: usize = 0;
    let rng = &mut thread_rng();
    let mut last_generation = [false; N + 1];
    last_generation[PROGENITOR] = true;
    for r in 1.. {
        let mut current_generation = [false; N + 1];
        for from in 0..=N {
            if last_generation[from] {
                let to1 = std::iter::repeat_with(|| rng.gen_range(0..=N))
                    .find(|&x| x != from)
                    .unwrap();
                let to2 = std::iter::repeat_with(|| rng.gen_range(0..=N))
                    .find(|&x| x != from && x != to1)
                    .unwrap();
                current_generation[to1] = true;
                current_generation[to2] = true;
            }
        }
        if current_generation[PROGENITOR] {
            return r;
        } else {
            last_generation = current_generation;
        }
    }
    unreachable!() // actually reachable, but with unspeakably low probability
}

const NUMBER_OF_EXPERIMENTS: usize = 10_000_000;

fn main() {
    println!("(n + 1) = {}", N + 1);
    let result: Vec<usize> = (0..NUMBER_OF_EXPERIMENTS)
        .into_par_iter()
        .map(|_| experiment())
        .fold(Vec::new, |mut results, r| {
            if results.len() <= r {
                results.resize(r + 1, 0);
            }
            results[r] += 1;
            results
        })
        .reduce(Vec::new, |res1, res2| {
            let [from, mut to] = if res1.len() < res2.len() {
                [res1, res2]
            } else {
                [res2, res1]
            };
            for (from, to) in Iterator::zip(from.into_iter(), to.iter_mut()) {
                *to += from
            }
            to
        });
    assert_eq!(result.iter().sum::<usize>(), NUMBER_OF_EXPERIMENTS);
    println!("Не входит ни в одно из первых `r` поколений с вероятностью `p`:");
    let mut p = NUMBER_OF_EXPERIMENTS;
    for (r, delta_p) in result.into_iter().enumerate().skip(1) {
        p -= delta_p;
        println!(
            "r = {:2}, p = {:>8} / {} = {:08.5}%",
            r,
            p,
            NUMBER_OF_EXPERIMENTS,
            (p as f64 / NUMBER_OF_EXPERIMENTS as f64) * 100.,
        );
    }
}
