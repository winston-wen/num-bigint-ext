use crate::{one, rand::RandFixedLength, small_primes, two, zero};
use num_bigint::{BigInt, RandBigInt};
use num_traits::Euclid;
use rand::{rngs::OsRng, RngCore};

pub fn rand_prime(nbits: usize, rng: &mut impl RngCore) -> BigInt {
    loop {
        let mut p = BigInt::rand_exact_nbits(nbits, rng);
        p.set_bit(0, true); // make sure `p` is odd.

        if try_div(&p) == false {
            continue;
        }
        if miller_rabin(&p, 5) {
            return p;
        }
    }
}

/// q and p=2q+1 are both prime.
/// In this case, they are called Sophie Germain prime and safe prime.
pub fn rand_safe_prime(nbits: usize, rng: &mut impl RngCore) -> BigInt {
    // see footnote (SP-1)
    fn half_small_prime(n: &BigInt) -> bool {
        for sp in small_primes()[1../* start from 3 */].iter() {
            let half = sp >> 1;
            if n % sp == half {
                return true;
            }
        }
        return false;
    }

    loop {
        let mut q = BigInt::rand_exact_nbits(nbits - 1, rng);
        q.set_bit(0, true);
        // !!! WRONG !!! -- let p = &q << 1 + 1; -- it is parsed as &q << (1 + 1);
        let p: BigInt = (&q << 1) + 1;

        if false == try_div(&q) {
            continue;
        }
        if half_small_prime(&q) {
            continue;
        }
        if false == try_div(&p) {
            continue;
        }

        if false == miller_rabin(&q, 5) {
            continue;
        }
        if miller_rabin(&p, 5) {
            return p;
        }
    }
}

pub fn is_prime(n: &BigInt) -> bool {
    for small_prime in small_primes().iter() {
        if &n.rem_euclid(small_prime) == zero() {
            return false;
        }
    }
    miller_rabin(n, 4)
}

pub fn try_div(n: &BigInt) -> bool {
    for sp in small_primes().iter() {
        if &(n % sp) == zero() {
            return false;
        }
    }
    true
}

pub fn miller_rabin(n: &BigInt, trial: usize) -> bool {
    let mut rng = OsRng;
    let n_minus_one: BigInt = n - 1;
    let n_minus_two = n - 2;

    // solve $s, d$ such that $n - 1 = 2^s \cdot d$ and $d$ is odd.
    let (s, d) = {
        let mut d = n_minus_one.clone();
        let mut s: u64 = 0;
        while d.bit(0) == false {
            d >>= 1;
            s += 1;
        } // while d is even
        (s, d)
    };

    'outer: for _ in 0..trial {
        let a = rng.gen_bigint_range(two(), &n_minus_two);
        let mut x = a.modpow(&d, &n);

        // Check if n is "strong probable prime".
        if &x == one() || x == n_minus_one {
            continue 'outer;
        }

        // Iterate over `x = a^(2^r * d) % n` for r in 1..=s.
        // The r-th x is the square root of the (r+1)-th x.
        for _r in 1..=s {
            let x_was_minus_one = x == n_minus_one;
            x = x.modpow(two(), &n);
            let x_is_one = &x == one();

            if x_is_one && !x_was_minus_one {
                return false;
            } // Non-trivial sqrt of 1 modulo n. See footnotes (MR-1).

            if x_is_one {
                continue 'outer;
            } // A little optimization of speed. See footnotes (MR-2).
        }

        return false; // Now `x != 1`, which fails the Fermat test.
    }
    true
}

/// https://www.cse.iitk.ac.in/users/manindra/algebra/primality_v6.pdf
/// TODO
#[allow(dead_code)]
pub fn aks(_p: &BigInt) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_miller_rabin() {
        let mersenne_1279 = BigInt::from(2).pow(1279) - 1;
        assert_eq!(miller_rabin(&mersenne_1279, 5), true);
        let a001262_32 = BigInt::from(486737);
        assert_eq!(miller_rabin(&a001262_32, 5), false);
        let a020231_33 = BigInt::from(315121);
        assert_eq!(miller_rabin(&a020231_33, 5), false);

        // source:
        // Arnault F., Rabin-Miller primality test: composite numbers that pass it, Math. Comp., Volume 64, No. 209, 355-361, 1995.
        let large_strong_pseudo = "
        80383745745363949125707961434194210813883768828755
        81458374889175222974273765333652186502336163960045
        45791504202360320876656996676098728404396540823292
        87387918508691668573282677617710293896977394701670
        82304286871099974399765441448453411558724506334092
        79022275296229414984230688168540432645753401832978
        6111298960644845216191652872597534901"
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let large_strong_pseudo = BigInt::from_str(&large_strong_pseudo).unwrap();
        assert_eq!(miller_rabin(&large_strong_pseudo, 5), false);
    }

    #[test]
    fn test_rand_prime() {
        let mut rng = OsRng;
        let p = rand_prime(2048, &mut rng);
        println!("rand_prime: {}", p);
    }

    #[test]
    fn test_rand_safe_prime() {
        let mut rng = OsRng;
        let p = rand_safe_prime(512, &mut rng);
        println!("rand_safe_prime: {}", p);
    }
}

/* ===== Footnotes =====

(MR-1) Why `x^2 % n == 1` indicates that `n` is composite?

`x^2 % n == 1` implies that `n` divides `(x-1)(x+1)`.
`x >=2 && x <= n-2` implies that `n` does not divide `x-1` and `x+1`.
By this property of great common divisor: `gcd(a1, b) * gcd(a2, b) == gcd(a1*a2, b)`
one has `gcd(x-1, n) * gcd(x+1, n) == gcd((x-1)(x+1), n) == n`.
So `n` is composite.

(MR-2) Why optimize the inner loop of Miller Rabin?

If some r causes `x(r) == 1` where `x(r) = a^(2^r * d) % n`,
since `x(r+1) = x(r) ^ 2 % n`, then `x(r+1) == 1`.
By induction, `x(r+k) == 1` for all `k >= 0`.
Specifically, `x(s) == a^(2^s * d) % n == a^(n-1) % n == 1`,
which passes the Fermat test, and passes one Miller-Rabin trial.

========== */
