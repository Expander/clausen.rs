pub fn sl25(x: f64) -> f64 {
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
    let pi20 = pi10*pi10;
    let pi22 = pi12*pi10;
    let pi24 = pi12*pi12;
    let x2 = x*x;
    x*(236364091.0*pi24/201919571963756521875.0 + x2*(-77683.0*pi22/40343570821929375.0 + x2*(174611.0*pi20/183759535834875000.0 + x2*(-43867.0*pi18/196455649219830000.0 + x2*(3617.0*pi16/118168811560800000.0 + x2*(-1.0/364105581840000.0*pi14 + x2*(691.0*pi12/3976032953692800000.0 + x2*(-1.0/122339475498240000.0*pi10 + x2*(pi8/3361246195507200000.0 + x2*(-1.0/114954619886346240000.0*pi6 + x2*(pi4/4598184795453849600000.0 + x2*(-1.0/155112100433309859840000.0*pi2 + (pi/1240896803466478878720000.0 - 1.0/31022420086661971968000000.0*x)*x))))))))))))
}
