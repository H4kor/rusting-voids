//! Module to handle all physical measurements and units.

#[cfg(test)]
mod test;

use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Display;

/// Used to implement conversion between different scalars.
/// E.g. convert from meter to light seconds
pub trait ConvertibleUnit: PartialEq + Eq + PartialOrd + Ord + Copy + Display {
    fn min() -> Self;
    fn max() -> Self;
    
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
pub trait UnitValue<U: ConvertibleUnit>: Clone + Copy where Self: Sized{

    /// creates measurement for given value and scalar
    fn new(v: f64, u: U) -> Self where Self: Sized;
    /// returns scalar of the measurement
    fn unit(&self) -> &U;
    /// returns value of the measurement
    fn value(&self) -> f64;
   
    /// converts the measurement to the optimal scalar
    fn cvt_to_opt(& self) -> Self {
        if self.value() > 1.0 && *self.unit() != U::max() {
            let (s, nu) = self.unit().up();
            let n = &mut Self::new(self.value() * s, nu);
            if n.value() > 1.0 {
                return n.cvt_to_opt()
            }   
        }
        if self.value() < 1.0 && *self.unit() != U::min() {
            let (s, nu) = self.unit().down();
            let n = &mut Self::new(self.value() * s, nu);
            if n.value() < 10.0 {
                return n.cvt_to_opt()
            }
        }
        *self
    }

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

        impl Mul<$x> for f64 {
            type Output = $x;
            fn mul(self, rhs: $x) -> $x {
                $x::new(self * rhs.value(), *rhs.unit())
            }
        }
    }
}

macro_rules! unit_disply {
    ($x:ident) => {
        impl fmt::Display for $x {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let cvt = self.cvt_to_opt();
                write!(f, "{:.4} {}", cvt.value(), cvt.unit())
            }
        }
    }
}

pub mod lens;
pub mod masses;


