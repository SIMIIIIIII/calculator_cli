mod types;

pub use types::{CalcError, Operator, ParsedExpression, get_signs};

fn factorial(value: u64) -> f64 {
    let mut result: u64 = 1;

    for number in 1..=value {
        result *= number;
    }

    result as f64
}

fn is_integer(value: f64) -> Option<u64> {
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
        None
    } else {
        Some(value as u64)
    }
}

pub fn parse_expression(input: &str) -> Result<ParsedExpression, CalcError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CalcError::EmptyInput);
    }

    let mut temp = String::from("");
    let signs = get_signs();
    let mut operators: Vec<Operator> = Vec::new();
    let mut parts: Vec<f64> = Vec::new();

    for x in trimmed.chars(){
        let operator = signs.get(&x).copied().unwrap_or(Operator::NoOperator);

        if operator != Operator::NoOperator {
            let value: f64 = temp.trim()
                .parse::<f64>()
                .map_err(|_| CalcError::InvalidNumber(temp.trim().to_string()))?;

            if operators.len() == 1 && operators[0] == Operator::Factorial {
                return Err(CalcError::InvalidFormat);
            }

            operators.push(operator);
            parts.push(value);
            temp.clear();
        }
        else {
            temp.push(x);
        }
    }

    if !temp.trim().is_empty() {
        parts.push(
            temp.trim()
            .parse::<f64>()
            .map_err(|_| CalcError::InvalidNumber(temp.trim().to_string()))?
        );
    }

    if operators.is_empty() || parts.is_empty(){
        return Err(CalcError::EmptyInput);
    }
    
    if operators.len() == parts.len() && operators.len() != 1 {
        return Err(CalcError::InvalidFormat);
    }
    
    if operators.len() == 1 &&  parts.len() == 1 && operators[0] != Operator::Factorial {
        return Err(CalcError::InvalidFormat);
    }
    
    if operators.len() + 1 != parts.len() && operators.len() != 1 {
        return Err(CalcError::InvalidFormat);
    }

    Ok(ParsedExpression { operators, parts })
    
}

pub fn calculate (left: f64, operator: Operator, right: f64) -> Result<f64, CalcError> {
    let value = match operator {
        Operator::Add => left + right,
        Operator::Subtract => left - right,
        Operator::Multiply => left * right,
        Operator::Divide => {
            if right == 0.0 {
                return Err(CalcError::DivisionByZero);
            }
            left / right
        }
        Operator::Factorial => {
            let value = is_integer(left).ok_or(CalcError::DecimalNumber)?;
            if value < 2 {
                1.0
            } else {
                factorial(value)
            }
        },
        Operator::Modulo => {
            if right == 0.0 {
                return Err(CalcError::DivisionByZero);
            }
            left % right
        },
        Operator::NoOperator => return Err(CalcError::DivisionByZero)
    };

    Ok(value)
}

pub fn evaluate_expression(input: &str) -> Result<f64, CalcError> {
    let expression = parse_expression(input)?;
    
    let operators = expression.operators;
    let parts = expression.parts;

    let mut value: f64 = parts[0];
    let mut operator: Operator = operators[0];

    if operator == Operator::Factorial {
        return calculate(value, operator, 0.0);
    }

    for i in 1..=parts.len() - 1 {
        let right = parts[i];
        value = match calculate(value, operator, right) {
            Ok(x) => x,
            Err(error ) => return Err(error)
        };
        if i < operators.len() {
            operator = operators[i];
        }
    }
    Ok(value)

}

pub fn format_result(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
