use std::num::Wrapping;

use crate::random::{DOUBLE_MULTIPLIER, FLOAT_MULTIPLIER, java_string_hash, PositionalRandomFactory, RandomSource};

#[derive(Copy, Clone)]
pub struct LegacyRandom(Wrapping<i64>);

const MODULUS_BITS: Wrapping<usize> = Wrapping(48);
const MODULUS_MASK: Wrapping<i64> = Wrapping(281474976710655i64);
const MULTIPLIER: Wrapping<i64> = Wrapping(25214903917i64);
const INCREMENT: Wrapping<i64> = Wrapping(11i64);

impl LegacyRandom {
    pub fn new(seed: i64) -> Box<dyn RandomSource> {
        Box::new(Self(Wrapping(seed)))
    }

    pub fn next_bits(&mut self, bits: usize) -> i32 {
        self.0 = (self.0 * MULTIPLIER + INCREMENT) & MODULUS_MASK;
        return (self.0 >> (MODULUS_BITS.0 - bits)).0 as i32;
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
        self.0.0 = seed;
    }

    fn next_i32(&mut self) -> i32 {
        self.next_bits(32)
    }

    fn next_i32_bound(&mut self, bound: i32) -> i32 {
        assert!(bound > 0);

        if bound & bound - 1 == 0 {
            (((bound as i64) * (self.next_bits(31) as i64)) >> 31) as i32
        } else {
            let mut i;
            let mut j;
            loop {
                i = self.next_bits(31);
                j = i % bound;
                if i - j + (bound - 1) >= 0 { return j; }
            }
        }
    }

    fn next_i64(&mut self) -> i64 {
        let lo = self.next_i32();
        let hi = self.next_i32();
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
        (((i as i64) << 27) + (j as i64)) as f64 * DOUBLE_MULTIPLIER
    }
}

struct LegacyPositionalRandomFactory(i64);

impl PositionalRandomFactory for LegacyPositionalRandomFactory {
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        let mut i = Wrapping(((x as i64) * 3129871) ^ ((z as i64) as i64 * 116129781) ^ ((y as i64) as i64));
        i = i * i * Wrapping(42317861) + i * INCREMENT;
        i >>= 16;

        LegacyRandom::new(i.0 ^ self.0)
    }

    fn with_hash_of(&self, string: &str) -> Box<dyn RandomSource> {
        LegacyRandom::new(java_string_hash(string) as i64 ^ self.0)
    }
}