use std::marker::PhantomData;
use std::sync::Arc;

use eyre::eyre;
use serde::{Deserialize, Deserializer};
use valence::prelude::BlockPos;

use crate::density_function::DensityFunction;
use crate::density_function::deserialize::DensityFunctionTree;
use crate::random::random_state::RandomState;

#[derive(Copy, Clone)]
pub struct Blueprint;

#[derive(Copy, Clone)]
pub struct Built;

#[derive(Clone)]
pub enum CubicSpline<State = Blueprint> {
    Constant(f32),
    Multipoint {
        coordinate: Arc<dyn DensityFunction>,
        points: Vec<CubicSplinePoint<Built>>,
    },
    MultipointBlueprint {
        coordinate: Arc<DensityFunctionTree>,
        points: Vec<CubicSplinePoint<State>>,
    },
}

impl<'de> Deserialize<'de> for CubicSpline<Blueprint> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Constant(f32),
            MultipointBlueprint {
                coordinate: Arc<DensityFunctionTree>,
                points: Vec<CubicSplinePoint<Blueprint>>,
            },
        }
        Raw::deserialize(d).map(|r|
            match r {
                Raw::Constant(arg) => CubicSpline::<Blueprint>::Constant(arg),
                Raw::MultipointBlueprint { points, coordinate } =>
                    CubicSpline::<Blueprint>::MultipointBlueprint { points, coordinate }
            }
        )
    }
}

#[derive(Clone)]
pub struct CubicSplinePoint<State = Blueprint> {
    marker: PhantomData<State>,
    location: f32,
    derivative: f32,
    value: CubicSpline<State>,
}

impl<'de> Deserialize<'de> for CubicSplinePoint<Blueprint> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        struct Raw {
            location: f32,
            derivative: f32,
            value: CubicSpline<Blueprint>,
        }
        Raw::deserialize(d).map(|r|
            CubicSplinePoint::<Blueprint> {
                marker: Default::default(),
                location: r.location,
                derivative: r.derivative,
                value: r.value,
            }
        )
    }
}

impl CubicSpline<Blueprint> {
    pub fn compile(&self, random_state: &RandomState) -> eyre::Result<CubicSpline<Built>> {
        Ok(match self {
            CubicSpline::Constant(arg) => CubicSpline::Constant(*arg),
            CubicSpline::Multipoint { coordinate, points } => CubicSpline::<Built>::Multipoint {
                coordinate: coordinate.clone(),
                points: points.clone(),
            },
            CubicSpline::MultipointBlueprint { coordinate, points } => {
                if points.is_empty() {
                    return Err(eyre!("spline points must not be empty"));
                }

                CubicSpline::<Built>::Multipoint {
                    coordinate: Arc::from(coordinate.compile(random_state)?),
                    points: points.iter().map(|p| p.compile(random_state))
                        .fold(Ok(Vec::with_capacity(points.len())), |acc, res| {
                            let mut points = match acc {
                                Err(e) => return Err(e),
                                Ok(points) => points,
                            };

                            match res {
                                Ok(p) => points.push(p),
                                Err(e) => return Err(e),
                            }

                            Ok(points)
                        })?,
                }
            }
        })
    }
}

impl CubicSpline<Built> {
    pub fn compute(&self, pos: BlockPos) -> f32 {
        match self {
            CubicSpline::Constant(arg) => *arg,
            CubicSpline::Multipoint { coordinate, points } => {
                let x = coordinate.compute(pos) as f32;
                let i = match points.binary_search_by(|p| p.location.total_cmp(&x)) {
                    Ok(i) => i as i32,
                    Err(i) => i as i32,
                } - 1;

                if i < 0 {
                    let p = points.first().expect("should contain at least one part");
                    return p.value.compute(pos) + p.derivative * (x - p.location);
                } else if i == (points.len() - 1) as i32 {
                    let p = points.last().expect("should contain at least one part");
                    return p.value.compute(pos) + p.derivative * (x - p.location);
                }

                let p0 = &points[i as usize];
                let p1 = &points[(i + 1) as usize];

                let t = (x - p0.location) / (p1.location - p0.location);

                let y0 = p0.value.compute(pos);
                let y1 = p1.value.compute(pos);

                let u = p0.derivative * (p1.location - p0.location) - (y1 - y0);
                let v = -p1.derivative * (p1.location - p0.location) + (y1 - y0);

                lerp_32(t, y0, y1) + t * (1f32 - t) * lerp_32(t, u, v)
            }
            CubicSpline::MultipointBlueprint { .. } => unreachable!(),
        }
    }

