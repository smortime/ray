use core::{fmt, panic};
use std::{cmp, ops};

#[derive(Copy, Clone, Debug)]
pub(crate) struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub(crate) fn origin() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub(crate) fn x(&self) -> f64 {
        self.x
    }

    pub(crate) fn y(&self) -> f64 {
        self.y
    }

    pub(crate) fn z(&self) -> f64 {
        self.z
    }

    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn length_squared(self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
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

    pub(crate) fn unit_vector(self) -> Vec3 {
        self / self.length()
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
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index must be < 3"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index must be < 3"),
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs[0] * self, rhs[1] * self, rhs[2] * self)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

// Aliases for reasons!
pub(crate) type Point3 = Vec3;

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn test_basic_vec3_usage() {
        // constructors
        assert_eq!(Vec3::origin(), Vec3::new(0.0, 0.0, 0.0));

        // accessors
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);

        // calculations
        let v_sq = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v_sq.length(), 3.0);
        assert_eq!(v_sq.length_squared(), 9.0);
        assert_eq!(v_sq.dot(&v), 11.0);
        assert_eq!(v_sq.cross(&v), Vec3::new(2.0, -1.0, 0.0));
        assert_eq!(
            v_sq.unit_vector(),
            Vec3::new(1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0)
        );

        // ops
        assert_eq!(-Vec3::new(1.0, 2.0, -3.0), Vec3::new(-1.0, -2.0, 3.0));

        let x = Vec3::new(1.0, 2.0, 3.0);
        let y = Vec3::new(3.0, 2.0, 1.0);

        assert_eq!(x + y, Vec3::new(4.0, 4.0, 4.0));
        assert_eq!(x * y, Vec3::new(3.0, 4.0, 3.0));
        assert_eq!(x - y, Vec3::new(-2.0, 0.0, 2.0));

        assert_eq!(x * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(x / 2.0, Vec3::new(0.5, 1.0, 1.5));

        let mut x_mut = Vec3::new(1.0, 2.0, 3.0);
        x_mut += y;
        assert_eq!(x_mut, Vec3::new(4.0, 4.0, 4.0));
        x_mut = Vec3::new(1.0, 2.0, 3.0);
        x_mut *= 2.0;
        assert_eq!(x_mut, Vec3::new(2.0, 4.0, 6.0));
        x_mut = Vec3::new(1.0, 2.0, 3.0);
        x_mut /= 2.0;
        assert_eq!(x_mut, Vec3::new(0.5, 1.0, 1.5))
    }
}
