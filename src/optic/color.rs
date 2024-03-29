use std::ops::{Add, Mul};

use crate::error::RayTracingError;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}
// * note that the creation is made to bound it to 0..=1 but subsequent operations can bring the number higher than 1
// this allows for color summation and averaging, but we must be careful when converting back to an actual color format such as triple u8

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Result<Self, RayTracingError> {
        if !(0. ..=1.).contains(&r) || !(0. ..=1.).contains(&g) || !(0. ..=1.).contains(&b) {
            Err(RayTracingError::ColorCoefficientOOB(r, g, b))
        } else {
            Ok(Color { r, g, b })
        }
    }
    pub fn new_from_color(self) -> Result<Self, RayTracingError> {
        if !(0. ..=1.).contains(&self.r)
            || !(0. ..=1.).contains(&self.g)
            || !(0. ..=1.).contains(&self.b)
        {
            Err(RayTracingError::ColorCoefficientOOB(self.r, self.g, self.b))
        } else {
            Ok(Color {
                r: self.r,
                g: self.g,
                b: self.b,
            })
        }
    }

    pub fn to_diffusion_coefficient(&self) -> Result<DiffusionCoefficient, RayTracingError> {
        let Color { r, g, b } = *self;
        DiffusionCoefficient::new(r, g, b)
    }

    pub fn get_components(&self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    pub fn into_rgb(self) -> Result<(u8, u8, u8), RayTracingError> {
        let Color { r, g, b } = self.new_from_color()?;
        let r = (r * u8::MAX as f32) as u8;
        let g = (g * u8::MAX as f32) as u8;
        let b = (b * u8::MAX as f32) as u8;
        Ok((r, g, b))
    }
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        // Rust 1.45 introduced overflow control with the as keyword
        // setting the value to the crossed bound
        let r = (self.r as f64 * rhs) as f32;
        let g = (self.g as f64 * rhs) as f32;
        let b = (self.b as f64 * rhs) as f32;

        Color { r, g, b }
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        rhs * self
    }
}

impl Add for &Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        let r = self.r + rhs.r;
        let g = self.g + rhs.g;
        let b = self.b + rhs.b;
        Color { r, g, b }
    }
}

impl Mul for &Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        let r = self.r * rhs.r;
        let g = self.g * rhs.g;
        let b = self.b * rhs.b;
        Color { r, g, b }
    }
}

#[allow(dead_code)]
pub const BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
};
#[allow(dead_code)]
pub const WHITE: Color = Color {
    r: 1.,
    g: 1.,
    b: 1.,
};
#[allow(dead_code)]
pub const RED: Color = Color {
    r: 1.,
    g: 0.,
    b: 0.,
};
#[allow(dead_code)]
pub const GREEN: Color = Color {
    r: 0.,
    g: 1.,
    b: 0.,
};
#[allow(dead_code)]
pub const BLUE: Color = Color {
    r: 0.,
    g: 0.,
    b: 1.,
};

#[derive(Clone, Copy, Debug)]
pub struct DiffusionCoefficient {
    dr: f32, // should be between 0 and 1
    dg: f32,
    db: f32,
}

impl DiffusionCoefficient {
    pub fn new(dr: f32, dg: f32, db: f32) -> Result<Self, RayTracingError> {
        if !(0. ..=1.).contains(&dr) || !(0. ..=1.).contains(&dg) || !(0. ..=1.).contains(&db) {
            Err(RayTracingError::DiffusionCoefficientOOB(dr, dg, db))
        } else {
            Ok(DiffusionCoefficient { dr, dg, db })
        }
    }
}

impl Mul<&Color> for &DiffusionCoefficient {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        let r = rhs.r * self.dr;
        let g = rhs.g * self.dg;
        let b = rhs.b * self.db;

        Color { r, g, b }
    }
}

impl Mul<&DiffusionCoefficient> for &Color {
    type Output = Color;
    fn mul(self, rhs: &DiffusionCoefficient) -> Self::Output {
        rhs * self
    }
}

// ! deprecated since we use a cos weighted distribution and random bounces instead of searching for all light sources
// * indeed, searching for light sources in the scene works if using point lights but does not work for area lights (as that would be an infinity of points)
// pub fn diffused_color(
//     source_color: Color,
//     object_diffusion_coefficient: DiffusionCoefficient,
//     source_ray: Ray,
//     surface_normal_vector: Vector,
// ) -> Result<Color, RayTracingError> {
//     // ! start by checking if source is visible from the point
//     // since we only have spheres for now, this is done by checking the scalar product normal.ray is <= 0
//     // source is above the horizon if normal.(ray from surface to source) >= 0, since we have the ray from source to surface it's the opposite
//     let vector_from_surface_to_source = -1. * &source_ray.direction.to_vector();
//     let normal_ray_scalar_prod =
//         surface_normal_vector.scalar_product(&vector_from_surface_to_source);
//     if normal_ray_scalar_prod <= 0. {
//         Err(RayTracingError::SourceNotVisibleFromPoint(format!("Source has ray : {0:?} | Surface normal vector is : {1:?} | Their scalar product is {2}", source_ray, surface_normal_vector, normal_ray_scalar_prod)))
//     } else {
//         // we use a emisphere cos weighted distribution for rays directions, meaning we don't correct the amount of light received here to follow Lambert Reflectance law
//         // this correction is done by the cos weighted distribution
//         Ok(&source_color * &object_diffusion_coefficient)
//     }
// }
