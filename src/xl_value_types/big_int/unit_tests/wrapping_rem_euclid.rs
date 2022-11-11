use super::*;
use assert2::assert;
use num::BigInt;

#[test]
fn positive_dividend_and_positive_divisor_computes_expected_remainder() {
    // Given
    let expected_remainder = BigInt::from(1_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(3_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn positive_dividend_and_even_positive_divisor_yields_zero_remainder() {
    // Given
    let expected_remainder = BigInt::from(0_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(5_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn positive_dividend_and_negative_divisor_computes_expected_remainder() {
    // Given
    let expected_remainder = BigInt::from(1_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(-3_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn positive_dividend_and_even_negative_divisor_yields_zero_remainder() {
    // Given
    let expected_remainder = BigInt::from(0_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(-5_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn negative_dividend_and_positive_divisor_computes_expected_remainder() {
    // Given
    let expected_remainder = BigInt::from(2_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(3_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn negative_dividend_and_even_positive_divisor_yields_zero_remainder() {
    // Given
    let expected_remainder = BigInt::from(0_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(5_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn negative_dividend_and_negative_divisor_computes_expected_remainder() {
    // Given
    let expected_remainder = BigInt::from(2_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(-3_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}

#[test]
fn negative_dividend_and_even_negative_divisor_yields_zero_remainder() {
    // Given
    let expected_remainder = BigInt::from(0_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(-5_i8);

    // When
    let remainder = dividend.wrapping_rem_euclid(divisor);

    // Then
    assert!(remainder == expected_remainder);
}
