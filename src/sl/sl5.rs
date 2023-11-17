pub fn sl5(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let x2 = x*x;
    x*(pi4/90.0 + x2*(-1.0/36.0*pi2 + (pi/48.0 - 1.0/240.0*x)*x))
}
