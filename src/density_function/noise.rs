use std::simd::{f64x4, i32x4};
use std::sync::Arc;

use valence::prelude::{BlockPos, Ident};

use crate::density_function::DensityFunction;
use crate::noise::normal::NormalNoise;
use crate::random::random_state::RandomState;

pub type InputScrambler = fn(BlockPos) -> f64x4;

const DEFAULT_SCRAMBLER: InputScrambler = |pos: BlockPos| i32x4::from_array([pos.x, pos.y, pos.z, 0]).cast();

#[derive(Clone)]
pub struct Noise {
    noise: Arc<NormalNoise>,
    value_factor: f64,
    input_factor: f64x4,
    input_scrambler: InputScrambler,
    shift: Arc<Shift>,
}

enum Shift {
    None,
    Dynamic {
        x: Box<dyn DensityFunction>,
        y: Box<dyn DensityFunction>,
        z: Box<dyn DensityFunction>,
    },
}

impl Noise {
    pub fn new(noise: NormalNoise, value_factor: f64, input_factor: f64x4) -> Box<dyn DensityFunction> {
        Self::new_with_scrambler(noise, value_factor, input_factor, DEFAULT_SCRAMBLER)
    }

    pub fn new_with_shift(noise: NormalNoise, value_factor: f64, input_factor: f64x4,
                          shift_x: Box<dyn DensityFunction>,
                          shift_y: Box<dyn DensityFunction>,
                          shift_z: Box<dyn DensityFunction>,
    ) -> Box<dyn DensityFunction> {
        Box::new(
            Self {
                noise: Arc::new(noise),
                value_factor,
                input_factor,
                input_scrambler: DEFAULT_SCRAMBLER,
                shift: Arc::new(Shift::Dynamic {
                    x: shift_x,
                    y: shift_y,
                    z: shift_z,
                }),
            }
        )
    }

    pub fn new_with_scrambler(noise: NormalNoise, value_factor: f64, input_factor: f64x4,
                              input_scrambler: InputScrambler) -> Box<dyn DensityFunction> {
        Box::new(
            Self {
                noise: Arc::new(noise),
                value_factor,
                input_factor,
                input_scrambler,
                shift: Arc::new(Shift::None),
            }
        )
    }
}

impl DensityFunction for Noise {
    fn compute(&self, pos: BlockPos) -> f64 {
        let input = (self.input_scrambler)(pos) * self.input_factor;

        let input = match self.shift.as_ref() {
            Shift::None => input,
            Shift::Dynamic { x, y, z } =>
                input + f64x4::from_array([
                    x.compute(pos),
                    y.compute(pos),
                    z.compute(pos),
                    0.0
                ])
        };

        self.noise.get_value(input) * self.value_factor
    }

    fn map(&self, _: fn(&dyn DensityFunction) -> Box<dyn DensityFunction>) -> Box<dyn DensityFunction> {
        Box::new(self.clone())
    }

    fn min(&self) -> f64 {
        -self.max()
    }

    fn max(&self) -> f64 {
        self.noise.max() * self.value_factor
    }
}

pub fn noise(id: &Ident<String>, random_state: &RandomState, value_factor: f64, input_factor: f64x4) -> eyre::Result<Box<dyn DensityFunction>> {
    let noise = instantiate_noise(id, random_state)?;
    Ok(Noise::new(noise, value_factor, input_factor))
}

pub fn shift_noise(id: &Ident<String>, random_state: &RandomState, value_factor: f64, input_factor: f64x4, input_scrambler: InputScrambler) -> eyre::Result<Box<dyn DensityFunction>> {
    let noise = instantiate_noise(id, random_state)?;
    Ok(Noise::new_with_scrambler(noise, value_factor, input_factor, input_scrambler))
}

pub fn shifted_noise(id: &Ident<String>, random_state: &RandomState, value_factor: f64, input_factor: f64x4,
                     shift_x: Box<dyn DensityFunction>, shift_y: Box<dyn DensityFunction>, shift_z: Box<dyn DensityFunction>)
                     -> eyre::Result<Box<dyn DensityFunction>>
{
    let noise = instantiate_noise(id, random_state)?;
    Ok(Noise::new_with_shift(noise, value_factor, input_factor, shift_x, shift_y, shift_z))
}

fn instantiate_noise(id: &Ident<String>, random_state: &RandomState) -> eyre::Result<NormalNoise> {
    let noise_data = random_state.registry.noise(id)?;
    let noise = NormalNoise::new(random_state.random.with_hash_of(id.as_str()).as_mut(), &noise_data);
    Ok(noise)
}