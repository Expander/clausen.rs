pub fn sl8(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let x2 = x*x;
    pi8/9450.0 + x2*(-1.0/1890.0*pi6 + x2*(pi4/2160.0 + x2*(-1.0/4320.0*pi2 + (pi/10080.0 - 1.0/80640.0*x)*x)))
}
