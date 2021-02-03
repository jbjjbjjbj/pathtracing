pub mod core;
pub mod camera;
mod scene;

use std::ops::{Add, Sub, Mul, DivAssign};
use std::cmp;

/// Trait used to implement generics
///
/// This is used in Bound and Vectors
pub trait Number:
    Copy +
    Default +
    cmp::PartialOrd +
    Sub<Output = Self> +
    Add<Output = Self> +
    Mul<Output = Self> +
    DivAssign
{}

impl Number for i32 {}
impl Number for f32 {}

/// Used for representing floating point values throughout the program
/// 
/// A higher precision type will require more ram
pub type Float = f32;
