use std::fmt;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

/// This is a wrapper for f64 that implements Eq and Hash,
/// by defining that NaN == NaN. It intentionally can't
/// be used for arithmetic, as rounding errors would be bad
/// for e.g. HashMap keys.
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct f64eq(f64);

impl f64eq {
    pub fn new(val: f64) -> Self {
        f64eq(val)
    }
}

impl fmt::Display for f64eq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::LowerExp for f64eq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerExp::fmt(&self.0, f)
    }
}

impl PartialEq for f64eq {
    fn eq(&self, other: &f64eq) -> bool {
        if self.0.is_nan() && other.0.is_nan() {
            return true;
        }
        if (self.0 == 0f64 || self.0 == -0f64) || (other.0 == 0f64 || other.0 == -0f64) {
            return true;
        }
        // If this next line panics, it means I have forgotten a use case,
        // the idea of this function if to catch all cases that prevent float
        // from being Eq.
        self.0.partial_cmp(&other.0).unwrap() == Ordering::Equal
    }
}

impl Eq for f64eq {}

impl Hash for f64eq {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        // Inspired by https://docs.rs/ordered-float/0.5.0/src/ordered_float/lib.rs.html#543
        self.0.to_bits().hash(hasher);
    }
}

impl From<f64> for f64eq {
    fn from(v: f64) -> Self {
        f64eq(v)
    }
}

// TODO: maybe this was a bad idea, I don't want arithmetic like `powi`
//impl Float for f64eq {
//    fn is_nan(self) -> bool {
//        self.0.is_nan()
//    }
//
//    fn is_infinite(self) -> bool {
//        self.0.is_infinite()
//    }
//
//    fn is_finite(self) -> bool {
//        self.0.is_finite()
//    }
//
//    fn is_normal(self) -> bool {
//        self.0.is_normal()
//    }
//
//    fn classify(self) -> FpCategory {
//        self.0.classify()
//    }
//
//    fn abs(self) -> Self {
//        self.0.abs()
//    }
//
//    fn signum(self) -> Self {
//        self.0.signum()
//    }
//
//    fn is_sign_positive(self) -> bool {
//        self.0.is_sign_positive()
//    }
//
//    fn is_sign_negative(self) -> bool {
//        self.0.is_sign_negative()
//    }
//
//    fn recip(self) -> Self {
//        f64eq(self.0.recip())
//    }
//
//    fn powi(self, n: i32) -> Self {
//        f64eq(self.0.powi(n))
//    }
//
//    fn to_degrees(self) -> Self {
//        f64eq(self.0.to_degrees())
//    }
//
//    fn to_radians(self) -> Self {
//        f64eq(self.0.to_radians())
//    }
//
//    fn max(self, other: Self) -> Self {
//        f64eq(self.0.max(other.0))
//    }
//
//    fn min(self, other: Self) -> Self {
//        f64eq(self.0.min(other.0))
//    }
//}

// Ord is not yet implemented because there has not been a need,
// but if a need arises, it would be in the spirit of this wrapper
// to implement both PartialOrd and Ord.
