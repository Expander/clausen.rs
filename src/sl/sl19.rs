pub fn sl19(x: f64) -> f64 {
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
    x*(43867.0*pi18/38979295480125.0 + x2*(-3617.0*pi16/1953849397500.0 + x2*(pi14/1094593500.0 + x2*(-691.0*pi12/3218104890000.0 + x2*(pi10/33949238400.0 + x2*(-1.0/377213760000.0*pi8 + x2*(pi6/5884534656000.0 + x2*(-1.0/117690693120000.0*pi4 + x2*(pi2/2134124568576000.0 + (-1.0/12804747411456000.0*pi + 1.0/243290200817664000.0*x)*x)))))))))
}
