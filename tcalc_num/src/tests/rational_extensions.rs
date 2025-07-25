#![allow(dead_code, unused_macros, unused_imports)]

use std::str::FromStr;
use crate::rational_extensions::RationalExtensions;
use num::{BigInt, BigRational, FromPrimitive, One};

#[test]
pub fn test_rational_creation() {
    let tests = [
        ("1", "1", "1"),
        ("1.0", "1", "1"),
        ("-1", "-1", "1"),
        ("3", "3", "1"),
        ("3.5", "7", "2"),
        ("6.00000000000000000", "6", "1"),
        ("6.00000000000000001", "600000000000000001", "100000000000000000")
    ];

    for (input, numerator, denominator) in tests {
        let expected = BigRational::new(
            BigInt::from_str(numerator).unwrap(),
            BigInt::from_str(denominator).unwrap(),
        );
        let actual = BigRational::from_decimal_string(input);
        assert!(
            actual.is_some(),
            "Failed to create rational from string: {input}"
        );
        assert_eq!(
            expected,
            actual.unwrap(),
            "Failed to create rational from string: {input}"
        );
    }
}

#[test]
pub fn test_rational_creation_fail() {
    let tests = ["", "1.0.0", "1.0.0", "1.0.0"];
    for input in tests {
        let actual = BigRational::from_decimal_string(input);
        assert!(actual.is_none(), "Failed to fail to create rational from string: {input}");
    }
}