use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

use derive_new::new;

/// This is a wrapper for f64 that implements Eq and Hash,
/// by defining that NAN == NAN. It intentionally can't
/// be used for arithmetic, as rounding errors would be bad
/// for e.g. HashMap keys.
#[derive(new, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct f64eq(f64);

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
        if self.0.is_nan() {
            return other.0.is_nan();
        } else if other.0.is_nan() {
            return false;
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
        if self.0 == -0. {
            (0f64).to_bits().hash(hasher);
            return;
        }
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
//    fn is_NAN(self) -> bool {
//        self.0.is_NAN()
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

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;
    use std::f64::consts::PI;
    use std::f64::{INFINITY, NAN, NEG_INFINITY};
    use std::hash::{BuildHasher, Hash, Hasher};

    use lazy_static::lazy_static;

    use super::f64eq;

    #[test]
    fn test_eq() {
        assert_eq!(f64eq::new(42.), f64eq::new(42.));
        assert_eq!(f64eq::new(PI), f64eq::new(PI));
        assert_ne!(f64eq::new(42.), f64eq::new(-42.));
        assert_eq!(f64eq::new(0.), f64eq::new(-0.));
        assert_eq!(f64eq::new(INFINITY), f64eq::new(INFINITY));
        assert_ne!(f64eq::new(INFINITY), f64eq::new(NEG_INFINITY));
        assert_ne!(f64eq::new(42.), f64eq::new(NAN));
        assert_ne!(f64eq::new(NAN), f64eq::new(42.));
        assert_eq!(f64eq::new(NAN), f64eq::new(NAN));
    }

    lazy_static! {
        static ref RANDOM: RandomState = RandomState::new();
    }

    fn get_hash(x: f64eq) -> u64 {
        let mut hasher = RANDOM.build_hasher();
        x.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn test_hash() {
        // While different values are not 100% guaranteed to have different hashes,
        // it would be extremely suspicious of there is a collision in these few.
        assert_eq!(get_hash(f64eq::new(42.)), get_hash(f64eq::new(42.)));
        assert_eq!(get_hash(f64eq::new(PI)), get_hash(f64eq::new(PI)));
        assert_ne!(get_hash(f64eq::new(42.)), get_hash(f64eq::new(-42.)));
        assert_eq!(get_hash(f64eq::new(0.)), get_hash(f64eq::new(-0.)));
        assert_eq!(get_hash(f64eq::new(INFINITY)), get_hash(f64eq::new(INFINITY)));
        assert_ne!(get_hash(f64eq::new(INFINITY)), get_hash(f64eq::new(NEG_INFINITY)));
        assert_ne!(get_hash(f64eq::new(42.)), get_hash(f64eq::new(NAN)));
        assert_ne!(get_hash(f64eq::new(NAN)), get_hash(f64eq::new(42.)));
        assert_eq!(get_hash(f64eq::new(NAN)), get_hash(f64eq::new(NAN)));
    }
}
