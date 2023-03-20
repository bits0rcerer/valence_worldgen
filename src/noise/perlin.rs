use std::simd::{f64x2, f64x4};

use crate::noise::improved_noise::ImprovedNoise;
use crate::noise::wrap;
use crate::random::RandomSource;

pub struct PerlinNoise {
    noise_levels: Vec<Option<ImprovedNoise>>,
    amplitudes: Vec<f64>,
    lowest_freq_value_factor: f64,
    lowest_freq_input_factor: f64,
    max: f64,
}

impl PerlinNoise {
    pub fn new(r: &mut dyn RandomSource, first_octave: i32, amplitudes: &[f64]) -> Self {
        let random_factory = r.fork_positional();
        let noise_levels = amplitudes
            .iter()
            .enumerate()
            .map(|(i, &amp)| {
                if amp != 0.0 {
                    Some(ImprovedNoise::new(
                        random_factory
                            .with_hash_of(format!("octave_{}", first_octave + i as i32).as_str())
                            .as_mut(),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let lowest_freq_input_factor = 2.0f64.powi(first_octave);
        let lowest_freq_value_factor = 2.0f64.powi((amplitudes.len() - 1) as i32)
            / (2.0f64.powi(amplitudes.len() as i32) - 1.0);

        Self {
            max: Self::edge_value(2.0, amplitudes, lowest_freq_value_factor),
            noise_levels,
            lowest_freq_input_factor,
            lowest_freq_value_factor,
            amplitudes: Vec::from(amplitudes),
        }
    }

    pub fn new_legacy_nether(
        r: &mut dyn RandomSource,
        first_octave: i32,
        amplitudes: &[f64],
    ) -> Self {
        dbg!(r.kind());
        dbg!(first_octave);
        dbg!(amplitudes);
        todo!()
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn get_value(&self, xyz: f64x4) -> f64 {
        let mut factors =
            f64x2::from_array([self.lowest_freq_input_factor, self.lowest_freq_value_factor]);

        self.noise_levels
            .iter()
            .enumerate()
            .map(|(i, level)| {
                let amp = self.amplitudes[i] * factors.as_array()[1];
                let input = xyz * f64x4::splat(factors.as_array()[0]);
                factors *= f64x2::from_array([2.0, 0.5]);

                amp * match level {
                    None => 0.0,
                    Some(level) => level.noise(wrap(input)),
                }
            })
            .sum()
    }

    fn edge_value(x: f64, amplitudes: &[f64], lowest_freq_value_factor: f64) -> f64 {
        let mut value_factor = lowest_freq_value_factor;

        amplitudes
            .iter()
            .map(|amp| {
                let v = amp * x * value_factor;
                value_factor /= 2.0;
                v
            })
            .sum()
    }
}
