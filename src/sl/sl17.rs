pub fn sl17(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let pi10 = pi6*pi4;
    let pi12 = pi6*pi6;
    let pi14 = pi8*pi6;
    let pi16 = pi8*pi8;
    let x2 = x*x;
    x*(3617.0*pi16/325641566250.0 + x2*(-1.0/54729675.0*pi14 + x2*(691.0*pi12/76621545000.0 + x2*(-1.0/471517200.0*pi10 + x2*(pi8/3429216000.0 + x2*(-1.0/37721376000.0*pi6 + x2*(pi4/560431872000.0 + x2*(-1.0/7846046208000.0*pi2 + (pi/41845579776000.0 - 1.0/711374856192000.0*x)*x))))))))
}