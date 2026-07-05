use std::fmt::{self};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum CalcError {
    EmptyInput,
    InvalidFormat,
    InvalidNumber(String),
    InvalidOperator(String),
    DivisionByZero,
    NegativeNumber,
    DecimalNumber,
    Overflow,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Factorial,
    Modulo,
    Exponent,
    NoOperator
}

#[derive(Debug, PartialEq)]
pub struct ParsedExpression {
    pub operators: Vec<Operator>,
    pub parts: Vec<f64>,
    pub has_exponent: bool,
    pub has_factotial: bool,
    pub has_mult_or_div: bool
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "entree vide"),
            Self::InvalidFormat => write!(f, "format invalide: utilisez 'nombre operateur nombre'"),
            Self::InvalidNumber(value) => write!(f, "nombre invalide: '{value}'"),
            Self::InvalidOperator(op) => write!(f, "operateur invalide: '{op}' (attendu: + - * /)"),
            Self::DivisionByZero => write!(f, "division par zero"),
            Self::DecimalNumber => write!(f, "Nombre decimal invalide pour une factorielle"),
            Self::NegativeNumber => write!(f, "Nombre negatif invalode pour une factorielle"),
            Self::Overflow => write!(f, "depassement de capacite numerique"),
        }
    }
}

pub fn get_signs() -> HashMap<char, Operator> {
    return HashMap::from([
        ('+', Operator::Add),
        ('-', Operator::Subtract),
        ('/', Operator::Divide),
        ('*', Operator::Multiply),
        ('!', Operator::Factorial),
        ('%', Operator::Modulo),
        ('^', Operator::Exponent)
    ]);
}
