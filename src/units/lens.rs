
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

impl LU{
    fn greater(self) -> (f64, LU) {
        match self {
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

    fn smaller(self) -> (f64, LU) {
        match self {
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

pub struct L{
    pub v: f64,
    pub u: LU
}

impl L {
    pub fn new(v: f64, u: LU) -> L {
        L{v: v, u: u}
    }

    pub fn convert_to(self, nu: LU) -> L {
        L::_convert_to(nu, self.u, self.v) 
    }

    fn _convert_to(goal: LU, cur: LU, val: f64) -> L {
        if cur < goal {
            let (fac, next) = cur.greater();
            return L::_convert_to(goal, next, fac * val)
        }
        if cur > goal {
            let (fac, next) = cur.smaller();
            return L::_convert_to(goal, next, fac * val)
        }

        L{
            v: val,
            u: cur
        }
    }
}
