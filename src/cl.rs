mod cl1;
mod cl2;
mod cl3;
mod cl4;
mod cl5;
mod cl6;
mod cln;

use num::complex::Complex;
use polylog::Li;


pub trait Cl<T> {
    /// Returns the n-th order Standard Clausen function.
    ///
    /// # Example:
    /// ```
    /// use clausen::Cl;
    /// use num::complex::Complex;
    ///
    /// assert!((1.0.cl(2) - 1.0139591323607685).abs() < 1e-15);
    /// assert!((Complex::new(1.0, 1.0).cl(2) - Complex::new(1.4107754938116412, -0.1044778629291566)).norm() < 1e-15);
    /// ```
    fn cl(&self, n: i32) -> T;
}


impl Cl<f64> for f64 {
    fn cl(&self, n: i32) -> f64 {
        match n {
            1 => cl1::cl1(*self),
            2 => cl2::cl2(*self),
            3 => cl3::cl3(*self),
            4 => cl4::cl4(*self),
            5 => cl5::cl5(*self),
            6 => cl6::cl6(*self),
            _ => cln::cln(n, *self)
        }
    }
}


impl Cl<Complex<f64>> for Complex<f64> {
    fn cl(&self, n: i32) -> Complex<f64> {
        let eiz = (Complex::<f64>::i()*self).exp();
        let inv_eiz = 1.0/eiz;

        if is_even(n) {
            0.5*Complex::<f64>::i()*(inv_eiz.li(n) - eiz.li(n))
        } else {
            0.5*(inv_eiz.li(n) + eiz.li(n))
        }
    }
}


fn is_even(n: i32) -> bool {
    n % 2 == 0
}
