pub fn sl1(x: f64) -> f64 {
    if x == 0.0 {
        x
    } else {
        0.5*(core::f64::consts::PI - x)
    }
}
