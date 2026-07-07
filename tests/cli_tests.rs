use std::process::Command;


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
    assert!(stderr.contains("Division par zéro"));
}

#[test]
fn cli_fails_on_negatif_factorial() {
    let output = Command::new(env!("CARGO_BIN_EXE_calculator_cli"))
        .arg("(-4 + 3)")
        .arg("!")
        .output()
        .expect("failed to run binary");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid utf8");
    assert_eq!(stderr, "Erreur: La factorielle d'un nombre négatif n'est pas définie\n");
}

#[test]
fn cli_success_on_fatorial(){
    let output = Command::new(env!("CARGO_BIN_EXE_calculator_cli"))
        .arg("5")
        .arg("!")
        .output()
        .expect("failed to run binary");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid utf8");
    assert_eq!(stdout, "120\n");
}
