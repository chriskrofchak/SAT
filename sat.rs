mod cnf_scan;
mod cnf_parse;
mod make_ast;
mod connective;
mod dpll;

// from cnf_scan
use cnf_scan::Token;
use cnf_scan::scan;

// from cnf_parse
use cnf_parse::ParseNode;
use cnf_parse::parse;
use cnf_parse::inorder;

// from make_ast
use make_ast::make_ast;
use make_ast::simplify_ast;

// from connective
use connective::Connective;
use connective::ast_print;

use std::io::{self, Read};
use std::env;
use std::collections::HashSet;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut argset: HashSet<String> = HashSet::new();
    for arg in args.iter() {
        argset.insert(arg.clone());
    }

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let gram_file: String = String::from("grammar.txt");
    let lr1_file: String = String::from("DFA.txt");
    
    match handle.read_to_string(&mut buffer) {
        Ok(_) => {
            let mut vec: Vec<Token> = scan(&buffer);
            let root: ParseNode = parse(&mut vec, &gram_file, &lr1_file);
            if argset.contains(&"--deriv".to_string()) {
                inorder(&root);
                println!("---");
            }
            let ast_root: Option<Connective> = make_ast(&root);
            match ast_root {
                None => { println!("Conversion to AST failed."); },
                Some(r) => {
                    if argset.contains(&"--AST".to_string()) {
                        ast_print(&r);
                        println!("---");
                    }
                    let simp_ast: Connective = simplify_ast(&r);
                    ast_print(&simp_ast);
                    let (res, map) = dpll::dpll(&simp_ast);
                    println!("---");
                    println!("{:?}", res);
                    if res == dpll::Res::SAT {
                        for (key, val) in map.iter() {
                            println!("[{}]: {:?}", key, val);
                        }
                    }
                }
            }
        },
        Err(_) => {
            println!("Error reading input.");
        }
    }
    
}