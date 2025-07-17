use crate::error::NumError::DivisionByZero;
use crate::real::Real;

#[test]
pub fn simple_div_zero() {
    let zero = Real::from_u32(0);
    let one = Real::from_u32(1);
    let result = one / zero;
    
    assert!(result.is_err());
    assert!(matches!(result, Err(DivisionByZero)))
}

#[test]
pub fn advanced_div_zero() {
    let zero = Real::from_u32(0);
    let one = Real::from_u32(1);
    let two = Real::from_u32(2);
    let result = ((two - one.clone()).unwrap() - one.clone()).unwrap() / zero;

    assert!(result.is_err());
    assert!(matches!(result, Err(DivisionByZero)))
}