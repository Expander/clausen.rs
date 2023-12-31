pub fn sl18(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let pi10 = pi6*pi4;
    let pi12 = pi6*pi6;
    let pi14 = pi8*pi6;
    let pi16 = pi8*pi8;
    let pi18 = pi10*pi8;
    let x2 = x*x;
    43867.0*pi18/38979295480125.0 + x2*(-3617.0*pi16/651283132500.0 + x2*(pi14/218918700.0 + x2*(-691.0*pi12/459729270000.0 + x2*(pi10/3772137600.0 + x2*(-1.0/34292160000.0*pi8 + x2*(pi6/452656512000.0 + x2*(-1.0/7846046208000.0*pi4 + x2*(pi2/125536739328000.0 + (-1.0/711374856192000.0*pi + 1.0/12804747411456000.0*x)*x))))))))
}
