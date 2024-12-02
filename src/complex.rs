use num::complex::Complex;


/// returns i*z
pub fn times_i(z: &Complex<f64>) -> Complex<f64> {
    Complex::new(-z.im, z.re)
}


/// returns exp(i*z)
pub fn cis(z: &Complex<f64>) -> Complex<f64> {
    times_i(z).exp()
}
