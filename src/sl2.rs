pub fn sl2(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    1.0/6.0*pi*pi + (-0.5*pi + 0.25*x)*x
}
