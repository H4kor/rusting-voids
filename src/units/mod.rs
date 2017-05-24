#[cfg(test)]
mod test;

use std::ops::{Add, Sub, Mul, Div};

trait ConvertibleUnit: PartialEq + Eq + PartialOrd + Ord + Copy {
    fn up(&self) -> (f64, Self);
    fn down(&self) -> (f64, Self);
}

#[macro_export]
macro_rules! unit_construction {
    ($u:ident, $v:ident) => {
        impl Mul<$u> for f64 {
            type Output = $v;
            fn mul(self, rhs: $u) -> $v {
                $v::new(self, rhs)
            }
        }
    }
}

trait UnitValue<U: ConvertibleUnit> where Self: Sized{

    fn new(v: f64, u: U) -> Self where Self: Sized;
    fn unit(&self) -> &U;
    fn value(&self) -> f64;
    
    fn cvt_to(self, nu: U) -> Self where Self: Sized {
        UnitValue::_cvt_to(&nu, self.unit(), self.value())
    }

    fn  _cvt_to(goal: &U, cur: &U, val: f64) -> Self where Self: Sized {
        if cur < goal {
            let (fac, next) = cur.up();
            return UnitValue::_cvt_to(goal, &next, fac * val)
        } 
        if cur > goal {
            let (fac, next) = cur.down();
            return UnitValue::_cvt_to(goal, &next, fac * val)
        }
        Self::new(val, *cur)
    }
}

#[macro_export]
macro_rules! unit_arithmetic {
    ($x:ident) => {    
        impl Add for $x {
            type Output = $x;
            fn add(self, rhs: $x) -> $x {
                if self.unit() < rhs.unit() {
                    // convert to rhs unit
                    let conv = self.cvt_to(*rhs.unit());
                    $x::new(rhs.value() + conv.value(), *conv.unit())
                } else {
                    // convert to self unit
                    let conv = rhs.cvt_to(*self.unit());
                    $x::new(self.value() + conv.value(), *conv.unit())
                }
            }
        }

        impl Sub for $x {
            type Output = $x;
            fn sub(self, rhs: $x) -> $x {
                if self.unit() < rhs.unit() {
                    // convert to rhs unit
                    let conv = self.cvt_to(*rhs.unit());
                    $x::new(rhs.value() - conv.value(), *conv.unit())
                } else {
                    // convert to self unit
                    let conv = rhs.cvt_to(*self.unit());
                    $x::new(self.value() - conv.value(), *conv.unit())
                }
            }
        }
    }
}

pub mod lens;
pub mod masses;


