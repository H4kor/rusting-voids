//! Module to handle all physical measurements and units.

#[cfg(test)]
mod test;

use std::ops::{Add, Sub, Mul, Div};

/// Used to implement conversion between different scalars.
/// E.g. convert from meter to light seconds
pub trait ConvertibleUnit: PartialEq + Eq + PartialOrd + Ord + Copy {
    /// returns factor and next bigger scalar
    fn up(&self) -> (f64, Self);
    /// returns factor and next smaller scalar
    fn down(&self) -> (f64, Self);
}

/// Allows construction of measurements via multiplaction.
/// E.g `let a = 1.0 * LU::m;` for 1 meter
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

/// Basis implementation of measurements.
/// Allows conversion between scalars.
pub trait UnitValue<U: ConvertibleUnit> where Self: Sized{

    /// creates measurement for given value and scalar
    fn new(v: f64, u: U) -> Self where Self: Sized;
    /// returns scalar of the measurement
    fn unit(&self) -> &U;
    /// returns value of the measurement
    fn value(&self) -> f64;
    
    /// converts measurement to the given scalar
    fn cvt_to(self, nu: U) -> Self where Self: Sized {
        UnitValue::_cvt_to(&nu, self.unit(), self.value())
    }
    
    /// internal function to realize ``cvt_to``
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

/// Basic arithmetic for measurements.
/// Implements addition and substraction.
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


