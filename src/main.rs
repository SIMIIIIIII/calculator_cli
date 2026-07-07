mod files;

use std::io::{self, Write};
use std::process::ExitCode;

use calculator_cli::{Expression};
pub use files::{open_history_file, save_history, print_history,};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut file = open_history_file();

    if !args.is_empty() {
        let expression = args.join(" ");

        return if expression.trim().eq_ignore_ascii_case("history") {
            match file.as_mut() {
                Ok(f) => {
                    if let Err(error) = print_history(f) {
                        eprintln!("Erreur historique: {error}");
                        ExitCode::FAILURE
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(_) => {
                    println!("Pas d'historique!");
                    ExitCode::SUCCESS
                }
            }
        } else {
            match Expression::from(&expression) {
                Err(e) => {
                    eprintln!("Erreur I/O: {e}");
                    ExitCode::FAILURE
                },
                Ok(parsed) => {
                    match parsed.eval() {
                        Ok(result) => {
                            println!("{}", Expression::format_result(result));

                            match file.as_mut() {
                                Ok(f) => {
                                    if save_history(expression.trim(), result, f).is_ok() {
                                    }
                                }
                                Err(_) => {},
                            }

                            ExitCode::SUCCESS
                        }
                        Err(error) => {
                            eprintln!("Erreur: {error}");
                            ExitCode::FAILURE
                        }
                    }
                }
            }
        };
    };

    
    println!("Calculatrice CLI Rust");
    println!("Format: nombre operateur nombre (ex: 12.5 * 4)");
    println!("Commandes: help, quit");
    
    let mut input = String::new();

    loop {
        print!("> ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("Erreur I/O: {error}");
            return ExitCode::FAILURE;
        }
        
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => return ExitCode::SUCCESS,
            Ok(_) => {}
            Err(error) => {
                eprintln!("Erreur de lecture: {error}");
                return ExitCode::FAILURE;
            }
        }

        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            continue;
        }
        
        if trimmed.eq_ignore_ascii_case("quit") {
            println!("Au revoir.");
            return ExitCode::SUCCESS;
        }
        
        if trimmed.eq_ignore_ascii_case("help") {
            println!("Entrez: nombre operateur nombre");
            println!("Operateurs supportes: + - * /");
            continue;
        }

        if trimmed.eq_ignore_ascii_case("history") {
            match file.as_mut() {
                Ok(f) => {
                    if let Err(error) = print_history(f) {
                        eprintln!("Erreur historique: {error}");
                    }
                }
                Err(_) => println!("Pas d'historique!"),
            }
            continue;
        }
        
        match Expression::from(&trimmed) {
            Err(e) => {
                eprintln!("Erreur I/O: {e}");
            },
            Ok(parsed) => {
                match parsed.eval() {
                    Ok(result) => {
                        println!("{}", Expression::format_result(result));
                        
                        match file.as_mut() {
                            Ok(f) => {
                                if save_history(trimmed, result, f).is_ok() {
                                }
                            }
                            Err(_) => {},
                        }
                    }
                    Err(error) => {
                        eprintln!("Erreur: {error}");
                    }
                }
            }
        }
    }
}
