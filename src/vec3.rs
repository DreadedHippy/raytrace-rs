use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub}};

/// A struct for implementing geometric vectors
#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new() -> Self {
        Self{e: [0.0, 0.0, 0.0]}
    }

    pub fn from_xyz(e0: f64, e1: f64, e2: f64) -> Self {
        Self{e: [e0, e1, e2]}
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




