use crate::real::Real;

#[test]
fn one_plus_one() {
    let one_1 = Real::from_u32(1);
    let one_2 = Real::from_u32(1);
    let result = (one_1 + one_2).unwrap();
    println!("{:?}", result.evaluate_to_string_with_dp(10))
}