use std::num::Wrapping;

use valence::prelude::BlockPos;

pub mod legacy;
pub mod xoroshiro;

const FLOAT_MULTIPLIER: f32 = 5.9604645E-8f32;
const DOUBLE_MULTIPLIER: f64 = 1.110223E-16f64;

pub trait RandomSource {
    fn fork(&mut self) -> Box<dyn RandomSource>;
    fn fork_positional(&mut self) -> Box<dyn PositionalRandomFactory>;
    fn set_seed(&mut self, seed: i64);
    fn next_i32(&mut self) -> i32;
    fn next_i32_bound(&mut self, bound: i32) -> i32;
    fn next_i32_between_inclusive(&mut self, interval: (i32, i32)) -> i32 {
        self.next_i32_bound(interval.1 - interval.0 + 1) + interval.0
    }
    fn next_i64(&mut self) -> i64;
    fn next_bool(&mut self) -> bool;
    fn next_f32(&mut self) -> f32;
    fn next_f64(&mut self) -> f64;
    fn consume(&mut self, count: usize) {
        for _ in 0..count {
            self.next_i32();
        }
    }
}

pub trait PositionalRandomFactory {
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource>;
    fn at_block(&self, pos: BlockPos) -> Box<dyn RandomSource> {
        self.at(pos.x, pos.y, pos.z)
    }
    fn with_hash_of(&self, string: &str) -> Box<dyn RandomSource>;
}

pub fn java_string_hash(str: &str) -> i32 {
    let mut hash = Wrapping(0i32);

    for b in str.as_bytes() {
        hash = Wrapping(31) * hash + Wrapping((b & 0xff) as i32)
    }

    hash.0
}