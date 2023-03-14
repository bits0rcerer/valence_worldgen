use std::num::Wrapping;

use crate::random::{block_seed, DOUBLE_MULTIPLIER, FLOAT_MULTIPLIER, GOLDEN_RATIO_64, Kind, PositionalRandomFactory, RandomSource, SILVER_RATIO_64};

pub struct XoroshiroRandom {
    seed_lo: Wrapping<i64>,
    seed_hi: Wrapping<i64>,
}

impl XoroshiroRandom {
    pub fn new(seed: i64) -> Box<dyn RandomSource> {
        let seed = Self::upgrade_seed_to_128bit(seed);
        XoroshiroRandom::new_128(seed.0, seed.1)
    }

    pub fn new_128(seed_lo: i64, seed_hi: i64) -> Box<dyn RandomSource> {
        if seed_lo == 0 && seed_hi == 0 {
            Box::new(XoroshiroRandom {
                seed_lo: Wrapping(-7046029254386353131_i64),
                seed_hi: Wrapping(7640891576956012809_i64),
            })
        } else {
            Box::new(XoroshiroRandom {
                seed_lo: Wrapping(seed_lo),
                seed_hi: Wrapping(seed_hi),
            })
        }
    }

    fn mix_stafford_13(seed: i64) -> i64 {
        let mut seed = Wrapping(seed);

        seed = Wrapping(seed.0 ^ (seed.0 as u64 >> 30) as i64) * Wrapping(-4658895280553007687_i64);
        seed = Wrapping(seed.0 ^ (seed.0 as u64 >> 27) as i64) * Wrapping(-7723592293110705685_i64);
        seed = Wrapping(seed.0 ^ (seed.0 as u64 >> 31) as i64);

        seed.0
    }

    fn upgrade_seed_to_128bit(seed: i64) -> (i64, i64) {
        let l = seed ^ SILVER_RATIO_64.0;
        let m = Wrapping(l) + GOLDEN_RATIO_64;
        (Self::mix_stafford_13(l), Self::mix_stafford_13(m.0))
    }

    fn next_bits(&mut self, bits: usize) -> i64 {
        let i = self.seed_lo;
        let mut j = self.seed_hi;
        let k = (i + j).rotate_left(17) + i;
        j ^= i;

        self.seed_lo = i.rotate_left(49) ^ j ^ j << 21;
        self.seed_hi = j.rotate_left(28);

        (k.0 as u64 >> (64 - bits)) as i64
    }
}

impl RandomSource for XoroshiroRandom {
    fn fork(&mut self) -> Box<dyn RandomSource> {
        XoroshiroRandom::new_128(self.next_i64(), self.next_i64())
    }

    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory> {
        Box::new(XoroshiroPositionalRandomFactory {
            seed_lo: Wrapping(self.next_i64()),
            seed_hi: Wrapping(self.next_i64()),
        })
    }

    fn set_seed(&mut self, seed: i64) {
        let seed = XoroshiroRandom::upgrade_seed_to_128bit(seed);
        self.seed_lo.0 = seed.0;
        self.seed_hi.0 = seed.1;
    }

    fn next_i32(&mut self) -> i32 {
        self.next_i64() as i32
    }

    fn next_i32_bound(&mut self, bound: i32) -> i32 {
        assert!(bound >= 0);

        let mut i = (self.next_i32() as u32) as i64;
        let mut j = i * bound as i64;
        let mut k = j & 4294967295_i64;
        if k < bound as i64 {
            let l = ((!bound + 1) as u64 & bound as u64) as i64;
            while k < l {
                i = (self.next_i32() as u64) as i64;
                j = i * bound as i64;
                k = j & 4294967295_i64;
            }
        }

        (j >> 32) as i32
    }

    fn next_i64(&mut self) -> i64 {
        self.next_bits(64)
    }

    fn next_bool(&mut self) -> bool {
        self.next_i64() & 1 != 0
    }

    fn next_f32(&mut self) -> f32 {
        self.next_bits(24) as f32 * FLOAT_MULTIPLIER
    }

    fn next_f64(&mut self) -> f64 {
        self.next_bits(53) as f64 * DOUBLE_MULTIPLIER
    }

    fn kind(&self) -> Kind {
        Kind::Xoroshiro
    }
}

struct XoroshiroPositionalRandomFactory {
    seed_lo: Wrapping<i64>,
    seed_hi: Wrapping<i64>,
}

impl PositionalRandomFactory for XoroshiroPositionalRandomFactory {
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource> {
        XoroshiroRandom::new_128(block_seed(x, y, z) ^ self.seed_lo.0, self.seed_hi.0)
    }

    fn with_hash_of(&self, string: &str) -> Box<dyn RandomSource> {
        let hash_bytes = md5::compute(string);

        let lo = u64::from_be_bytes(hash_bytes.as_slice()[0..8].try_into().unwrap()) as i64;
        let hi = u64::from_be_bytes(hash_bytes.as_slice()[8..16].try_into().unwrap()) as i64;

        XoroshiroRandom::new_128(lo ^ self.seed_lo.0, hi ^ self.seed_hi.0)
    }

    fn kind(&self) -> Kind {
        Kind::Xoroshiro
    }
}