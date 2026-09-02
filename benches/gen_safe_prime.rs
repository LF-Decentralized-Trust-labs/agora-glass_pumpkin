use criterion::{Criterion, criterion_group, criterion_main};
use glass_pumpkin::{prime, safe_prime};
use num_bigint::BigUint;
use num_traits::Num;
use rand::{SeedableRng, rngs::StdRng};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gen_prime u256", |b| b.iter(|| prime::new(256).unwrap()));
    c.bench_function("gen_safe_prime u256", |b| {
        b.iter(|| safe_prime::new(256).unwrap())
    });

    let prime_512 = BigUint::from_str_radix(
        "1675975991242824637446753124775730765934920727574049172215445180465220503759193372100234287270862928461253982273310756356719235351493321243304213304923049",
        10,
    )
    .unwrap();
    c.bench_function("Baillie-PSW u512", |b| {
        b.iter(|| prime::strong_check(std::hint::black_box(&prime_512)))
    });

    let mut rng = StdRng::seed_from_u64(0x5eed);
    c.bench_function("Miller-Rabin u512", |b| {
        b.iter(|| prime::check_with(std::hint::black_box(&prime_512), &mut rng))
    });

    let composite_1024 = &prime_512 * &prime_512;
    c.bench_function("Baillie-PSW composite u1024", |b| {
        b.iter(|| prime::strong_check(std::hint::black_box(&composite_1024)))
    });

    let sieve_worst_case_512 = &prime_512 * 17_863_u32;
    c.bench_function("small-prime sieve u512", |b| {
        b.iter(|| prime::strong_check(std::hint::black_box(&sieve_worst_case_512)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
