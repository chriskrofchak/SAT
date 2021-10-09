mod cnf_scan;
mod cnf_parse;
mod make_ast;

// from cnf_scan
use cnf_scan::Token;
use cnf_scan::scan;

// from cnf_parse
use cnf_parse::ParseNode;
use cnf_parse::parse;
use cnf_parse::inorder;

use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let gram_file: String = String::from("grammar.txt");
    let lr1_file: String = String::from("DFA.txt");
    
    match handle.read_to_string(&mut buffer) {
        Ok(_) => {
            println!("{}", buffer);
            let mut vec: Vec<Token> = scan(&buffer);
            let root: ParseNode = parse(&mut vec, &gram_file, &lr1_file );
            inorder(&root);
        },
        Err(_) => {
            println!("Error reading input.");
        }
    }
    
}