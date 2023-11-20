pub fn sl15(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let pi10 = pi6*pi4;
    let pi12 = pi6*pi6;
    let pi14 = pi8*pi6;
    let x2 = x*x;
    x*(2.0*pi14/18243225.0 + x2*(-691.0*pi12/3831077250.0 + x2*(pi10/11226600.0 + x2*(-1.0/47628000.0*pi8 + x2*(pi6/342921600.0 + x2*(-1.0/3592512000.0*pi4 + x2*(pi2/37362124800.0 + (-1.0/174356582400.0*pi + 1.0/2615348736000.0*x)*x)))))))
}
