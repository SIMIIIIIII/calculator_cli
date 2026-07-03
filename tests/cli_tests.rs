use std::process::Command;
use calculator_cli::{
    CalcError, Operator, ParsedExpression, evaluate_expression, format_result, parse_expression,
};


// Ce test vérifie le mode "commande directe": on passe l'expression en arguments.
#[test]
fn cli_returns_result_for_valid_expression_arg() {
    let output = Command::new(env!("CARGO_BIN_EXE_calculator_cli"))
        .arg("4")
        .arg("+")
        .arg("5")
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid utf8");
    assert_eq!(stdout.trim(), "9");
}

// Ce test vérifie qu'une erreur de calcul remonte bien avec un code d'échec.
#[test]
fn cli_fails_on_division_by_zero() {
    let output = Command::new(env!("CARGO_BIN_EXE_calculator_cli"))
        .arg("4")
        .arg("/")
        .arg("0")
        .output()
        .expect("failed to run binary");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid utf8");
    assert!(stderr.contains("division par zero"));
}

// Ce test vérifie que l'analyse d'une expression correcte fonctionne.
#[test]
fn parse_valid_expression() {
    let parsed = parse_expression("12 + 3").expect("parse should succeed");

    assert_eq!(
        parsed,
        ParsedExpression {
            operators: vec![Operator::Add],
            parts: vec![12.0, 3.0]
        }
    );
}

// Ici, on vérifie qu'une ligne vide est bien refusée.
#[test]
fn parse_rejects_empty_input() {
    let error = parse_expression("   ").expect_err("empty input should fail");
    assert_eq!(error, CalcError::EmptyInput);
}

// Ici, il manque un élément dans l'expression, donc le format est invalide.
#[test]
fn parse_rejects_wrong_token_count() {
    let error = parse_expression("12 +").expect_err("invalid format should fail");
    assert_eq!(error, CalcError::InvalidFormat);
}

// On vérifie qu'un opérateur inconnu est refusé.
#[test]
fn parse_rejects_invalid_operator() {
    let error = parse_expression("12 x 3").expect_err("invalid operator should fail");
    assert_eq!(error, CalcError::InvalidOperator("x".to_string()));
}

// On teste chaque opération une par une pour être sûr du résultat.
#[test]
fn evaluate_addition() {
    let value = evaluate_expression("2 + 3").expect("evaluation should succeed");
    assert_eq!(value, 5.0);
}

#[test]
fn evaluate_subtraction() {
    let value = evaluate_expression("10 - 4").expect("evaluation should succeed");
    assert_eq!(value, 6.0);
}

#[test]
fn evaluate_multiplication() {
    let value = evaluate_expression("3 * 7").expect("evaluation should succeed");
    assert_eq!(value, 21.0);
}

// La division par zéro doit retourner une erreur propre.
#[test]
fn evaluate_division_by_zero_fails() {
    let error = evaluate_expression("10 / 0").expect_err("division by zero should fail");
    assert_eq!(error, CalcError::DivisionByZero);
}

// Le formatage doit afficher un entier sans décimale inutile.
#[test]
fn format_result_hides_trailing_zero() {
    assert_eq!(format_result(8.0), "8");
    assert_eq!(format_result(8.25), "8.25");
}

#[test]
fn evaluate_factorial() {
    let value = evaluate_expression("5.0 !").expect("evaluation should succeed");
    assert_eq!(value, 120.0);
}

#[test]
fn evaluate_factorial_decimal_fail() {
    let error = evaluate_expression("5.6 !").expect_err("factoriel of decimal number fails");
    assert_eq!(error, CalcError::DecimalNumber);
}


