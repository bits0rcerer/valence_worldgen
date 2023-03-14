use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
pub struct ClimatePoint {
    temperature: ClimateRange,
    humidity: ClimateRange,
    continentalness: ClimateRange,
    erosion: ClimateRange,
    depth: ClimateRange,
    weirdness: ClimateRange,
    offset: i64,
}

pub type ClimateRange = [f64; 2];