use crate::range_reduction::{range_reduce_even, range_reduce_odd};


/// Standard Clausen function Cl_n(x) for a real argument
pub fn cln(n: i64, x: f64) -> f64 {
    let (r, sgn) = range_reduce(n, x);

    if is_even(n) && (r == 0.0 || r == core::f64::consts::PI) {
        0.0
    } else if n < 10 {
        sgn*cln_zeta(n, r)
    } else {
        sgn*cln_series(n, r)
    }
}


/// Series expansion of Cl_n(x) in terms of the zeta function from
/// [Jiming Wu, Xiaoping Zhang, Dongjie Liu, "An efficient calculation
/// of the Clausen functions Cl_n(0)(n >= 2)", Bit Numer Math 50,
/// 193-206 (2010), https://doi.org/10.1007/s10543-009-0246-8].
fn cln_zeta(n: i64, x: f64) -> f64 {
    // first line in Eq.(2.13)
    let term1 = if x == 0.0 {
        0.0
    } else {
        let sgn = if is_even((n + 1)/2) { 1.0 } else { -1.0 };
        sgn*x.powf((n - 1) as f64)*inv_fac(n - 1)*(2.0*(0.5*x).sin()).ln()
    };

    // second line in Eq.(2.13)
    let sgn = if is_even(n/2) { 1.0 } else { -1.0 };
    let term2 = pcal(n, x) - sgn*inv_fac(n - 2)*nsum(n, x);

    term1 + term2
}


/// returns 1/n!
fn inv_fac(n: i64) -> f64 {
    [1.0, 1.0, 0.5, 1.0/6.0, 1.0/24.0, 1.0/120.0, 1.0/720.0, 1.0/5040.0, 1.0/40320.0][n as usize]
}


fn pcal(n: i64, x: f64) -> f64 {
    0.0 // @todo
}


fn nsum(n: i64, x: f64) -> f64 {
    0.0 // @todo
}


/// Series expansion of Cl_n(x)
fn cln_series(n: i64, x: f64) -> f64 {
    let kmax = (f64::EPSILON.powf(-(n as f64).recip())).ceil() as i64;

    if is_even(n) {
        let (mut si, co) = x.sin_cos();
        let mut si2 = 0.0; // sin((n-2)*x)
        let mut si1 = si;  // sin((n-1)*x)
        let mut sum = si;
        for k in 2..=kmax {
            si = 2.0*co*si1 - si2; // sin(n*x)
            si2 = si1;
            si1 = si;
            sum += si/(k as f64).powf(n as f64);
        }
        sum
    } else {
        let co = x.cos();
        let mut co2 = 1.0; // cos((n-2)*x)
        let mut co1 = co;  // cos((n-1)*x)
        let mut sum = co;
        for k in 2..=kmax {
            let con = 2.0*co*co1 - co2; // cos(n*x)
            co2 = co1;
            co1 = con;
            sum += con/(k as f64).powf(n as f64);
        }
        sum
    }
}


fn range_reduce(n: i64, x: f64) -> (f64, f64) {
    if is_even(n) {
        range_reduce_even(x)
    } else {
        (range_reduce_odd(x), 1.0)
    }
}


fn is_even(n: i64) -> bool {
    n % 2 == 0
}
