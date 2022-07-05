extern crate criterion;

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::PrimeField;
use ark_std::cmp::min;
use ark_std::UniformRand;
use ark_test_curves::bls12_381::G1Affine;
use std::time::Instant;

pub fn size_range(log_interval: usize, min_degree: usize, max_degree: usize) -> Vec<usize> {
    let mut to_ret = vec![min_degree.next_power_of_two()];
    let interval = 1 << log_interval;
    while *to_ret.last().unwrap() < max_degree {
        let next_elem = min(max_degree, interval * to_ret.last().unwrap());
        to_ret.push(next_elem);
    }
    to_ret
}

// degree bounds to benchmark on
// e.g. degree bound of 2^{15}, means we do an FFT for a degree (2^{15} - 1) polynomial
const BENCHMARK_MIN_DEGREE: usize = 1 << 16;
const BENCHMARK_MAX_DEGREE_BLS12_381: usize = 1 << 20;
const BENCHMARK_LOG_INTERVAL_DEGREE: usize = 1;

// returns vec![2^{min}, 2^{min + interval}, ..., 2^{max}], where:
// interval = BENCHMARK_LOG_INTERVAL_DEGREE
// min      = ceil(log_2(BENCHMARK_MIN_DEGREE))
// max      = ceil(log_2(BENCHMARK_MAX_DEGREE))
fn default_size_range_bls12_381() -> Vec<usize> {
    size_range(
        BENCHMARK_LOG_INTERVAL_DEGREE,
        BENCHMARK_MIN_DEGREE,
        BENCHMARK_MAX_DEGREE_BLS12_381,
    )
}

fn msm_common_setup<G: AffineCurve>(
    degree: usize,
) -> (Vec<<G::ScalarField as PrimeField>::BigInt>, Vec<G>) {
    let rng = &mut ark_std::test_rng();

    let mut random_scalars = Vec::new();
    for _ in 0..=degree {
        random_scalars.push(G::ScalarField::rand(rng).into_repr());
    }

    let mut random_points = Vec::new();
    for _ in 0..=degree {
        random_points.push(G::Projective::rand(rng).into_affine());
    }

    (random_scalars, random_points)
}

fn msm_benches<G: AffineCurve>(size_range: &[usize]) {
    for l in size_range {
        let (a, b) = msm_common_setup::<G>(*l);

        let time = Instant::now();
        for _ in 0..10 {
            let _ = ark_ec::msm::VariableBaseMSM::multi_scalar_mul(&b, &a);
        }
        println!("{} {}", l, time.elapsed().as_secs_f64() / 10.0);
    }
}

fn main() {
    msm_benches::<G1Affine>(&default_size_range_bls12_381());
}
