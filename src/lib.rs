//! # Complex numbers implementation
//! Created for checking how traits and generics works in rust

use std::ops::Add;
use std::fmt::{Display, Formatter, Result};


#[derive(Default, Debug, PartialEq, Copy, Clone)]
/// A Complex number represented here
pub struct Complex<T> {
    /// real part
    re: T,
    /// imaginary part
    im: T,
}

impl<T> Complex<T> {
    /// create Complex<T>
    pub fn new (re: T, im: T) -> Self {
        Complex{re, im}
    }
}


impl<T: Add<Output = T>> Add for Complex<T> {
    /// return complex number
    type Output = Self;
    /// you can complex numbers For example (x_1, y_1i) + (x_2, y_2i) = ((x_1+x_2), (y_1+y_2)i)
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    /// generate Complex from tulpe (T, T)
    fn from(data: (T, T)) -> Self {
        Complex {re: data.0, im: data.1}
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;

    #[test]
    fn complex_basics() {
        let first = Complex::new(3,5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re, 3);
        assert_eq!(first.im, 5);
        assert_eq!(second.re, second.im);
        println!("{:?}", first);
    }

    #[test]
    fn complex_add_test() {
        let first = Complex::new(3,5);
        let second: Complex<i32> = Complex::default();
        let res = first + second;
        assert_eq!(res, first);
    }

    #[test]
    fn complex_display() {
        let my_imaginary = Complex::new(2345, 456);
        println!("{}", my_imaginary);
    }
}