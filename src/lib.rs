//! # Complex numbers implementation
//! Created for checking how traits and generics works in rust


/// A Complex number represented here
#[derive(Default)]
struct Complex<T> {
    /// real part
    re: T,
    /// imaginary part
    im: T,
}

/// creating Complex number from other types
impl<T> Complex<T> {
    fn new (re: T, im: T) -> Self {
        Complex{re, im}
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
    }
}