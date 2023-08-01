use num_traits::Float;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct Vec3<T>
where
    T: Float
        + Copy
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
}

impl<T> Add for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> Index<usize> for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

impl<T> Mul for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    type Output = T;

    fn mul(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T> Neg for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Vec3<T>
where
    T: Copy
        + Float
        + Default
        + Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>,
{
    pub(crate) const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub(crate) fn norm(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub(crate) fn normalized(&self) -> Self {
        let length = self.norm();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}
