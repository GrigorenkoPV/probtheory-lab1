use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn experiment(n: usize, p: f64) -> usize {
    assert!((0.0..=1.0).contains(&p));
    let rng = &mut thread_rng();
    (0..n).filter(|_| rng.gen_range(0.0..1.0) < p).count()
}

fn main() {
    for n in [10, 100, 1000, 10000] {
        let number_of_experiments = 1_000_000_000 / n;
        for p in [0.001, 0.01, 0.1, 0.25, 0.5] {
            let result: Vec<usize> = (0..number_of_experiments)
                .into_par_iter()
                .map(|_| experiment(n, p))
                .fold(
                    || vec![0; n + 1],
                    |mut res, sn| {
                        res[sn] += 1;
                        res
                    },
                )
                .reduce_with(|mut res1, res2| {
                    for i in 0..=n {
                        res1[i] += res2[i];
                    }
                    res1
                })
                .expect("We are not conducting zero experiments, are we");
            for q in [
                0.001, 0.01, 0.1, 1., 2., 4., 10., 100., 1000., 10000., 100000.,
            ] {
                let sqrt = (n as f64 * p * q).sqrt();
                let lower = n as f64 / 2. - sqrt;
                let upper = n as f64 / 2. + sqrt;
                let range = lower..=upper;

                let mut r = 0;
                for (sn, &prob) in result.iter().enumerate() {
                    if range.contains(&(sn as f64)) {
                        r += prob;
                    }
                }
                println!(
                    "n = {:<5}, p = {:<5}, q = {:<6}, P = {:9.5}%",
                    n,
                    p,
                    q,
                    (r as f64 / number_of_experiments as f64) * 100.
                )
            }
        }
    }
}
