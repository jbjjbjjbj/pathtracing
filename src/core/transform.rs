//! Implements matrix tranformations
//!
//! Used for placing shapes and camera into the scene.
//! Provides some common initializations methods for transforms like rotation, translate and camera
//! placement.
//!
//! # Examples
//!
//! ```
//! use pathtrace::core::{Vector3f, Transform};
//!
//! let t = Transform::new_translate(3.0, 5.0, 6.0);
//! let v = t.eval_point(&Vector3f::new_xyz(1.0, 1.0, 1.0));
//!
//! assert!(v.x == 4.0 && v.y == 6.0 && v.z == 7.0);
//! ```
use super::matrix4x4::Matrix4x4f;
use crate::Float;
use crate::core::Vector3f;
use std::ops;

pub struct Transform {
    m: Matrix4x4f,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            m: Matrix4x4f::new_ident(1.0),
        }
    }

    /// Evaluation a point through the matrix
    pub fn eval_point(&self, p: &Vector3f) -> Vector3f {
        let m = &self.m.m;
        let x = m[0][0]*p.x + m[0][1]*p.y + m[0][2]*p.z + m[0][3];
        let y = m[1][0]*p.x + m[1][1]*p.y + m[1][2]*p.z + m[1][3];
        let z = m[2][0]*p.x + m[2][1]*p.y + m[2][2]*p.z + m[2][3];
        let w = m[3][0]*p.x + m[3][1]*p.y + m[3][2]*p.z + m[3][3];

        let mut out = Vector3f::new_xyz(x, y, z);
        if w != 1.0 {
            out /= w;
        }

        out
    }

    /// Evaluation of a vector
    ///
    /// This will not work for normal vectors as they become distorted
    pub fn eval_vector(&self, v: &Vector3f) -> Vector3f {
        let m = &self.m.m;
        let x = m[0][0]*v.x + m[0][1]*v.y + m[0][2]*v.z;
        let y = m[1][0]*v.x + m[1][1]*v.y + m[1][2]*v.z;
        let z = m[2][0]*v.x + m[2][1]*v.y + m[2][2]*v.z;

        Vector3f::new_xyz(x, y, z)
    }

    pub fn inverse(&self) -> Self {
        Transform {
            m: self.m.inverse(),
        }
    }
}

impl ops::Mul for Transform {
    type Output = Transform;

    fn mul(self, op: Self) -> Self::Output {
        Transform {
            m: &self.m * &op.m
        }
    }
}

// Creation of different transformations
impl Transform {
    pub fn new_translate(x: Float, y: Float, z: Float) -> Self {
        Transform { m: Matrix4x4f::new(
                1.0, 0.0, 0.0, x,
                0.0, 1.0, 0.0, y,
                0.0, 0.0, 1.0, z,
                0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn new_scale(x: Float, y: Float, z: Float) -> Self {
        Transform { m: Matrix4x4f::new(
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn new_rotate_x(theta: Float) -> Self {
        let theta = theta.to_radians();
        let cost = theta.cos();
        let sint = theta.sin();
        Transform { m: Matrix4x4f::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, cost, -sint, 0.0,
                0.0, sint, cost, 0.0,
                0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn new_rotate_y(theta: Float) -> Self {
        let theta = theta.to_radians();
        let cost = theta.cos();
        let sint = theta.sin();
        Transform { m: Matrix4x4f::new(
                cost, 0.0, sint, 0.0,
                0.0, 1.0, 0.0, 0.0,
                -sint, 0.0, cost, 0.0,
                0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn new_rotate_z(theta: Float) -> Self {
        let theta = theta.to_radians();
        let cost = theta.cos();
        let sint = theta.sin();
        Transform { m: Matrix4x4f::new(
                cost, -sint, 0.0, 0.0,
                sint, cost, 0.0, 0.0,
                0.0, 0.0, 1.1, 0.0,
                0.0, 0.0, 0.0, 1.0)
        }
    }

    pub fn new_look_at(pos: &Vector3f, look: &Vector3f, up: &Vector3f) -> Self {
        let dir = (*look - *pos).norm();  // This is what the z axis should map to
        let right = up.norm().cross(&dir).norm();
        let newup = dir.cross(&right);

        Transform { m: Matrix4x4f::new(
                right.x, newup.x, dir.x, pos.x,
                right.y, newup.y, dir.y, pos.y,
                right.z, newup.z, dir.z, pos.z,
                0.0    , 0.0    , 0.0  , 1.0)
        }
    }
}
