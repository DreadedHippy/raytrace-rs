use core::f64;
use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub}};

use crate::rand::{random_f64, random_f64_range};

/// A struct for implementing geometric vectors
#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new() -> Self {
        Self{e: [0.0, 0.0, 0.0]}
    }

    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        Self{e: [x, y, z]}
    }

    pub fn x(&self) -> f64 {self.e[0]}
    pub fn y(&self) -> f64 {self.e[1]}
    pub fn z(&self) -> f64 {self.e[2]}

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let e = self.e;
        return (e[0]*e[0]) + (e[1]*e[1]) + (e[2]*e[2]);
    }

    pub fn random() -> Self {
        return Self::from_xyz(random_f64(), random_f64(), random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        return Self::from_xyz(random_f64_range(min, max), random_f64_range(min, max), random_f64_range(min, max))
    }

    pub fn dot(&self, other: &Self) -> f64{
        let u = self.e;
        let v = other.e;

        return u[0] * v[0]
            + u[1] * v[1]
            + u[2] * v[2];
    }

    pub fn cross(&self, other: &Self) -> Vec3 {
        let u = self.e;
        let v = other.e;

        return Self::from_xyz(
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0]
        )
    }

    pub fn unit_vector(v: &Vec3) -> Self {
      return *v / v.length()
    }

    pub fn iter(&self) -> Vec3Iterator {
        Vec3Iterator { vec3: self, index: 0 }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];


    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs;
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = self.e;
        writeln!(f, "{} {} {}", e[0], e[1], e[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let u = self.e;
        let v = rhs.e;
        Self::from_xyz(u[0]+v[0], u[1]+v[1], u[2]+v[2])
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let u = self.e;
        let v = rhs.e;
        Self::from_xyz(u[0]-v[0], u[1]-v[1], u[2]-v[2])
    }
}


impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let u = self.e;
        let v = rhs.e;
        Self::from_xyz(u[0]*v[0], u[1]*v[1], u[2]*v[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let u = self.e;
        let t = rhs;
        Self::from_xyz(u[0]*t, u[1]*t, u[2]*t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let e = self.e;
        Self{e: [-e[0], -e[1], -e[2]]}
    }
}



pub struct Vec3Iterator<'a> {
    vec3: &'a Vec3,
    index: usize
}

impl<'a> Iterator for Vec3Iterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        // for the 3 fields x, y, z
        if self.index < 3 {
            let res = match self.index {
                0 => Some(self.vec3.x()),
                1 => Some(self.vec3.y()),
                2 => Some(self.vec3.z()),
                _ => None
            };
            self.index += 1;
            res
        } else {
            None
        }
    }
}


pub type Point3 = Vec3;

/// Returns a random unit vector
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.length_squared();
        // black hole point
        let blackhole = 1e-160;
        if (blackhole < lensq && lensq <= 1.0) {
            return p/lensq.sqrt()
        }
    }
}


pub fn random_on_hemisphere(normal: &Vec3) -> Vec3{
    // get random vector satisfying criteria
    let on_unit_sphere = random_unit_vector();
    // check angle between random and normal, if > 0 i.e, same hemisphere as normal
    if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
        // return
        return on_unit_sphere
    } else {
        // invert the vector, and now it definitely is in the same hemisphere as normal
        return -on_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    // reflected ray direction of a ray v = v + 2b where b is height of v parallel to the normal
    // n is unit vector of len 1 but v may not be
    // To get b, we scale normal vector by the length of the projection of v onto n
    // basically perpendicular height of v as said before
    // this is given by dot product of v and n
    // if n were not a unit vector, we'd also need to divide this dot product by length of n i.e, normalize it
    // now v point onto the surface and we want the reflection to point out of the surface
    // so v+2b becomes v-2b
    return *v - (2.0 * (Vec3::dot(v, n) * *n))
}
