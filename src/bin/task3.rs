use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

type Count = usize;

fn experiment(n: usize, p: f64) -> [Count; 2] {
    assert!((0.0..=1.0).contains(&p));
    let rng = &mut thread_rng();
    let mut result = [0, 0];
    for s_i in &mut result {
        for _ in 0..n {
            if rng.gen_range(0.0..1.0) < p {
                *s_i += 1;
            }
        }
    }
    result
}

fn choose(n: Count, k: Count) -> f64 {
    assert!(k <= n);
    let mut result = 1.0;
    let k = k.min(n - k);
    for i in 0..k {
        result *= (n - i) as f64 / (i + 1) as f64;
    }
    result
}

const NUMBER_OF_EXPERIMENTS: usize = 10_000_000;
const N: usize = 10;
const P: f64 = 0.625;

fn main() {
    let result: Vec<Vec<Count>> = (0..NUMBER_OF_EXPERIMENTS)
        .into_par_iter()
        .map(|_| experiment(N, P))
        .fold(
            || vec![vec![0; N + 1]; 2 * N + 1],
            |mut res, [s1, s2]| {
                res[s1 + s2][s1] += 1;
                res
            },
        )
        .reduce_with(|mut res1, res2| {
            for m in 0..=(2 * N) {
                for k in 0..=N {
                    res1[m][k] += res2[m][k];
                }
            }
            res1
        })
        .expect("We are not conducting zero experiments, are we?");
    for (m, result) in result.into_iter().enumerate() {
        let total = result.iter().cloned().sum::<Count>();
        if total == 0 {
            println!("m = {:2} never happened", m);
            continue;
        }
        for (k, r) in result.into_iter().enumerate() {
            if k > m || m - k > N {
                debug_assert_eq!(r, 0);
                continue;
            }
            let two_n_choose_m = choose(2 * N, m);
            println!(
                "m = {:<2} k = {:<2} : {:6} / {:<6}  got {:06.3}%, expected {:06.3}%",
                m,
                k,
                r,
                total,
                (r as f64 / total as f64) * 100.,
                (choose(N, k) * choose(N, m - k) / two_n_choose_m) * 100.
            );
        }
    }
}
