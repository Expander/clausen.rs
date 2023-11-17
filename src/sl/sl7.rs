pub fn sl7(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let x2 = x*x;
    x*(pi6/945.0 + x2*(-1.0/540.0*pi4 + x2*(pi2/720.0 + (-1.0/1440.0*pi + 1.0/10080.0*x)*x)))
}
