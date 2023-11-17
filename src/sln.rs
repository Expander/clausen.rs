/// returns Sl_n(x) using the expansion of Sl_n(x) in terms of sin(x) and cos(x)
pub fn sln(n: i64, x: f64) -> f64 {
    let kmax = (f64::EPSILON.powf(-(n as f64).recip())).ceil() as i64;

    if is_even(n) {
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
    } else {
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
    }
}


fn is_even(n: i64) -> bool {
    n % 2 == 0
}
