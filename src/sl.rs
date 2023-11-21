mod sl0;
mod sl1;
mod sl2;
mod sl3;
mod sl4;
mod sl5;
mod sl6;
mod sl7;
mod sl8;
mod sl9;
mod sl10;
mod sl11;
mod sl12;
mod sl13;
mod sl14;
mod sl15;
mod sl16;
mod sl17;
mod sl18;
mod sl19;
mod sl20;
mod sl21;
mod sl22;
mod sl23;
mod sl24;
mod sl25;
mod sln;

use crate::range_reduction::{range_reduce_even, range_reduce_odd};
use num::complex::Complex;
use polylog::Li;


pub trait Sl<T> {
    /// Returns the n-th order Glaisher-Clausen function.
    ///
    /// # Example:
    /// ```
    /// use clausen::Sl;
    /// use num::complex::Complex;
    ///
    /// assert!((1.0.sl(2) - 0.32413774005332982).abs() < 1e-15);
    /// assert!((Complex::new(1.0, 1.0).sl(2) - Complex::new(0.07413774005332982, -1.07079632679489662)).norm() < 1e-15);
    /// ```
    fn sl(&self, n: i32) -> T;
}


impl Sl<f64> for f64 {
    fn sl(&self, n: i32) -> f64 {
        let (r, sgn) = range_reduce(n, *self);

        match n {
            k if k < 0 => 0.0,
            0 => sgn*sl0::sl0(),
            1 => sgn*sl1::sl1(r),
            2 => sgn*sl2::sl2(r),
            3 => sgn*sl3::sl3(r),
            4 => sgn*sl4::sl4(r),
            5 => sgn*sl5::sl5(r),
            6 => sgn*sl6::sl6(r),
            7 => sgn*sl7::sl7(r),
            8 => sgn*sl8::sl8(r),
            9 => sgn*sl9::sl9(r),
            10 => sgn*sl10::sl10(r),
            11 => sgn*sl11::sl11(r),
            12 => sgn*sl12::sl12(r),
            13 => sgn*sl13::sl13(r),
            14 => sgn*sl14::sl14(r),
            15 => sgn*sl15::sl15(r),
            16 => sgn*sl16::sl16(r),
            17 => sgn*sl17::sl17(r),
            18 => sgn*sl18::sl18(r),
            19 => sgn*sl19::sl19(r),
            20 => sgn*sl20::sl20(r),
            21 => sgn*sl21::sl21(r),
            22 => sgn*sl22::sl22(r),
            23 => sgn*sl23::sl23(r),
            24 => sgn*sl24::sl24(r),
            25 => sgn*sl25::sl25(r),
            _ => sgn*sln::sln(n, r)
        }
    }
}


impl Sl<Complex<f64>> for Complex<f64> {
    fn sl(&self, n: i32) -> Complex<f64> {
        if n < 0 {
            Complex::new(0.0, 0.0)
        } else if n == 0 {
            Complex::new(-0.5, 0.0)
        } else if n == 1 && self.re == 0.0 && self.im == 0.0 {
            Complex::new(0.0, 0.0)
        } else {
            let eiz = (Complex::<f64>::i()*self).exp();
            let inv_eiz = 1.0/eiz;

            if is_even(n) {
                0.5*(inv_eiz.li(n) + eiz.li(n))
            } else {
                0.5*Complex::<f64>::i()*(inv_eiz.li(n) - eiz.li(n))
            }
        }
    }
}


/// map x to [0,pi] using the symmetries of Sl_n(x)
fn range_reduce(n: i32, x: f64) -> (f64, f64) {
    if is_even(n) {
        (range_reduce_odd(x), 1.0)
    } else {
        range_reduce_even(x)
    }
}


fn is_even(n: i32) -> bool {
    n % 2 == 0
}
