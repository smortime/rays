use core::fmt;
use std::ops;

use crate::utility::{random_f64, random_f64_range};

pub(crate) type Point3 = Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub(crate) fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub(crate) fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub(crate) fn random() -> Self {
        Self::new(random_f64(), random_f64(), random_f64())
    }

    pub(crate) fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
        )
    }

    pub(crate) fn random_unit_vector() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub(crate) fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub(crate) fn x(&self) -> f64 {
        self[0]
    }

    pub(crate) fn y(&self) -> f64 {
        self[1]
    }

    pub(crate) fn z(&self) -> f64 {
        self[2]
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn dot(&self, rhs: &Vec3) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }

    pub(crate) fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    pub(crate) fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub(crate) fn reflection(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * *n
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 2 {
            panic!("Index must be less than 3");
        }
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e: [
                self[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        };
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn basic_vec3_tests() {
        // constructors
        assert_eq!(Vec3::origin(), Vec3 { e: [0.0, 0.0, 0.0] });

        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3 { e: [1.0, 2.0, 3.0] });

        // accessors
        assert_eq!(v.x(), v[0]);
        assert_eq!(v.y(), v[1]);
        assert_eq!(v.z(), v[2]);

        // lengths
        assert_eq!(v.length_squared(), 14.0);
        assert_eq!(v.length(), 14.0_f64.sqrt());

        // dot & cross
        let u = Vec3::new(3.0, 1.0, 2.0);
        assert_eq!(v.dot(&u), 11.0);
        assert_eq!(v.cross(&u), Vec3::new(1.0, 7.0, -5.0));

        // ops
        assert_eq!(-v, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(v - u, Vec3::new(-2.0, 1.0, 1.0));
        assert_eq!(v + u, Vec3::new(4.0, 3.0, 5.0));
        assert_eq!(v * u, Vec3::new(3.0, 2.0, 6.0));
        assert_eq!(v * 3.0, Vec3::new(3.0, 6.0, 9.0));
        assert_eq!(v / 2.0, Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(3.0 * v, Vec3::new(3.0, 6.0, 9.0));

        // mut ops
        let mut x = Vec3::origin();
        x += v;
        assert_eq!(v, v);
        let mut x = Vec3::new(3.0, 2.0, 1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(6.0, 4.0, 2.0));
        let mut x = Vec3::new(3.0, 2.0, 1.0);
        x /= 2.0;
        assert_eq!(x, Vec3::new(1.5, 1.0, 0.5));
        let mut x = Vec3::new(1.0, 1.0, 1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 2.0, 2.0));
        let mut x = Vec3::new(1.0, 1.0, 1.0);
        x /= 2.0;
        assert_eq!(x, Vec3::new(0.5, 0.5, 0.5));
        let mut x = Vec3::new(3.0, 2.0, 1.0);
        x += Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(x, Vec3::new(4.0, 4.0, 4.0));
    }

    #[test]
    #[should_panic]
    fn test_vec3_index_should_panic() {
        let v = Vec3::origin();
        v[4];
    }
}
