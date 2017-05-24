
use std::ops::{Add, Sub, Mul, Div};
use units::ConvertibleUnit;
use units::UnitValue;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LU{
    mm,
    cm,
    dm,
    m,
    km,
    lsec,
    lmin,
    lhour,
    lday,
    lyear,
}

impl ConvertibleUnit for LU{
    fn up(&self) -> (f64, LU) {
        match *self {
            LU::mm    => (0.1, LU::cm),
            LU::cm    => (0.1, LU::dm),
            LU::dm    => (0.1, LU::m),
            LU::m     => (0.001, LU::km),
            LU::km    => (3.33564095198e-6, LU::lsec),
            LU::lsec  => (1./60., LU::lmin),
            LU::lmin  => (1./60., LU::lhour),
            LU::lhour => (1./24., LU::lday),
            LU::lday  => (1./360., LU::lyear),
            LU::lyear => panic!("Light Year is the biggest L Unit"),
            _ => panic!("L unknown")
        }
    }

    fn down(&self) -> (f64, LU) {
        match *self {
            LU::mm    => panic!("mm is the smallest L Unit"),
            LU::cm    => (10., LU::mm),
            LU::dm    => (10., LU::cm),
            LU::m     => (10., LU::dm),
            LU::km    => (1000., LU::m),
            LU::lsec  => (299792.4580001367, LU::km),
            LU::lmin  => (60., LU::lsec),
            LU::lhour => (60., LU::lmin),
            LU::lday  => (24., LU::lhour),
            LU::lyear => (360., LU::lday),
            _ => panic!("L unknown")
        }
    }
}

//impl Add<LU> for f64 {
//    type Output = L;
//
//    fn  add(self, rhs: LU) -> L {
//        L{v: self, u: rhs}
//    }
//
//}

pub struct L{
    pub v: f64,
    pub u: LU
}

impl UnitValue<LU> for L {
    fn new(v: f64, u: LU) -> L {
        L{v: v, u: u}
    }
    fn unit(&self) -> &LU {
        &self.u
    }
    fn value(&self) -> f64 {
        self.v
    }
}

unit_arithmetic!(L);
unit_construction!(LU, L);
