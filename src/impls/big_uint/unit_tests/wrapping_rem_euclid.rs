use super::*;
use assert2::assert;
use num::BigUint;

#[test]
fn positive_dividend_and_positive_divisor_computes_expected_remainder() {
    // Given
    let expected_remainder = BigUint::from(1_u8);
    let dividend = BigUint::from(10_u8);
    let divisor = BigUint::from(3_u8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn positive_dividend_and_even_positive_divisor_yields_zero_remainder() {
    // Given
    let expected_remainder = BigUint::from(0_u8);
    let dividend = BigUint::from(10_u8);
    let divisor = BigUint::from(5_u8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}
