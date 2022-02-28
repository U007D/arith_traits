use assert2::assert;
use num::BigInt;
use super::*;

#[test]
fn positive_dividend_and_positive_divisor_yields_positive_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigInt::from(3_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(3_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}

#[test]
fn positive_dividend_and_even_positive_divisor_yields_positive_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigInt::from(2_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(5_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}

#[test]
fn positive_dividend_and_negative_divisor_yields_negative_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigInt::from(-3_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(-3_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}

#[test]
fn positive_dividend_and_even_negative_divisor_yields_positive_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigInt::from(-2_i8);
    let dividend = BigInt::from(10_i8);
    let divisor = BigInt::from(-5_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}

#[test]
fn negative_dividend_and_positive_divisor_yields_negative_quotient_and_rounds_away_from_zero() {
    // Given
    let expected_quotient = BigInt::from(-4_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(3_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}

#[test]
fn negative_dividend_and_even_positive_divisor_yields_positive_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigInt::from(-2_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(5_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}

#[test]
fn negative_dividend_and_negative_divisor_yields_positive_quotient_and_rounds_away_from_zero() {
    // Given
    let expected_quotient = BigInt::from(4_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(-3_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}


#[test]
fn negative_dividend_and_even_negative_divisor_yields_positive_quotient_and_rounds_toward_zero() {
    // Given
    let expected_quotient = BigInt::from(2_i8);
    let dividend = BigInt::from(-10_i8);
    let divisor = BigInt::from(-5_i8);

    // When
    let quotient = dividend.wrapping_div_euclid(divisor);

    // Then
    assert!(quotient == expected_quotient);
}
