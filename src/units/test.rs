
use units::lens::{LU, L};
use units::UnitValue;

#[test]
fn test_unit_ordering() {
    assert!(LU::mm < LU::cm );
    assert!(LU::cm < LU::dm );
    assert!(LU::dm < LU::m );
    assert!(LU::m < LU::km );
    assert!(LU::km < LU::lsec );
    assert!(LU::lsec < LU::lmin );
    assert!(LU::lmin < LU::lhour );
    assert!(LU::lhour < LU::lday );
    assert!(LU::lday < LU::lyear );
}

#[test]
fn test_conversion() {

    let one_m = L{v: 1.0, u: LU::m};
    let thousand_mm = L{v: 1000.0, u: LU::mm};

    assert!((one_m.cvt_to(LU::mm).v - thousand_mm.v).abs() < 1e-10 );
    
    let one_m = L{v: 1.0, u: LU::m};
    assert!((one_m.cvt_to(LU::lyear).v - 1.057e-16).abs()  < 1e-16);
    
    let one_ly = L{v: 1.0, u: LU::lyear};
    assert!((one_ly.cvt_to(LU::mm).cvt_to(LU::lyear).v - 1.0).abs() < 1e-10); 

}

#[test]
fn test_arithmetic() {

    let am = L{v: 1.0, u: LU::m};
    let bm = L{v: 1.0, u: LU::m};
    assert!((am+bm).v == 2.0);
    let am = L{v: 1.0, u: LU::m};
    let bm = L{v: 1.0, u: LU::mm};
    assert!(((am+bm).v - 1.001).abs() < 1e-14);

    let am = L{v: 1.0, u: LU::m};
    let bm = L{v: 1.0, u: LU::m};
    assert!((am-bm).v == 0.0);
    let am = L{v: 1.0, u: LU::m};
    let bm = L{v: 1.0, u: LU::mm};
    assert!(((am-bm).v - 0.999).abs() < 1e-14);

}

#[test]
fn test_construction() {
    let a = 1.0 * LU::m;
    assert!(a.v == 1.0);
    let b = 23.0 * LU::lyear;
    assert!(b.u == LU::lyear);
}