    pub fn min(&self) -> f32 {
        match self {
            CubicSpline::Constant(c) => *c,
            CubicSpline::Multipoint { points, .. } => points
                .iter()
                .fold(f32::INFINITY, |acc, b| f32::min(acc, b.value.min())),

            CubicSpline::MultipointBlueprint { .. } => unreachable!(),
        }
    }

    pub fn max(&self) -> f32 {
        match self {
            CubicSpline::Constant(c) => *c,
            CubicSpline::Multipoint { points, .. } => points
                .iter()
                .fold(f32::NEG_INFINITY, |acc, b| f32::max(acc, b.value.max())),

            CubicSpline::MultipointBlueprint { .. } => unreachable!(),
        }
    }
}

impl CubicSplinePoint<Blueprint> {
    pub fn compile(&self, random_state: &RandomState) -> eyre::Result<CubicSplinePoint<Built>> {
        Ok(CubicSplinePoint::<Built> {
            marker: Default::default(),
            derivative: self.derivative,
            location: self.location,
            value: self.value.compile(random_state)?,
        })
    }
}

pub fn lerp_32(t: f32, u0: f32, u1: f32) -> f32 {
    u0 + t * (u1 - u0)
}

#[cfg(test)]
mod test {
    use valence::prelude::BlockPos;

    use crate::random::xoroshiro::XoroshiroRandom;
    use crate::spline::{Built, CubicSpline, CubicSplinePoint};

