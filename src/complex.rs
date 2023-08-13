use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub im: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let im = self.im + rhs.im;

        Self { real, im }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.real;
        let c = rhs.real;
        let b = self.im;
        let d = rhs.im;

        let real = (a * c) - (b * d);
        let im = (a * d) + (c * b);

        Self { real, im }
    }
}

impl Complex {
    pub fn new(real: f64, im: f64) -> Self {
        Self { real, im }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn i() -> Self {
        Self::new(0.0, 1.0)
    }

    pub fn magnitude(&self) -> f64 {
        let a = self.real;
        let b = self.im;

        return f64::sqrt(a * a + b * b)
    }
}

#[cfg(test)]
mod test {
    mod add {
        use crate::complex::Complex;

        #[test]
        fn add_two_reals() {
            let a = Complex::new(2.0, 0.0);
            let b = Complex::new(5.0, 0.0);
            let c = Complex::new(-2.0, 0.0);

            assert_eq!(a + b, Complex::new(7.0, 0.0));
            assert_eq!(a + c, Complex::zero())
        }

        #[test]
        fn add_two_complex() {
            let a = Complex::new(2.0, 3.0);
            let b = Complex::new(4.0, -4.0);

            assert_eq!(a + b, Complex::new(6.0, -1.0))
        }
    }

    mod mul {
        use crate::complex::Complex;

        #[test]
        fn mul_two_reals() {
            let a = Complex::new(2.0, 0.0);
            let b = Complex::new(5.0, 0.0);
            let mul = a * b;
            let expected = Complex::new(10.0, 0.0);

            assert_eq!(mul, expected, "expected: {expected:?}, actual: {mul:?}");
        }

        #[test]
        fn mul_two_complexes() {
            let a = Complex::new(2.0, 3.0);
            let b = Complex::new(5.0, 6.0);

            let mul = a * b;
            let expected = Complex::new(-8.0, 27.0);

            assert_eq!(mul, expected, "expected: {expected:?}, actual: {mul:?}");
        }
    }

    mod magnitude {
        use crate::complex::Complex;

        #[test]
        fn it_should_return_the_correct_value() {
            assert_eq!(Complex::new(2.0, 3.0).magnitude(), f64::sqrt(13.0))
        }
    }
}
