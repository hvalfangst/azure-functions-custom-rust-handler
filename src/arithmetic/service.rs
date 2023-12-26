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
    pub fn add_numbers(num1: i32, num2: i32) -> i32 {
        num1 + num2
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
    pub fn subtract_numbers(num1: i32, num2: i32) -> i32 {
        num1 - num2
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
    pub fn multiply_numbers(num1: i32, num2: i32) -> i32 {
        num1 * num2
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

    /// Calculates the average of a list of numbers.
    ///
    /// # Arguments
    ///
    /// * `numbers` - A slice containing the list of numbers.
    ///
    /// # Returns
    ///
    /// `Some(f64)` if the list is not empty, `None` if the list is empty.
    pub fn get_average(numbers: &[i32]) -> Option<f64> {
        let sum: i32 = numbers.iter().sum();
        let count = numbers.len() as i32;

        if count > 0 {
            Some(sum as f64 / count as f64)
        } else {
            None // Cannot calculate average for an empty list
        }
    }

    /// Returns metadata about available routes.
    ///
    /// # Returns
    ///
    /// A vector of JSON objects representing metadata for each route.
    pub fn routes_metadata() -> Vec<Value> {
        let routes = vec![
            json!({
            "route": "/arithmetic/add/:num1/:num2",
            "method": "GET",
            "description": "Add two numbers",
            "params_type": "query",
            "params": [
                {"name": "num1", "type": "integer", "required": true},
                {"name": "num2", "type": "integer", "required": true}
            ]
        }),
            json!({
            "route": "/arithmetic/subtract/:num1/:num2",
            "method": "GET",
            "description": "Subtract two numbers",
            "params_type": "query",
            "params": [
                {"name": "num1", "type": "integer", "required": true},
                {"name": "num2", "type": "integer", "required": true}
            ]
        }),
            json!({
            "route": "/arithmetic/multiply/:num1/:num2",
            "method": "GET",
            "description": "Multiply two numbers",
            "params_type": "query",
            "params": [
                {"name": "num1", "type": "integer", "required": true},
                {"name": "num2", "type": "integer", "required": true}
            ]
        }),
            json!({
            "route": "/arithmetic/divide/:num1/:num2",
            "method": "GET",
            "description": "Divide two numbers",
            "params_type": "query",
            "params": [
                {"name": "num1", "type": "integer", "required": true},
                {"name": "num2", "type": "integer", "required": true}
            ]
        }),
            json!({
            "route": "/arithmetic/average",
            "method": "POST",
            "description": "Calculate the average of a list of numbers",
            "params_type": "request_body",
            "params": [
                {"name": "numbers", "type": "array", "item_type": "integer", "required": true}
            ]
        }),
        ];
        routes
    }
}
