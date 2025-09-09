use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl Vector3<i32> {
    pub fn magnitude(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        (x * x + y * y + z * z).sqrt()
    }
}

impl Vector3<f64> {
    pub fn magnitude(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        let z = self.z as f64;
        (x * x + y * y + z * z).sqrt()
    }
}

impl<T> Add for Vector3<T>
where
    T: Add<Output = T>,
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

impl<T> Sub for Vector3<T>
where
    T: Sub<Output = T>,
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
