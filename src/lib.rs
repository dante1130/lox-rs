mod ast;
mod error;
mod lexer;
mod value;

use std::io::Write;

use ast::{
    ast_printer::AstPrinter,
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
};
use lexer::{scanner::Scanner, token::Token, token_type::TokenType};

use crate::value::Value;

pub fn run(args: Vec<String>) {
    // if args.len() > 2 {
    //     println!("Usage: rslox [script]");
    //     std::process::exit(64);
    // }
    //
    // if args.len() == 2 {
    //     run_file(args[0].clone());
    // } else {
    //     run_prompt();
    // }

    let expression = Expr::Binary(BinaryExpr::new(
        Token::new(TokenType::Star, String::from("*"), None, 1),
        Expr::Unary(UnaryExpr::new(
            Token::new(TokenType::Minus, String::from("-"), None, 1),
            Expr::Literal(LiteralExpr::new(Value::Number(123.0))),
        )),
        Expr::Grouping(GroupingExpr::new(Expr::Literal(LiteralExpr::new(
            Value::Number(45.67),
        )))),
    ));

    println!("{}", AstPrinter {}.print(&expression));
}

fn run_file(path: String) {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            println!("Failed to read file: {}", err);
            std::process::exit(1);
        }
    };

    run_source(source);

    if error::had_error() {
        std::process::exit(65);
    }
}

fn run_prompt() {
    loop {
        print!("> ");

        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to flush stdout: {}", err);
                std::process::exit(1);
            }
        }

        let mut line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to read line: {}", err);
                std::process::exit(1);
            }
        }

        run_source(line);

        error::reset_error();
    }
}

fn run_source(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}
