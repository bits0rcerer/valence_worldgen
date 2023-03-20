use std::fmt::Formatter;
use std::num::Wrapping;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use valence_protocol::block_pos::BlockPos;

#[cfg(test)]
mod test;

pub mod legacy;
pub mod random_state;
pub mod xoroshiro;

const FLOAT_MULTIPLIER: f32 = 5.9604645E-8_f32;
const DOUBLE_MULTIPLIER: f64 = 1.110223E-16_f32 as f64;
const GOLDEN_RATIO_64: Wrapping<i64> = Wrapping(-7046029254386353131_i64);
const SILVER_RATIO_64: Wrapping<i64> = Wrapping(7640891576956012809_i64);

const MODULUS_BITS: Wrapping<usize> = Wrapping(48);
const MODULUS_MASK: Wrapping<i64> = Wrapping(281474976710655_i64);
const MULTIPLIER: Wrapping<i64> = Wrapping(25214903917_i64);
const INCREMENT: Wrapping<i64> = Wrapping(11_i64);

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
    fn kind(&self) -> Kind;
}

pub trait PositionalRandomFactory {
    fn at(&self, x: i32, y: i32, z: i32) -> Box<dyn RandomSource>;
    fn at_block(&self, pos: BlockPos) -> Box<dyn RandomSource> {
        self.at(pos.x, pos.y, pos.z)
    }
    fn with_hash_of(&self, string: &str) -> Box<dyn RandomSource>;
    fn kind(&self) -> Kind;
}

pub fn java_string_hash(str: &str) -> i32 {
    let mut hash = Wrapping(0i32);

    for b in str.as_bytes() {
        hash = Wrapping(31) * hash + Wrapping(*b as i32)
    }

    hash.0
}

fn block_seed(x: i32, y: i32, z: i32) -> i64 {
    let mut seed = Wrapping((Wrapping(x) * Wrapping(3129871_i32)).0 as i64)
        ^ (Wrapping(z as i64) * Wrapping(116129781_i64))
        ^ Wrapping(y as i64);
    seed = seed * seed * Wrapping(42317861_i64) + seed * INCREMENT;
    seed.0 >> 16
}

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    LegacyRandom,
    Xoroshiro,
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Kind;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a boolean indicating the usage of the legacy random source"
                )
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(match v {
                    true => Kind::LegacyRandom,
                    false => Kind::Xoroshiro,
                })
            }
        }

        deserializer.deserialize_bool(V)
    }
}

impl Kind {
    pub fn new_instance(&self, seed: i64) -> Box<dyn RandomSource> {
        match self {
            Kind::LegacyRandom => legacy::LegacyRandom::new(seed),
            Kind::Xoroshiro => xoroshiro::XoroshiroRandom::new(seed),
        }
    }
}
