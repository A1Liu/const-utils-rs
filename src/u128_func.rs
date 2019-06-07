/// Returns 1 if `n` is zero and 0 if `n` is greater than zero
///
/// ```rust
/// use const_utils::u128::is_zero;
/// assert!(is_zero(0) == 1);
/// assert!(is_zero(1) == 0);
/// assert!(is_zero(2) == 0);
/// assert!(is_zero(3) == 0);
/// assert!(is_zero(5) == 0);
/// ```
pub const fn is_zero(n: u128) -> u128 {
    (n == 0) as u128
}

/// Returns 1 if `n` is zero and 0 if `n` is greater than zero
///
/// ```rust
/// use const_utils::u128::not_zero;
/// assert!(not_zero(0) == 0);
/// assert!(not_zero(1) == 1);
/// assert!(not_zero(2) == 1);
/// assert!(not_zero(3) == 1);
/// assert!(not_zero(5) == 1);
/// ```
pub const fn not_zero(n: u128) -> u128 {
    is_zero(is_zero(n))
}

/// Returns `if_true` if cond is true, and otherwise returns `if_false`
///
/// ```rust
/// use const_utils::u128::cond;
/// assert!(cond(true, 33, 121) == 33);
/// assert!(cond(false, 33, 121) == 121);
/// ```
pub const fn cond(cond: bool, if_true: u128, if_false: u128) -> u128 {
    (cond as u128) * if_true + (!cond as u128) * if_false
}

/// Returns `dividend - divisor` if divisor isn't zero, and `core::u128::MAX`
/// otherwise.
///
/// ```rust
/// use const_utils::u128::safe_div;
/// assert!(safe_div(100, 10) == 10);
/// assert!(safe_div(100, 0) == core::u128::MAX);
/// ```
pub const fn safe_div(dividend: u128, divisor: u128) -> u128 {
    let val = dividend / (divisor + is_zero(divisor));
    cond(divisor == 0, core::u128::MAX, val)
}

///Returns the maximum of `a` and `b`
///
/// ```rust
/// use const_utils::u128::max;
/// assert!(max(100, 10) == 100);
/// assert!(max(0, 100) == 100);
/// ```
pub const fn max(a: u128, b: u128) -> u128 {
    cond(a > b, a, b)
}

///Returns the minimum of `a` and `b`
///
/// ```rust
/// use const_utils::u128::min;
/// assert!(min(100, 10) == 10);
/// assert!(min(0, 100) == 0);
/// ```
pub const fn min(a: u128, b: u128) -> u128 {
    cond(a > b, b, a)
}
