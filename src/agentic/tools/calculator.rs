use eyre::{eyre, Result};
use crate::agentic::tools::tool::Tool;

// Example implementation #1: Calculator tool
pub struct CalculatorTool;

impl Tool for CalculatorTool {
    fn name(&self) -> String {
        "calc".to_string()
    }

    fn example(&self) -> String {
        "calc add 5 3".to_string()
    }

    fn description(&self) -> String {
        "Performs basic arithmetic operations: add, subtract, multiply, divide".to_string()
    }

    fn run(&self, data: &str) -> Result<String> {

        let values: Vec<&str> = data.split(' ').collect();

        if values.len() < 3 {
            return Err(eyre!("Not enough arguments".to_string()))
        }

        let operation = &values[0];
        let num1: f64 = values[1].parse().map_err(|_| eyre!("First number is invalid".to_string()))?;
        let num2: f64 = values[2].parse().map_err(|_| eyre!("Second number is invalid".to_string()))?;

        match operation.to_string().as_str() {
            "add" => Ok(format!("{}", num1 + num2)),
            "subtract" => Ok(format!("{}", num1 - num2)),
            "multiply" => Ok(format!("{}", num1 * num2)),
            "divide" => {
                if num2 == 0.0 {
                    Err(eyre!("Cannot divide by zero".to_string()))
                } else {
                    Ok(format!("{}", num1 / num2))
                }
            }
            _ => Err(eyre!("Unknown operation: {}", operation))
        }
    }
}
