pub mod complex_numbers {
    use std::fmt::Debug;
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::Sub;

    #[derive(Clone)]
    pub struct Complex<T> {
        re: T,
        im: T,
    }

    impl<T> Complex<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone + Default,
    {
        pub fn new(re: T, im: T) -> Self {
            Complex { re, im }
        }

        pub fn re(self) -> T {
            return self.re;
        }

        pub fn im(self) -> T {
            return self.im;
        }
    }

    impl<T> Add for Complex<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone + Default,
    {
        type Output = Complex<T>;

        fn add(self, rhs: Complex<T>) -> Self::Output {
            Complex::new(self.re + rhs.re, self.im + rhs.im)
        }
    }

    impl<T> Mul for Complex<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone + Default,
    {
        type Output = Complex<T>;

        fn mul(self, rhs: Self) -> Self::Output {
            let real_part: T = self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone();
            let imaginary_part: T = self.re * rhs.im + self.im * rhs.re;

            return Complex::new(real_part, imaginary_part);
        }
    }

    impl<T> Sub for Complex<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone + Default,
    {
        type Output = Complex<T>;

        fn sub(self, rhs: Self) -> Self::Output {
            Complex::new(self.re - rhs.re, self.im - rhs.im)
        }
    }

    impl<T> Default for Complex<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone + Default,
    {
        fn default() -> Self {
            Self {
                re: T::default(),
                im: T::default(),
            }
        }
    }

    impl<T> std::fmt::Debug for Complex<T>
    where
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Clone + Default + Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}+{:?}i", self.re, self.im)
        }
    }

    impl<T> PartialEq for Complex<T>
    where
        T: PartialEq,
    {
        fn eq(&self, other: &Self) -> bool {
            self.re == other.re && self.im == other.im
        }

        fn ne(&self, other: &Self) -> bool {
            !self.eq(other)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::complex_numbers::Complex;

    #[test]
    fn test_complex_initialization() {
        let _complex_i32: Complex<i32> = Complex::new(1, 2);
        let _complex_f32: Complex<f32> = Complex::new(2.5, 1.325);
        let _complex_usize: Complex<usize> = Complex::new(1, 1);
        let complex_from_default: Complex<i32> = Complex::default();

        assert_eq!(complex_from_default, Complex::<i32>::new(0, 0));
    }

    #[test]
    fn test_complex_addition() {
        let z_1: Complex<i32> = Complex::new(1, 2);
        let z_2: Complex<i32> = Complex::new(1, -3);

        assert_eq!(z_1 + z_2, Complex::new(2, -1));

        let z_1_f: Complex<f32> = Complex::new(2.12, 6.5);
        let z_2_f: Complex<f32> = Complex::new(3.5, 2.1);

        assert_eq!(z_1_f + z_2_f, Complex::new(5.62, 8.6));
    }

    #[test]
    fn test_complex_multiplication() {
        let z_1: Complex<i32> = Complex::new(2, -2);
        let z_2: Complex<i32> = Complex::new(9, 17);
        assert_eq!(z_1 * z_2, Complex::new(52, 16));
    }
}
