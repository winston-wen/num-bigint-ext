use num_bigint_ext::*;
use rand_core::OsRng;

fn now() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn main() {
    let mut rng = OsRng;
    let beg = now();
    let p = prime::rand_safe_prime(2048, &mut rng);
    let end = now();
    let q = &p >> 1;
    assert_eq!(true, prime::is_prime(&q));
    println!("Prime = {}", p);
    println!("Time = {} ms", end - beg);
}
