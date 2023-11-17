pub fn sl3(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    x*(1.0/6.0*pi*pi + (-0.25*pi + 1.0/12.0*x)*x)
}