    #[test]
    fn spline_test() {
        /*
        Java Code to generate sample

        var size = 128;
        var r = new XoroshiroRandomSource(0x786b544d6f473757L);
        var f = DensityFunctions.yClampedGradient(-64, 64, -64, 64);
        var f2 = DensityFunctions.yClampedGradient(-64, 64, 32, -32);

        var subSpline = CubicSpline.builder(new DensityFunctions.Spline.Coordinate(Holder.direct(f2)), ToFloatFunction.IDENTITY)
                .addPoint(-32, -16)
                .addPoint(0, 8)
                .addPoint(32, -16)
                .build();

        var spline = CubicSpline.builder(new DensityFunctions.Spline.Coordinate(Holder.direct(f)), ToFloatFunction.IDENTITY)
                .addPoint(-64, -32)
                .addPoint(-32, 32)
                .addPoint(0, subSpline)
                .addPoint(32, -128)
                .addPoint(64, 128)
                .build();

        var b = new StringBuilder();
        b.append("\nlet sample = vec![");
        for (int i = 0; i < size; i++) {
            if (i > 0) b.append(", ");
            b.append(String.format("%.17f_f64", spline.apply(new DensityFunctions.Spline.Point(new DensityFunction.SinglePointContext(0, r.nextIntBetweenInclusive(-96, 96), 0)))));
        }
        b.append("];\n");

        System.out.println(b.toString());
         */

        let sample = vec![-103.14118957519531000_f64, -30.41796875000000000_f64, 128.00000000000000000_f64, 23.87500000000000000_f64, -32.00000000000000000_f64, -61.87500000000000000_f64, 127.26562500000000000_f64, -128.00000000000000000_f64, 128.00000000000000000_f64, 9.18547439575195300_f64, -9.37687873840332000_f64, -32.00000000000000000_f64, 111.20312500000000000_f64, 2.99609375000000000_f64, 19.66015625000000000_f64, 128.00000000000000000_f64, 9.18547439575195300_f64, 7.59251785278320300_f64, 7.59251785278320300_f64, -32.00000000000000000_f64, 96.60937500000000000_f64, -86.72999572753906000_f64, -32.00000000000000000_f64, -32.00000000000000000_f64, -96.60937500000000000_f64, 128.00000000000000000_f64, 128.00000000000000000_f64, 128.00000000000000000_f64, 13.18946933746337900_f64, -47.00000000000000000_f64, 17.05169296264648400_f64, 26.09375000000000000_f64, 0.00000000000000000_f64, 128.00000000000000000_f64, 128.00000000000000000_f64, -32.00000000000000000_f64, 23.87500000000000000_f64, 16.01582527160644500_f64, 121.67187500000000000_f64, -117.00000000000000000_f64, 128.00000000000000000_f64, 9.18547439575195300_f64, -19.26372337341308600_f64, 30.54930114746093800_f64, 26.00920104980468800_f64, 128.00000000000000000_f64, 12.35692691802978500_f64, 117.00000000000000000_f64, 121.67187500000000000_f64, 2.99609375000000000_f64, 7.59251785278320300_f64, -126.59480285644531000_f64, -36.54530334472656000_f64, -17.15625000000000000_f64, -127.26562500000000000_f64, -61.87500000000000000_f64, 128.00000000000000000_f64, -117.00000000000000000_f64, 128.00000000000000000_f64, -80.73841094970703000_f64, 128.00000000000000000_f64, 32.00000000000000000_f64, -32.00000000000000000_f64, 128.00000000000000000_f64, 125.12500000000000000_f64, -32.00000000000000000_f64, -32.00000000000000000_f64, 26.09375000000000000_f64, 128.00000000000000000_f64, -116.24882507324219000_f64, -32.00000000000000000_f64, -97.97807312011719000_f64, -14.12011718750000000_f64, 1.89830017089843750_f64, 128.00000000000000000_f64, -19.66015625000000000_f64, 128.00000000000000000_f64, -1.31257343292236330_f64, -32.00000000000000000_f64, 4.48871231079101600_f64, -9.37687873840332000_f64, -126.59480285644531000_f64, 31.89834213256836000_f64, -127.26562500000000000_f64, 31.81640625000000000_f64, -5.08929347991943400_f64, -32.00000000000000000_f64, -32.00000000000000000_f64, -32.00000000000000000_f64, -9.37687873840332000_f64, 9.18547439575195300_f64, 128.00000000000000000_f64, 35.57812500000000000_f64, -32.00000000000000000_f64, 16.01582527160644500_f64, -126.59480285644531000_f64, 0.00000000000000000_f64, 0.00000000000000000_f64, 128.00000000000000000_f64, -107.93652343750000000_f64, -112.32005310058594000_f64, -32.00000000000000000_f64, 21.49792098999023400_f64, -30.41796875000000000_f64, -19.26372337341308600_f64, 28.98554611206054700_f64, -32.00000000000000000_f64, 10.24226284027099600_f64, -112.32005310058594000_f64, 0.00000000000000000_f64, 2.99609375000000000_f64, -26.09375000000000000_f64, -128.00000000000000000_f64, 8.20145702362060500_f64, -32.00000000000000000_f64, -14.12011718750000000_f64, 11.98437500000000000_f64, -32.00000000000000000_f64, 8.20145702362060500_f64, -32.00000000000000000_f64, 31.60832023620605500_f64, 128.00000000000000000_f64, -47.00000000000000000_f64, 96.60937500000000000_f64, 128.00000000000000000_f64, 0.00000000000000000_f64, -29.25000000000000000_f64, -116.24882507324219000_f64];
        let mut r = XoroshiroRandom::new(0x786b544d6f473757_i64);
        let f = crate::density_function::y_clamped_gradient::YClampedGradient::new(-64, 64, -64.0, 64.0).unwrap();
        let f2 = crate::density_function::y_clamped_gradient::YClampedGradient::new(-64, 64, 32.0, -32.0).unwrap();

        let sub_spline = CubicSpline::<Built>::Multipoint {
            coordinate: f2.into(),
            points: vec![
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: -32.0,
                    derivative: 0.0,
                    value: CubicSpline::<Built>::Constant(-16.0),
                },
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: 0.0,
                    derivative: 0.0,
                    value: CubicSpline::<Built>::Constant(8.0),
                },
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: 32.0,
                    derivative: 0.0,
                    value: CubicSpline::<Built>::Constant(-16.0),
                },
            ],
        };

        let spline: CubicSpline<Built> = CubicSpline::<Built>::Multipoint {
            coordinate: f.into(),
            points: vec![
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: -64.0,
                    derivative: 0.0,
                    value: CubicSpline::<Built>::Constant(-32.0),
                },
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: -32.0,
                    derivative: 0.0,
                    value: CubicSpline::<Built>::Constant(32.0),
                },
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: 0.0,
                    derivative: 0.0,
                    value: sub_spline,
                },
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: 32.0,
                    derivative: 0.0,
                    value: CubicSpline::<Built>::Constant(-128.0),
                },
                CubicSplinePoint::<Built> {
                    marker: Default::default(),
                    location: 64.0,
                    derivative: 0.0,
                    value: CubicSpline::<Built>::Constant(128.0),
                },
            ],
        };

        for s in sample {
            assert_eq!(s, spline.compute(BlockPos::new(0, r.next_i32_between_inclusive((-96, 96)), 0)) as f64)
        }
    }
}