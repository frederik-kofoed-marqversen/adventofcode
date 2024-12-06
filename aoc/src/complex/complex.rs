use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Default, Copy, Clone)]
pub struct Complex<T> {
    pub real: T,
    pub imag: T,
}

impl<T> Complex<T>
where
    T: Neg<Output = T> + Copy,
{
    pub fn conj(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl<T> Display for Complex<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {} * i", self.real, self.imag)
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl<T> Add<T> for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, value: T) -> Self {
        Self {
            real: self.real + value,
            imag: self.imag,
        }
    }
}

impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl<T> Sub<T> for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, value: T) -> Self {
        Self {
            real: self.real - value,
            imag: self.imag,
        }
    }
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl<T> Mul<T> for Complex<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}

impl<T> Div for Complex<T>
where
    T: Mul<Output = T>
        + Div<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Copy,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.conj() / (other.real * other.real + other.imag * other.imag)
    }
}

impl<T> Div<T> for Complex<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
