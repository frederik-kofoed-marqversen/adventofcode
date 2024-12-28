// Used to define the identity elements
pub trait Scalar<T> {
    const ONE: T;
    const ZERO: T;
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
