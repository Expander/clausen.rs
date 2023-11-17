pub fn sl4(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let x2 = x*x;
    pi*pi*pi*pi/90.0 + x2*(-1.0/12.0*pi*pi + (pi/12.0 - 1.0/48.0*x)*x)
}
