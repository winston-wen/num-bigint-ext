use num_bigint::{BigInt, Sign};
use rand_core::{OsRng, RngCore};

pub trait RandFixedLength {
    fn rand_nbits(nbits: usize) -> Self;
    fn rand_exact_nbits(nbits: usize) -> Self;
}

impl RandFixedLength for BigInt {
    fn rand_nbits(nbits: usize) -> Self {
        if nbits == 0 {
            return BigInt::from(0);
        }
        let mut rng = OsRng;
        let nbytes = (nbits - 1) / 8 + 1;
        let mut buf: Vec<u8> = vec![0; nbytes];
        rng.fill_bytes(&mut buf);
        BigInt::from_bytes_be(num_bigint::Sign::Plus, &buf)
    }

    fn rand_exact_nbits(nbits: usize) -> Self {
        if nbits == 0 {
            return BigInt::from(0);
        }
        let mut rng = OsRng;
        let nbytes = nbits.div_ceil(8);
        let mut buf: Vec<u8> = vec![0; nbytes];
        rng.fill_bytes(&mut buf);
        let mut n = BigInt::from_bytes_be(Sign::Plus, &buf) >> (nbytes * 8 - nbits);
        n.set_bit(nbits as u64 - 1, true);
        n
    }
}
