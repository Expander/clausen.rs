pub fn sl1(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        0.5*(core::f64::consts::PI - x)
    }
}
