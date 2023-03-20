use std::num::Wrapping;

use crate::random::{
    block_seed, java_string_hash, Kind, PositionalRandomFactory, RandomSource, DOUBLE_MULTIPLIER,
    FLOAT_MULTIPLIER, INCREMENT, MODULUS_BITS, MODULUS_MASK, MULTIPLIER,
};

#[derive(Copy, Clone)]
pub struct LegacyRandom(Wrapping<i64>);

impl LegacyRandom {
    pub fn new(seed: i64) -> Box<dyn RandomSource> {
        let mut r = LegacyRandom(Wrapping(0));
        r.set_seed(seed);

        Box::new(r)
    }

    pub fn next_bits(&mut self, bits: usize) -> i32 {
        self.0 = (self.0 * MULTIPLIER + INCREMENT) & MODULUS_MASK;
        (self.0 >> (MODULUS_BITS.0 - bits)).0 as i32
    }
}

impl RandomSource for LegacyRandom {
    fn fork(&mut self) -> Box<dyn RandomSource> {
        LegacyRandom::new(self.next_i64())
    }

    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory> {
        Box::new(LegacyPositionalRandomFactory(self.next_i64()))
    }

    fn set_seed(&mut self, seed: i64) {
        self.0 = (Wrapping(seed) ^ MULTIPLIER) & MODULUS_MASK;
    }

    fn next_i32(&mut self) -> i32 {
        self.next_bits(32)
    }

    fn next_i32_bound(&mut self, bound: i32) -> i32 {
        assert!(bound > 0);

        if bound & (bound - 1) == 0 {
            (((bound as i64) * (self.next_bits(31) as i64)) >> 31) as i32
        } else {
            let mut i;
            let mut j;
            loop {
                i = self.next_bits(31);
                j = i % bound;
                if i - j + (bound - 1) >= 0 {
                    return j;
                }
            }
        }
    }

    fn next_i64(&mut self) -> i64 {
        let hi = self.next_i32();
        let lo = self.next_i32();
        ((hi as i64) << 32) + (lo as i64)
    }

    fn next_bool(&mut self) -> bool {
        self.next_bits(1) != 0
    }

    fn next_f32(&mut self) -> f32 {
        self.next_bits(24) as f32 * FLOAT_MULTIPLIER
    }

    fn next_f64(&mut self) -> f64 {
        let i = self.next_bits(26);
        let j = self.next_bits(27);
        let k = Wrapping((i as i64) << 27) + Wrapping(j as i64);
        k.0 as f64 * DOUBLE_MULTIPLIER
    }

    fn kind(&self) -> Kind {
        Kind::LegacyRandom
    }
}

struct LegacyPositionalRandomFactory(i64);

impl PositionalRandomFactory for LegacyPositionalRandomFactory {
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        LegacyRandom::new(block_seed(x, y, z) ^ self.0)
    }

    fn with_hash_of(&self, string: &str) -> Box<dyn RandomSource> {
        LegacyRandom::new(java_string_hash(string) as i64 ^ self.0)
    }

    fn kind(&self) -> Kind {
        Kind::LegacyRandom
    }
}
