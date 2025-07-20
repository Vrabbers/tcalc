use std::clone::Clone;
use std::sync::LazyLock;
use crate::constructive_real::{ConstructiveReal, ConstructiveRealKnownValue};

pub static ONE: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(1).with_known_value(ConstructiveRealKnownValue::One));
pub static PI: LazyLock<ConstructiveReal> = LazyLock::new(ConstructiveReal::pi);
pub static E: LazyLock<ConstructiveReal> = LazyLock::new(|| ONE.clone().exp().unwrap().with_known_value(ConstructiveRealKnownValue::E));
pub static SQRT_2: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(2).sqrt().with_known_value(ConstructiveRealKnownValue::Sqrt2));
pub static SQRT_3: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(3).sqrt().with_known_value(ConstructiveRealKnownValue::Sqrt3));
pub static LN_2: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(2).ln().unwrap());
pub static LN_3: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(3).ln().unwrap());
pub static LN_5: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(5).ln().unwrap());
pub static LN_6: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(6).ln().unwrap());
pub static LN_7: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(7).ln().unwrap());
pub static LN_10: LazyLock<ConstructiveReal> = LazyLock::new(|| ConstructiveReal::from(10).ln().unwrap().with_known_value(ConstructiveRealKnownValue::Ln10));

pub static CR_SQUARE_ROOTS: LazyLock<[Option<ConstructiveReal>; 11]> = LazyLock::new(|| [
    None,
    Some(ONE.clone()),
    Some(SQRT_2.clone()),
    Some(SQRT_3.clone()),
    None,
    Some(ConstructiveReal::from(5).sqrt()),
    Some(ConstructiveReal::from(6).sqrt()),
    Some(ConstructiveReal::from(7).sqrt()),
    None,
    None,
    Some(ConstructiveReal::from(10).sqrt())
]);

pub static CR_LNS: LazyLock<[Option<ConstructiveReal>; 11]> = LazyLock::new(|| [
    None,
    None,
    Some(LN_2.clone()),
    Some(LN_3.clone()),
    None,
    Some(LN_5.clone()),
    Some(LN_6.clone()),
    Some(LN_7.clone()),
    None,
    None,
    Some(LN_10.clone())
]);