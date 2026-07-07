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
    ExpressionConstruction,
    InvalidOperation,
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
            Self::EmptyInput => write!(f, "L'expression est vide"),
            Self::InvalidFormat => write!(f, "Format invalide : vérifiez la syntaxe de l'expression (ex: 5 + 3, (2*4) - 1)"),
            Self::InvalidNumber(value) => write!(f, "'{value}' n'est pas un nombre valide"),
            Self::InvalidOperator(op) => write!(f, "Opérateur inconnu : '{op}' (opérateurs supportés : + - * / % ^ !)"),
            Self::DivisionByZero => write!(f, "Division par zéro"),
            Self::DecimalNumber => write!(f, "La factorielle d'un nombre décimal n'est pas définie"),
            Self::NegativeNumber => write!(f, "La factorielle d'un nombre négatif n'est pas définie"),
            Self::Overflow => write!(f, "Dépassement de capacité : le résultat est trop grand"),
            Self::ExpressionConstruction => write!(f, "Construction d'expression invalide"),
            Self::InvalidOperation => write!(f, "Opération invalide"),
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