mod types;

pub use types::{CalcError, Operator, ParsedExpression, get_signs};

fn factorial(value: u64) -> Result<f64, CalcError> {
    let mut result: u64 = 1;

    for number in 1..=value {
        result = result
            .checked_mul(number)
            .ok_or(CalcError::Overflow)?;
    }

    Ok(result as f64)
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
    let mut fact = false;
    let mut exp = false;
    let mut mult_div = false;

    for x in trimmed.chars(){
        if x.is_whitespace(){
            continue;
        }

        let operator = signs.get(&x)
            .copied()
            .unwrap_or(Operator::NoOperator);

        if operator != Operator::NoOperator {
            if !temp.is_empty() {
                let value: f64 = temp.trim()
                    .parse::<f64>()
                    .map_err(|_| CalcError::InvalidNumber(temp.trim().to_string()))?;

                parts.push(value);
                temp.clear();
            }

            operators.push(operator);

            if operator == Operator::Factorial {
                fact = true;
            }
            else if operator == Operator::Exponent {
                exp = true;
            }
            else if operator == Operator::Divide ||
                operator == Operator::Modulo ||
                operator == Operator::Multiply
                {
                mult_div = true;
            }
        }
        else {

            if !x.is_numeric() && x != '.' {
                return Err(CalcError::InvalidOperator(x.to_string()));
            }

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
    
    if operators.len() != parts.len() && operators.len() + 1 != parts.len() {
        return Err(CalcError::InvalidFormat);
    }

    Ok(ParsedExpression {
        operators: operators,
        parts: parts,
        has_exponent: exp,
        has_factotial: fact,
        has_mult_or_div: mult_div
    })
    
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
                factorial(value)?
            }
        },
        Operator::Modulo => {
            if right == 0.0 {
                return Err(CalcError::DivisionByZero);
            }
            left % right
        },
        Operator::Exponent => left.powf(right),
        Operator::NoOperator => return Err(CalcError::DivisionByZero)
    };

    Ok(value)
}

pub fn evaluate_expression(input: &str) -> Result<f64, CalcError> {
    let expression = parse_expression(input)?;
    let mut operators = expression.operators;
    let mut parts = expression.parts;

    if operators[0] == Operator::Factorial {
        return calculate(parts[0], operators[0], 0.0);
    }

    let mut i = 0;
    let mut fact = !expression.has_factotial;
    let mut exp = !expression.has_exponent;
    let mut mult_div = !expression.has_mult_or_div;

    while !operators.is_empty() {
        if operators[i] == Operator::Factorial {
            parts[i] = match calculate(parts[i], operators[i], 0.0) {
                Ok(x) => x,
                Err(error ) => return Err(error)
            };

            operators.remove(i);
        }

        else if fact && operators[i]==Operator::Exponent {
            if i + 1 >= parts.len() {
                return Err(CalcError::InvalidFormat);
            }
            parts[i] = match calculate(parts[i], operators[i], parts[i+1]) {
                Ok(x) => x,
                Err(error ) => return Err(error)
            };
            operators.remove(i);
            parts.remove(i + 1);
        }

        else if fact &&
                exp &&
                (
                    operators[i] == Operator::Modulo ||
                    operators[i] == Operator::Divide ||
                    operators[i] == Operator::Multiply
                )
            {
            if i + 1 >= parts.len() {
                return Err(CalcError::InvalidFormat);
            }
            parts[i] = match calculate(parts[i], operators[i], parts[i+1]) {
                Ok(x) => x,
                Err(error ) => return Err(error)
            };
            operators.remove(i);
            parts.remove(i + 1);
        }

        else if fact &&
                exp &&
                mult_div && (
                    operators[i] == Operator::Add ||
                    operators[i] == Operator::Subtract
                ) {
            if i + 1 >= parts.len() {
                return Err(CalcError::InvalidFormat);
            }
            parts[i] = match calculate(parts[i], operators[i], parts[i+1]) {
                Ok(x) => x,
                Err(error ) => return Err(error)
            };
            operators.remove(i);
            parts.remove(i + 1);
        }

        else {
            i+= 1;
        }
        
        if i >= operators.len() {
            i = 0;

            if !fact {
                fact = true;
            }
            else if fact && !exp {
                exp = true;
            }
            else if fact && exp && !mult_div {
                mult_div = true
            }
        }

    }

    if parts.len() != 1 || !operators.is_empty() {
        return Err(CalcError::InvalidFormat);
            
    }

    let value = parts[0];

    Ok(value)

}

pub fn format_result(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
