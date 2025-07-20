use num::{BigRational, One, Signed};
use crate::rational_extensions::RationalExtensions;
use crate::real::cr_property::CRProperty;
use crate::real::cr_property::CRPropertyType::SinPi;

/// Pair returned by trig normalization routines.
struct SignedProperty {
    property: CRProperty,
    negative: bool,
}

impl SignedProperty {
    /// Return a property corresponding to sin(pi*arg), normalizing arg to the correct range. Caller is
    /// responsible for ensuring that the argument is not one that leads to a rational result.
    /// The negate field of the result is set if the property corresponds to the negated argument,
    /// rather than the argument itself.
    /// Returns None if we can't normalize the argument.
    pub fn sin_pi(arg: BigRational) -> Self {
        let mut n_arg = arg.reduced_arg();
        let mut neg = false;
        if n_arg >= BigRational::new(1.into(), 2.into()) {
            n_arg -= BigRational::one();
        }
        if n_arg.is_negative() {
            n_arg = n_arg.abs();
            neg = true;
        }
        SignedProperty {
            property: CRProperty::new(SinPi, n_arg),
            negative: neg
        }
    }
}