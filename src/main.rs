use num_bigint_ext::*;

fn now() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn main() {
    let beg = now();
    let p = prime::rand_safe_prime(2048);
    let end = now();
    let q = &p >> 1;
    assert_eq!(true, prime::is_prime(&q));
    println!("Prime = {}", p);
    println!("Time = {} ms", end - beg);
}
