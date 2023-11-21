/// general Clausen function S_n(x) defined as a series of sin(x) for
/// n > 0
pub fn si(n: i32, x: f64) -> f64 {
    let kmax = max_order(n);

    let (mut si, co) = x.sin_cos();
    let mut si2 = 0.0; // sin((n-2)*x)
    let mut si1 = si;  // sin((n-1)*x)
    let mut sum = si;
    for k in 2..=kmax {
        si = 2.0*co*si1 - si2; // sin(n*x)
        si2 = si1;
        si1 = si;
        sum += si/(k as f64).powi(n);
    }
    sum
}


/// general Clausen function C_n(x) defined in as a series of cos(x)
/// for n > 0
pub fn co(n: i32, x: f64) -> f64 {
    let kmax = max_order(n);

    let co = x.cos();
    let mut co2 = 1.0; // cos((n-2)*x)
    let mut co1 = co;  // cos((n-1)*x)
    let mut sum = co;
    for k in 2..=kmax {
        let con = 2.0*co*co1 - co2; // cos(n*x)
        co2 = co1;
        co1 = con;
        sum += con/(k as f64).powi(n);
    }
    sum
}


/// maximum order for the summation of the general Clausen functions
fn max_order(n: i32) -> i32 {
    (f64::EPSILON.powf(-(n as f64).recip())).ceil() as i32
}
