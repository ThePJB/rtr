use crate::impl_vec;
use std::cmp::PartialEq;
use super::*;

pub trait Lerp {
    fn lerp(self, other: Self, t: f32) -> Self;
}
impl Lerp for f64 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self * (1.0 - t as Self) + other * t as Self
    }
}
impl Lerp for f32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }
}

// Define the Floor trait
pub trait Floor {
    fn floor(&self) -> Self;
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {
    pub fn projx(&self) -> Vec2 {
        vec2(self.x, 0.0)
    }
    pub fn projy(&self) -> Vec2 {
        vec2(0.0, self.y)
    }
    pub fn rotate(&self, theta: f32) -> Vec2 {
        let c = theta.cos();
        let s = theta.sin();
        let c1 = vec2(c, s);
        let c2 = vec2(-s, c);
        vec2(c1.dot(&self), c2.dot(&self))
    }
    pub fn extend(&self, z: f32) -> Vec3 {
        vec3(self.x, self.y, z)
    }
    pub fn yx(&self) -> Vec2 {
        vec2(self.y, self.x)
    }
    pub fn norm(&self) -> f32 {
        self.dot(&self).sqrt()
    }
}

// Implement the Floor trait for Vec2
impl Floor for Vec2 {
    fn floor(&self) -> Self {
        vec2(self.x.floor(), self.y.floor())
    }
}


// yeh vec macros missing some stuff isnt it like some operator overloads with scalar, norm etc

impl_vec!(Vec2, f32, x, y);


#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

impl_vec!(Vec3, f32, x, y, z);

impl Vec3 {
    pub fn xy(&self) -> Vec2 {
        vec2(self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

impl_vec!(Vec4, f32, x, y, z, w);

#[derive(Clone, Copy, Debug, Eq, Hash, Default)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}

pub trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

impl AsVec2 for IVec2 {
    fn as_vec2(&self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}

impl_vec!(IVec2, i32, x, y);

// vecs refactor, colour is ivec4 of u8s, etc
