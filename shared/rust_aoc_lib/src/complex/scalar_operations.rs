use super::Complex;
use std::ops::{Add, Sub, Mul};

// These are unfortunately necessary for left addition and multiplication with a scalar

impl Add<Complex<f32>> for f32 {
    type Output = Complex<f32>;

    fn add(self, rhs: Complex<f32>) -> Self::Output {
        rhs + self
    }
}

impl Add<Complex<i32>> for i32 {
    type Output = Complex<i32>;

    fn add(self, rhs: Complex<i32>) -> Self::Output {
        rhs + self
    }
}

impl Sub<Complex<f32>> for f32 {
    type Output = Complex<f32>;

    fn sub(self, rhs: Complex<f32>) -> Self::Output {
        -rhs + self
    }
}

impl Sub<Complex<i32>> for i32 {
    type Output = Complex<i32>;

    fn sub(self, rhs: Complex<i32>) -> Self::Output {
        -rhs + self
    }
}

impl Mul<Complex<f32>> for f32 {
    type Output = Complex<f32>;

    fn mul(self, rhs: Complex<f32>) -> Self::Output {
        rhs * self
    }
}

impl Mul<Complex<i32>> for i32 {
    type Output = Complex<i32>;

    fn mul(self, rhs: Complex<i32>) -> Self::Output {
        rhs * self
    }
}
