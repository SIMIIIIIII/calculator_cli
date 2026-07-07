use crate::{
    CalcError::{ExpressionConstruction, InvalidFormat},
    Operator::{Factorial, NoOperator},
    get_signs,
    types::{CalcError, Operator}
};


#[derive(Debug, Clone)]
pub struct Expression {
    pub leaf: bool,
    pub operator: Operator,
    pub value: Option<f64>,
    pub left: Option<Box<Expression>>,
    pub right: Option<Box<Expression>>, 
}

impl Expression {
    pub fn new(
        leaf: bool,
        operator: Operator,
        value: Option<f64>,
        left: Option<Box<Expression>>,
        right: Option<Box<Expression>>) -> Result<Self, CalcError>
        {

            if leaf && (value.is_none() || operator != Operator::NoOperator || left.is_some() || right.is_some()) {
                return Err(ExpressionConstruction);
            }
            
            if !leaf && (value.is_some() || operator == Operator::NoOperator || left.is_none()) {
                return Err(ExpressionConstruction);
            }

            Ok(Expression {
                leaf,
                operator,
                value,
                left,
                right
            })
        }

    fn parse_leaf(temp: &str) -> Result<Self, CalcError> {
        let value = temp.trim()
            .parse::<f64>()
            .map_err(|_| CalcError::InvalidNumber(temp.trim().to_string()))?;
        Expression::new(true, NoOperator, Some(value), None, None)
    }

    fn from_bis(entry: &Vec<char>, j: usize, is_sub: bool) -> Result<(Self, usize), CalcError> {
        let mut temp = String::from("");
        let signs_table = get_signs();
        let mut i: usize = j;
        let mut expression: Option<Self> = None;

        let set_child = |expression: &mut Option<Self>, child: Self| {
            if expression.is_none() {
                *expression = Some(child);
            } else if let Some(expr) = expression {
                expr.right = Some(Box::new(child));
            }
        };

        if entry[i] == '-' || entry[i] == '+' {
            temp.push(entry[i]);
            i += 1;
        }

        let n = entry.len();

        while i <= n {
            if i < n && entry[i].is_whitespace() {
                i += 1;
                continue;
            }

            let operator = if i < n {
                signs_table.get(&entry[i]).copied().unwrap_or(NoOperator)
            } else {
                NoOperator
            };

            if operator != NoOperator {
                if !temp.trim().is_empty() {
                    let leaf = Self::parse_leaf(&temp)?;
                    set_child(&mut expression, leaf);
                    temp.clear();
                }

                if i < n || (!temp.is_empty() && operator == Factorial) {
                    expression = Some(Expression::new(
                        false,
                        operator,
                        None,
                        Some(Box::new(expression.unwrap())),
                        None
                    )?);
                }
            } else if i == n {
                if !temp.trim().is_empty() {
                    let leaf = Self::parse_leaf(&temp)?;
                    set_child(&mut expression, leaf);
                }
                break;
            } else if entry[i] == '(' {
                if !temp.is_empty() {
                    return Err(InvalidFormat);
                }
                let (sub, end) = Expression::from_bis(entry, i + 1, true)?;
                i = end;
                set_child(&mut expression, sub);
            } else if entry[i] == ')' {
                if !is_sub {
                    return Err(InvalidFormat);
                }
                if !temp.trim().is_empty() {
                    let leaf = Self::parse_leaf(&temp)?;
                    set_child(&mut expression, leaf);
                }
                return Ok((expression.unwrap(), i));
            } else if entry[i] != '.' && !entry[i].is_numeric() {
                return Err(CalcError::InvalidOperator(entry[i].to_string()));
            } else {
                temp.push(entry[i]);
            }

            i += 1;
        }

        if expression.is_none() {
            return Err(InvalidFormat);
        }
        if is_sub {
            return Err(InvalidFormat);
        }
        Ok((expression.unwrap(), i))
    }

    pub fn from(input: &str) -> Result<Self, CalcError> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(CalcError::EmptyInput);
        }
        
        let entry: Vec<char> = trimmed.chars().collect();

        let result = Expression::from_bis(&entry, 0, false);
        
        if result.is_err() {
            return Err(result.unwrap_err());
        }
        Ok(result.unwrap().0)
    }

    fn is_integer(value: f64) -> Result<u64, CalcError> {
        if !value.is_finite() || value.fract() != 0.0 {
            Err(CalcError::DecimalNumber)
        } else if value < 0.0 {
            Err(CalcError::NegativeNumber)
        } else {
            Ok(value as u64)
        }
    }

    fn factorial(value: u64) -> Result<u64, CalcError> {
        if value > 1 {
            let result: u64 = match Self::factorial(value-1) {
                Ok(x) => x,
                Err(e) => return Err(e)
            };
            
            result.checked_mul(value).ok_or(CalcError::Overflow)
        } else {
            Ok(1)
        }
    }

    fn calculate (left: f64, operator: Operator, right: f64) -> Result<f64, CalcError> {
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
                let value = Self::is_integer(left)?;
                if value < 2 {
                    1.0
                } else {
                    Self::factorial(value)? as f64
                }
            },
            Operator::Modulo => {
                if right == 0.0 {
                    return Err(CalcError::DivisionByZero);
                }
                left % right
            },
            Operator::Exponent => left.powf(right),
            Operator::NoOperator => return Err(CalcError::InvalidOperation)
        };

        Ok(value)
    }

    pub fn eval(&self) -> Result<f64, CalcError> {
        if self.leaf {
            return self.value.ok_or(CalcError::InvalidFormat);
        }

        let left = self.left.as_ref().ok_or(CalcError::InvalidFormat)?;
        let right: Option<&Box<Expression>> = if self.operator != Factorial {
            Some(self.right.as_ref().ok_or(CalcError::InvalidFormat)?)
        } else {None};

        let first = left.eval()?;
        let second = if right.is_some(){
             right.unwrap().eval()?
        } else {0.0};

        Self::calculate(first, self.operator, second)
    }

    pub fn format_result(value: f64) -> String {
        if value.fract() == 0.0 {
            format!("{value:.0}")
        } else {
            value.to_string()
        }
    }
}