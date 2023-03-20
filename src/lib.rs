#![feature(wrapping_int_impl)]
#![feature(portable_simd)]
extern crate core;

mod biome;
pub mod density_function;
pub mod noise;
pub mod random;
pub mod registry;
pub mod spline;
mod surface;

#[cfg(test)]
mod test;
