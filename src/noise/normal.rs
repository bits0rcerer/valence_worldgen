use std::simd::f64x4;

use crate::noise::NoiseParameters;
use crate::noise::perlin::PerlinNoise;
use crate::random::RandomSource;

const INPUT_FACTOR: f64 = 1.0181268882175227f64;

pub struct NormalNoise {
    value_factor: f64,
    max: f64,
    first: PerlinNoise,
    second: PerlinNoise,
}

impl NormalNoise {
    pub fn new(r: &mut dyn RandomSource, noise_data: NoiseParameters, not_legacy: bool) -> NormalNoise {
        let (first, second) = if not_legacy {
            (
                PerlinNoise::new(r, noise_data.first_octave, noise_data.amplitudes.as_slice()),
                PerlinNoise::new(r, noise_data.first_octave, noise_data.amplitudes.as_slice()),
            )
        } else {
            (
                PerlinNoise::new_legacy_nether(r, noise_data.first_octave, noise_data.amplitudes.as_slice()),
                PerlinNoise::new_legacy_nether(r, noise_data.first_octave, noise_data.amplitudes.as_slice()),
            )
        };

        let mut min_amp = i32::MAX;
        let mut max_amp = i32::MIN;

        for (i, &amp) in noise_data.amplitudes.iter().enumerate() {
            if amp != 0.0 {
                min_amp = min_amp.min(i as i32);
                max_amp = max_amp.max(i as i32);
            }
        }

        let expected_deviation = 0.1 * (1.0 + 1.0 / (max_amp - min_amp + 1) as f64);
        let value_factor = (1.0 / 6.0) / expected_deviation;
        let max_value = (first.max() + second.max()) * value_factor;

        Self {
            value_factor,
            max: max_value,
            first,
            second,
        }
    }

    pub fn get_value(&self, x: f64, y: f64, z: f64) -> f64 {
        let v = f64x4::from_array([x, y, z, 0.0]) * f64x4::splat(INPUT_FACTOR);
        dbg!((self.first.get_value(x, y, z) + self.second.get_value(v.as_array()[0], v.as_array()[1], v.as_array()[2])) * self.value_factor)
    }
}