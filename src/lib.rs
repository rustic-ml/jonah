/// Adds two numbers and returns the sum
///
/// # Examples
///
/// ```
/// use rust_crate::add;
///
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Subtracts the second number from the first and returns the difference
///
/// # Examples
///
/// ```
/// use rust_crate::subtract;
///
/// let result = subtract(5, 2);
/// assert_eq!(result, 3);
/// ```
pub fn subtract(left: u64, right: u64) -> u64 {
    left - right
}

/// Multiplies two numbers and returns the product
///
/// # Examples
///
/// ```
/// use rust_crate::multiply;
///
/// let result = multiply(2, 3);
/// assert_eq!(result, 6);
/// ```
pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtract_works() {
        let result = subtract(5, 2);
        assert_eq!(result, 3);
    }

    #[test]
    fn multiply_works() {
        let result = multiply(3, 4);
        assert_eq!(result, 12);
    }
}
