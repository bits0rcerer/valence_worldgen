use serde::Deserialize;

#[derive(Deserialize, Copy, Clone)]
pub struct ClimatePoint {
    temperature: ClimateRange,
    humidity: ClimateRange,
    continentalness: ClimateRange,
    erosion: ClimateRange,
    depth: ClimateRange,
    weirdness: ClimateRange,
    offset: f64,
}

#[derive(Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum ClimateRange {
    Point(f64),
    Range([f64; 2]),
}