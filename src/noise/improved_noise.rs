use std::f64;
use std::simd::{f64x4, i32x4, StdFloat};

use crate::noise::{grad_dot, lerp3, smooth_step};
use crate::random::RandomSource;

const SIZE: usize = 256;

pub struct ImprovedNoise {
    points: [u8; SIZE],
    xyz_origin: f64x4,
}

impl ImprovedNoise {
    pub fn new(r: &mut dyn RandomSource) -> Self {
        let xyz_origin = f64x4::from_array([r.next_f64(), r.next_f64(), r.next_f64(), 0.0])
            * f64x4::splat(SIZE as f64);

        let mut points = [0; SIZE];
        for (i, s) in points.iter_mut().enumerate() {
            *s = i as u8;
        }

        for i in 0..points.len() {
            let j = i + r.next_i32_bound((SIZE - i) as i32) as usize;
            (points[i], points[j]) = (points[j], points[i]);
        }

        Self { xyz_origin, points }
    }

    pub fn noise(&self, xyz: f64x4) -> f64 {
        let xyz = self.xyz_origin + xyz;
        let ijk = xyz.floor().cast::<i32>();
        let abc = xyz - (ijk.cast::<f64>());
        self.sample_and_lerp(ijk, abc)
    }

    fn p(&self, idx: usize) -> i32 {
        (self.points[idx % SIZE] as usize % SIZE) as i32
    }

    fn sample_and_lerp(&self, ijk: i32x4, abc: f64x4) -> f64 {
        let [i, j, k, _] = *ijk.as_array();

        let i2 = self.p(i as usize);
        let j2 = self.p((i + 1) as usize);
        let k2 = self.p((i2 + j) as usize);
        let l = self.p((i2 + j + 1) as usize);
        let i1 = self.p((j2 + j) as usize);
        let j1 = self.p((j2 + j + 1) as usize);

        let d0 = grad_dot(self.p((k2 + k) as usize) as usize, abc);
        let d1 = grad_dot(
            self.p((i1 + k) as usize) as usize,
            abc + f64x4::from_array([-1.0, 0.0, 0.0, 0.0]),
        );
        let d2 = grad_dot(
            self.p((l + k) as usize) as usize,
            abc + f64x4::from_array([0.0, -1.0, 0.0, 0.0]),
        );
        let d3 = grad_dot(
            self.p((j1 + k) as usize) as usize,
            abc + f64x4::from_array([-1.0, -1.0, 0.0, 0.0]),
        );
        let d4 = grad_dot(
            self.p((k2 + k + 1) as usize) as usize,
            abc + f64x4::from_array([0.0, 0.0, -1.0, 0.0]),
        );
        let d5 = grad_dot(
            self.p((i1 + k + 1) as usize) as usize,
            abc + f64x4::from_array([-1.0, 0.0, -1.0, 0.0]),
        );
        let d6 = grad_dot(
            self.p((l + k + 1) as usize) as usize,
            abc + f64x4::from_array([0.0, -1.0, -1.0, 0.0]),
        );
        let d7 = grad_dot(
            self.p((j1 + k + 1) as usize) as usize,
            abc + f64x4::splat(-1.0),
        );

        let [d8, d9, d10, _] = *smooth_step(abc).as_array();

        lerp3(d8, d9, d10, d0, d1, d2, d3, d4, d5, d6, d7)
    }
}
