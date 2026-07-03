use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if !args.is_empty() {
        let expression = args.join(" ");
        return match calculator_cli::evaluate_expression(&expression) {
            Ok(result) => {
                println!("{}", calculator_cli::format_result(result));
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("Erreur: {error}");
                ExitCode::FAILURE
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
        
        match calculator_cli::evaluate_expression(trimmed) {
            Ok(result) => println!("= {}", calculator_cli::format_result(result)),
            Err(error) => eprintln!("Erreur: {error}"),
        }
    }
}
