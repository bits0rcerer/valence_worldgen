use std::simd::{f64x2, f64x4, i32x4, SimdFloat, StdFloat};

pub mod deserialize;
mod improved_noise;
mod noise_router;
pub mod normal;
mod perlin;

#[cfg(test)]
mod test;

fn wrap(xyz: f64x4) -> f64x4 {
    xyz - f64x4::floor(xyz / f64x4::splat(3.3554432E7) + f64x4::splat(0.5))
        * f64x4::splat(3.3554432E7)
}

fn smooth_step(x: f64x4) -> f64x4 {
    x * x * x * (x * (x * f64x4::splat(6.0) - f64x4::splat(15.0)) + f64x4::splat(10.0))
}

const GRADIENTS: [i32x4; 16] = [
    i32x4::from_array([1, 1, 0, 0]),
    i32x4::from_array([-1, 1, 0, 0]),
    i32x4::from_array([1, -1, 0, 0]),
    i32x4::from_array([-1, -1, 0, 0]),
    i32x4::from_array([1, 0, 1, 0]),
    i32x4::from_array([-1, 0, 1, 0]),
    i32x4::from_array([1, 0, -1, 0]),
    i32x4::from_array([-1, 0, -1, 0]),
    i32x4::from_array([0, 1, 1, 0]),
    i32x4::from_array([0, -1, 1, 0]),
    i32x4::from_array([0, 1, -1, 0]),
    i32x4::from_array([0, -1, -1, 0]),
    i32x4::from_array([1, 1, 0, 0]),
    i32x4::from_array([0, -1, 1, 0]),
    i32x4::from_array([-1, 1, 0, 0]),
    i32x4::from_array([0, -1, -1, 0]),
];

fn grad_dot(grad_idx: usize, abc: f64x4) -> f64 {
    (GRADIENTS[grad_idx & (GRADIENTS.len() - 1)].cast::<f64>() * abc).reduce_sum()
}

fn lerp(t: f64, u0: f64, u1: f64) -> f64 {
    u0 + t * (u1 - u0)
}

fn lerp_x2(t: f64x2, u0: f64x2, u1: f64x2) -> f64x2 {
    u0 + t * (u1 - u0)
}

fn lerp2(s: f64, t: f64, v00: f64, v10: f64, v01: f64, v11: f64) -> f64 {
    let [u0, u1] = lerp_x2(
        f64x2::splat(s),
        f64x2::from_array([v00, v01]),
        f64x2::from_array([v10, v11]),
    )
    .to_array();
    lerp(t, u0, u1)
}

fn lerp3(
    r: f64,
    s: f64,
    t: f64,
    v000: f64,
    v001: f64,
    v100: f64,
    v101: f64,
    v010: f64,
    v011: f64,
    v110: f64,
    v111: f64,
) -> f64 {
    lerp(
        t,
        lerp2(r, s, v000, v001, v100, v101),
        lerp2(r, s, v010, v011, v110, v111),
    )
}
