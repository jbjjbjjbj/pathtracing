use crate::{Float, Number};
use std::ops::{Sub, Add, DivAssign};

#[derive(Clone, Copy)]
pub struct Vector3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vector3f = Vector3<Float>;

impl<T: Number> Vector3<T> {
    pub fn new(initial: T) -> Vector3<T> {
        Vector3 { 
            x: initial,
            y: initial,
            z: initial,
        }
    }

    pub fn new_xyz(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z}
    }
}

impl<T: Number> Sub for Vector3<T> {
    type Output = Self;
    fn sub(self, op: Self) -> Self::Output {
        Self::new_xyz(
            self.x - op.x,
            self.y - op.y,
            self.z - op.z,
        )
    }
}

impl<T: Number> Add for Vector3<T> {
    type Output = Self;
    fn add(self, op: Self) -> Self::Output {
        Self::new_xyz(
            self.x + op.x,
            self.y + op.y,
            self.z + op.z,
        )
    }
}

impl<T: Number> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, op: T) {
        self.x /= op;
        self.y /= op;
        self.z /= op;
    }
}

impl Vector3f {
    pub fn len_squared(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> Float {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, op: &Self) -> Float {
        self.x * op.x + self.y * op.y + self.z * op.z
    }

    pub fn norm_in(&mut self) {
        let len = self.len();
        if len == 0.0 {
            *self = Self::new(0.0);
        }

        *self /= len;
    }

    pub fn norm(&self) -> Self {
        let mut new = self.clone();
        new.norm_in();
        new
    }

    pub fn cross(&self, op: &Self) -> Self {
        Self::new_xyz(
            self.y * op.z - self.z * op.y,
            self.z * op.x - self.x * op.z,
            self.x * op.y - self.y * op.x,
            )

    }
}