pub fn sl10(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let pi10 = pi6*pi4;
    let x2 = x*x;
    pi10/93555.0 + x2*(-1.0/18900.0*pi8 + x2*(pi6/22680.0 + x2*(-1.0/64800.0*pi4 + x2*(pi2/241920.0 + (-1.0/725760.0*pi + 1.0/7257600.0*x)*x))))
}
