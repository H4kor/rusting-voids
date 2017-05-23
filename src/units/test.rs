
use units::lens::{LU, L};

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

    assert!((one_m.convert_to(LU::mm).v - thousand_mm.v).abs() < 1e-10 );
    
    let one_m = L{v: 1.0, u: LU::m};
    assert!((one_m.convert_to(LU::lyear).v - 1.057e-16).abs()  < 1e-16);
    
    let one_ly = L{v: 1.0, u: LU::lyear};
    assert!((one_ly.convert_to(LU::mm).convert_to(LU::lyear).v - 1.0).abs() < 1e-10); 

}
