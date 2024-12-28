use super::Multivec2D;
use std::ops::{Add, Sub, Mul};

// These are unfortunately necessary for left addition and multiplication with a scalar

impl Add<Multivec2D<f64>> for f64 {
    type Output = Multivec2D<f64>;

    fn add(self, rhs: Multivec2D<f64>) -> Self::Output {
        rhs + self
    }
}

impl Add<Multivec2D<i64>> for i64 {
    type Output = Multivec2D<i64>;

    fn add(self, rhs: Multivec2D<i64>) -> Self::Output {
        rhs + self
    }
}

impl Sub<Multivec2D<f64>> for f64 {
    type Output = Multivec2D<f64>;

    fn sub(self, rhs: Multivec2D<f64>) -> Self::Output {
        -rhs + self
    }
}

impl Sub<Multivec2D<i64>> for i64 {
    type Output = Multivec2D<i64>;

    fn sub(self, rhs: Multivec2D<i64>) -> Self::Output {
        -rhs + self
    }
}

impl Mul<Multivec2D<f64>> for f64 {
    type Output = Multivec2D<f64>;

    fn mul(self, rhs: Multivec2D<f64>) -> Self::Output {
        rhs * self
    }
}

impl Mul<Multivec2D<i64>> for i64 {
    type Output = Multivec2D<i64>;

    fn mul(self, rhs: Multivec2D<i64>) -> Self::Output {
        rhs * self
    }
}
