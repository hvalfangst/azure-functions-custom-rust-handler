pub mod service {
    use serde_json::{json, Value};

    /// Adds two numbers.
    ///
    /// # Arguments
    ///
    /// * `num1` - The first number.
    /// * `num2` - The second number.
    ///
    /// # Returns
    ///
    /// The sum of `num1` and `num2`.
    pub fn add_numbers(num1: i32, num2: i32) -> f64 {
        (num1 + num2) as f64
    }

    /// Subtracts two numbers.
    ///
    /// # Arguments
    ///
    /// * `num1` - The first number.
    /// * `num2` - The second number.
    ///
    /// # Returns
    ///
    /// The result of subtracting `num2` from `num1`.
    pub fn subtract_numbers(num1: i32, num2: i32) -> f64 {
        (num1 - num2) as f64
    }

    /// Multiplies two numbers.
    ///
    /// # Arguments
    ///
    /// * `num1` - The first number.
    /// * `num2` - The second number.
    ///
    /// # Returns
    ///
    /// The product of `num1` and `num2`.
    pub fn multiply_numbers(num1: i32, num2: i32) -> f64 {
        (num1 * num2) as f64
    }

    /// Divides two numbers.
    ///
    /// # Arguments
    ///
    /// * `num1` - The numerator.
    /// * `num2` - The denominator.
    ///
    /// # Returns
    ///
    /// `Some(f64)` if division is possible, `None` if `denominator` is zero.
    pub fn divide_numbers(numerator: i32, denominator: i32) -> Option<f64> {
        if denominator != 0 {
            Some(numerator as f64 / denominator as f64)
        } else {
            None // Division by zero is undefined
        }
    }
}
