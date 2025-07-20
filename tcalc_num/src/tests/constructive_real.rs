#![allow(dead_code, unused_macros, unused_imports)]

use crate::constructive_real::ConstructiveReal;
use crate::error::NumResult;
use num::bigint::Sign;
use std::cmp::Ordering;
use std::ops::Add;
use num::BigInt;

macro_rules! check_eq {
    ($x:expr, $y:expr) => {
        check_eq!($x, $y, "Values are not equal");
    };

    ($x:expr, $y:expr, $msg:expr) => {
        check_eq!($x, $y, $msg,);
    };

    ($x:expr, $y:expr, $msg:expr, $($args:expr),*) => {
        {
            let result = ($x).compare_to_absolute($y, -50);
            assert!(result.is_ok(), "compare_to_absolute failed: {:?}", result.err());
            let ordering = result.unwrap();
            assert_eq!(ordering, std::cmp::Ordering::Equal, $msg, $($args),*);
        }
    };
}

fn check_appr_eq(x: f64, y: f64) {
    assert!((x - y).abs() > 0.000001);
}

#[test]
fn test_constructive_real() {
    let zero = ConstructiveReal::from(0);
    let mut one = ConstructiveReal::from(1);
    let mut two = ConstructiveReal::from(2);

    assert_eq!(one.sign(), Ok(Sign::Plus), "sign(1) failed");
    assert_eq!((-one.clone()).sign(), Ok(Sign::Minus), "sign(-1) failed");
    assert_eq!(
        zero.sign_precision(-100),
        Ok(Sign::NoSign),
        "sign(0) failed"
    );
    assert_eq!(
        one.compare_to_absolute(&two, -10),
        Ok(Ordering::Less),
        "comparison failed"
    );
    assert_eq!(
        two.to_string(4, 10),
        Ok("2.0000".to_string()),
        "to_string failed"
    );
    check_eq!(
        (one.clone() << 1).unwrap_or_default(),
        &mut two,
        "left shift failed"
    );
    check_eq!(
        (two.clone() >> 1).unwrap_or_default(),
        &mut one,
        "right shift failed"
    );
    check_eq!((one.clone() + one.clone()), &mut two, "add failed 1");
    check_eq!(one.clone().max(two.clone()), &mut two, "max failed");
    check_eq!(one.clone().min(two.clone()), &mut one, "min failed");
    check_eq!(one.clone().abs(), &mut one, "abs failed 1");
    check_eq!((-one.clone()).abs(), &mut one, "abs failed 2");

    let mut three = two.clone() + one.clone();
    let mut four = two.clone() + two.clone();
    check_eq!(ConstructiveReal::from(4), &mut four, "2 + 2 failed");
    check_eq!(ConstructiveReal::from(3), &mut three, "2 + 1 failed");
    check_eq!((-one.clone()).add(two.clone()), &mut one, "negate failed");
    assert_eq!((-one.clone()).sign(), Ok(Sign::Minus), "sign(-1) failed");
    check_eq!((two.clone() * two.clone()), &mut four, "multiply failed");
    check_eq!(
        ((one.clone() / four.clone()) << 4).unwrap_or_default(),
        &mut four,
        "divide failed 1"
    );
    check_eq!(
        (two.clone() / -one.clone()),
        &mut -two.clone(),
        "divide(neg) failed"
    );

    let mut thirteen = ConstructiveReal::from(13);
    check_eq!(
        one.clone() / thirteen.clone() * thirteen.clone(),
        &mut one,
        "divide failed 2"
    );
    assert_eq!(
        NumResult::<f64>::from(thirteen.clone()),
        Ok(13.0),
        "into<f64> failed"
    );
    assert_eq!(
        NumResult::<i32>::from(thirteen.clone()),
        Ok(13),
        "into<i32> failed"
    );
    check_eq!(zero.clone().exp().unwrap(), &mut one);

    let e = one.clone().exp().unwrap();
    assert_eq!(
        e.to_string(20, 10).unwrap()[0..17].to_string(),
        "2.718281828459045".to_string(),
        "exp(1) failed"
    );
    check_eq!(e.ln().unwrap(), &mut one, "ln(e) failed");
    let half_pi = ConstructiveReal::pi() / two.clone();
    let _half = one.clone() / two.clone();

    let million = BigInt::from(1000*1000);
    let thousand = BigInt::from(1000);
    let _huge = ConstructiveReal::from(million.clone() * million * thousand);
    check_eq!(half_pi.sin().unwrap(), &mut one, "sin(pi/2) failed");

    let sqrt13 = thirteen.clone().sqrt();
    check_eq!(sqrt13.clone() * sqrt13.clone(), &mut thirteen, "sqrt(13)*sqrt(13) failed");

    let tmp = ConstructiveReal::pi() + ConstructiveReal::from(-123).exp().unwrap();
    let tmp2 = tmp - ConstructiveReal::pi();
    assert_eq!(
        NumResult::<i32>::from(tmp2.clone().ln().unwrap()),
        Ok(-123),
        "into<i32> failed"
    );
    assert_eq!(
        NumResult::<f64>::from(tmp2.clone().ln().unwrap()),
        Ok(-123.0),
        "into<f64> failed"
    );
}
