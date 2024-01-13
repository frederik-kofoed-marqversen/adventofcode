use std::fmt::{Debug, Display};
use std::iter::zip;
use std::ops::{Add, Sub, Mul, Div};

// Used to define the identity elements
pub trait Scalar<T> {
    const ONE: T;
    const ZERO: T;
}

#[derive(Debug, PartialEq, Hash, Default, Copy, Clone)]
pub struct Multivec2D<T> {
    data: [T; 4],
}

impl<T> Multivec2D<T>
where
    T: Scalar<T> + PartialEq + Debug,
{
    pub const E1: Self = Self {
        data: [T::ZERO, T::ONE, T::ZERO, T::ZERO],
    };
    pub const E2: Self = Self {
        data: [T::ZERO, T::ZERO, T::ONE, T::ZERO],
    };
    pub const I: Self = Self {
        data: [T::ZERO, T::ZERO, T::ZERO, T::ONE],
    };

    pub fn vector(s1: T, s2: T) -> Self {
        Self {
            data: [T::ZERO, s1, s2, T::ZERO],
        }
    }

    pub fn grade_project(mut self, grade: usize) -> Self {
        match grade {
            0 => {
                self.data[1..].iter_mut().for_each(|val| *val = T::ZERO);
            }
            1 => {
                self.data[0] = T::ZERO;
                self.data[3] = T::ZERO;
            }
            2 => {
                self.data[..3].iter_mut().for_each(|val| *val = T::ZERO);
            }
            _ => panic!("Grade {grade} is out of bounds."),
        };
        return self;
    }

    pub fn grade(&self) -> Option<usize> {
        // If homogenous; return the grade, otherwise return None.
        let is_zero: [bool; 4] = self
            .data
            .iter()
            .map(|x| x == &T::ZERO)
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap();
        match is_zero {
            [_, true, true, true] => Some(0),
            [true, true, true, _] => Some(2),
            [true, _, _, true] => Some(1),
            _ => None,
        }
    }
}

impl<T> From<T> for Multivec2D<T>
where
    T: Scalar<T>
{
    fn from(value: T) -> Self {
        Self {
            data: [value, T::ZERO, T::ZERO, T::ZERO],
        }
    }
}

impl<T> From<[T; 2]> for Multivec2D<T>
where
    T: Scalar<T> + Copy
{
    fn from(arr: [T; 2]) -> Self {
        Self {
            data: [T::ZERO, arr[0], arr[1], T::ZERO],
        }
    }
}

impl<T> Add for Multivec2D<T>
where
    T: Debug + Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            data: zip(self.data, other.data)
                .map(|(a, b)| a + b)
                .collect::<Vec<T>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<T> Add<T> for Multivec2D<T>
where
    T: Scalar<T> + PartialEq + Debug + Add<Output = T>,
{
    type Output = Self;

    fn add(self, value: T) -> Self {
        self + Self::from(value)
    }
}

impl<T> Sub for Multivec2D<T>
where
    T: Debug + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: zip(self.data, other.data)
                .map(|(a, b)| a - b)
                .collect::<Vec<T>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<T> Mul for Multivec2D<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] * other.data[0]
                    + self.data[1] * other.data[1]
                    + self.data[2] * other.data[2]
                    - self.data[3] * other.data[3],
                self.data[0] * other.data[1]
                    + self.data[1] * other.data[0]
                    + self.data[3] * other.data[2]
                    - self.data[2] * other.data[3],
                self.data[0] * other.data[2]
                    + self.data[2] * other.data[0]
                    + self.data[1] * other.data[3]
                    - self.data[3] * other.data[1],
                self.data[0] * other.data[3]
                    + self.data[3] * other.data[0]
                    + self.data[1] * other.data[2]
                    - self.data[2] * other.data[1],
            ],
        }
    }
}

impl<T> Mul<T> for Multivec2D<T>
where
    T: Mul<Output = T> + Debug + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            data: self
                .data
                .iter()
                .map(|&x| x * rhs)
                .collect::<Vec<T>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<T> Div<T> for Multivec2D<T>
where
    T: Div<Output = T> + Debug + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            data: self
                .data
                .iter()
                .map(|&x| x / rhs)
                .collect::<Vec<T>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl<T> Display for Multivec2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} + {} * e1 + {} * e2 + {} * e1 ∧ e2",
            self.data[0], self.data[1], self.data[2], self.data[3]
        )
    }
}

// Specific type implementations

impl Scalar<f64> for f64 {
    const ONE: f64 = 1.0;
    const ZERO: f64 = 0.0;
}

impl Scalar<i64> for i64 {
    const ONE: i64 = 1;
    const ZERO: i64 = 0;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        let e1 = Multivec2D::<i64>::E1;
        let e2 = Multivec2D::<i64>::E2;

        assert_eq!(
            1 + 4 * (e1 + e1 * e2) / 2,
            Multivec2D {
                data: [1, 2, 0, 2]
            }
        );
    }

    #[test]
    fn complex_numbers() {
        let i = Multivec2D::<i64>::I;
        
        assert_eq!(
            (1 + 2 * i) * i,
            Multivec2D {
                data: [-2, 0, 0, 1]
            }
        );

        assert_eq!(
            Multivec2D::from([1, 2]) * i,
            Multivec2D {
                data: [0, -2, 1, 0]
            }
        );
    }

    #[test]
    fn grade() {
        let a = Multivec2D::<i64> {
            data: [1, 2, 3, 4]
        };

        assert_eq!(
            a.grade_project(0),
            Multivec2D {
                data: [1, 0, 0, 0]
            }
        );

        assert_eq!(
            a.grade_project(1),
            Multivec2D {
                data: [0, 2, 3, 0]
            }
        );

        assert_eq!(
            a.grade_project(2),
            Multivec2D {
                data: [0, 0, 0, 4]
            }
        );

        assert_eq!(
            a.grade_project(1).grade(),
            Some(1)
        )
    }
}