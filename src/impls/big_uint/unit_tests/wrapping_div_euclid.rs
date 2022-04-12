use super::*;
use assert2::assert;
use num::BigUint;

#[test]
fn positive_dividend_and_positive_divisor_yields_positive_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigUint::from(3_u8);
    let dividend = BigUint::from(10_u8);
    let divisor = BigUint::from(3_u8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}

#[test]
fn positive_dividend_and_even_positive_divisor_yields_positive_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigUint::from(2_u8);
    let dividend = BigUint::from(10_u8);
    let divisor = BigUint::from(5_u8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}
