use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt;

use crate::cnf_scan::Token;
use crate::cnf_scan::Type;

use ParseVal::*;

///////////////////// Structs and such
/// 

// bool is TRUE for reduce.
// if bool then reduce by rule usize, else shift to state usize
#[derive(Debug, Clone)]
struct ReducePair(bool, usize);

// Rule is non leaf node which can have many children
// Term is a terminal. 
#[derive(Debug, Clone)]
pub enum ParseVal {
    Rule(Vec<String>),
    Term(Token)
}

// node can have many children! 
#[derive(Clone)]
pub struct ParseNode(pub ParseVal, pub Vec<ParseNode>);

pub fn get_token(node: &ParseNode) -> String {
    match &node.0 {
        Rule(vec) => vec[0].clone(),
        Term(tok) => tok.0.clone()
    }
}

impl fmt::Display for ParseVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Rule(vec) => {
                let mut vec_str: String = String::new();
                for item in vec.iter() {
                    vec_str.push_str(&*item.to_string());
                    vec_str.push_str(" ");
                }
                write!(f, "{}", vec_str)
            },
            Term(tok) => write!(f, "{} {}", tok.0.clone(), tok.1.to_string())
        }
    }
}


// will have to implement tree traversal on ParseNode

fn inorder_helper(node: &ParseNode, s: i16) {
    for _ in 0..s {
        print!(" ");
    }
    println!("{}", node.0);
    for child in node.1.iter().rev() {
        inorder_helper(child, s+1);
    }
}

#[allow(dead_code)]
pub fn inorder(node: &ParseNode) {
    inorder_helper(node, 0);
}
// */

///////////////////// FILE I/O

fn read_lines<P>(filename: &P) -> io::Lines<io::BufReader<File>>
where P: AsRef<Path> + std::fmt::Debug, {
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't read file {:?}. Exit with: {}", filename, why),
    };
    io::BufReader::new(file).lines()
}


fn read_lr1(lr1: &String) -> Vec<HashMap<String,ReducePair>> {
    let mut lr_vec: Vec<HashMap<String,ReducePair>> = Vec::new();
    let lines = read_lines(lr1);
    for (idx, line) in lines.enumerate() {
        match line {
            Ok(l) => {
                // we have INT (STR | { STR, .. }) (shift|reduce) INT
                let mut line_vec: Vec<String> = l.split_whitespace()
                                                 .map(|x| String::from(x))
                                                 .collect::<Vec<String>>();

                // if the line is empty... ignore
                if line_vec.len() > 0 {
                    // get the first and last two items. 
                    let in_state: usize = line_vec[0].parse::<usize>().unwrap();
                    let out_state: usize = line_vec[line_vec.len()-1].parse::<usize>().unwrap();
                    let is_red: bool = line_vec[line_vec.len()-2] == "reduce";
                    
                    // remove them
                    line_vec.remove(0);
                    line_vec.pop(); line_vec.pop();

                    // get all STR and get rid of the set syntax
                    line_vec = line_vec.into_iter()
                                       .filter(|x| x != "{" && x != "}" && x != ",")
                                       .collect::<Vec<String>>();

                    // reserve more space if needed for our map
                    if lr_vec.len() <= in_state {
                        lr_vec.resize(in_state+1, HashMap::new());
                    }

                    // add all the strings to our map.
                    for item in line_vec.iter() {
                        lr_vec[in_state].insert(item.clone(), ReducePair(is_red, out_state));
                    }
                }
            },
            Err(_) => {panic!("Error reading {} at line no. {}", lr1, idx)}
        }
    }
    lr_vec
}

fn reduce(a: &String, state: &usize, dfa_vec: &Vec<HashMap<String, ReducePair>>) -> bool {
    if !dfa_vec[*state].contains_key(&*a) { return false; } 
    match dfa_vec[*state].get(&*a) {
        Some(item) => item.0, // ReducePair.0 is bool! 
        None => false,
    }
}

fn last<T>(item: &Vec<T>) -> Option<&T> {
    match item.len() {
        0 => None,
        n => Some(&item[n-1])
    }
}

fn state_top(item: &Vec<usize>) -> usize {
    *last::<usize>(item).unwrap()
}

pub fn parse(stream: &mut Vec<Token>, grammar: &String, lr1: &String) -> ParseNode {
    // returns iterator
    let grammar_lines: Vec<Vec<String>> = read_lines(grammar).map(
        |x| x.unwrap().split_whitespace().map(|y| String::from(y)).collect::<Vec<String>>()
    ).collect::<Vec<Vec<String>>>();

    let dfa_vec = read_lr1(lr1);

    // "augment" stream
    let mut toks: Vec<Token> = Vec::new();
    toks.push(Token("BOF".to_string(), Type::BOF));
    toks.append(stream);
    toks.push(Token("EOF".to_string(), Type::EOF));

    // we use vec as stacks
    let mut state_stack: Vec<usize> = Vec::new();
    let mut sym_stack: Vec<ParseNode> = Vec::new();

    state_stack.push(0); // push the beginning state

    // now. 
    for Token(lexeme, kind) in toks.iter() {
        // reduce as much as possible. 


        while reduce(&kind.to_string(), 
                     &state_top(&state_stack), 
                     &dfa_vec)
        {
            let rule_no: usize = dfa_vec[state_top(&state_stack)]
                                     .get(&kind.to_string())
                                     .unwrap()
                                     .1;
            let rule: Vec<String> = grammar_lines[rule_no].clone();

            let mut lhs_node: ParseNode = ParseNode(Rule(rule.clone()), Vec::new());
            for _ in 0..rule.len()-1 {
                lhs_node.1.push( sym_stack.pop().unwrap() );
                state_stack.pop();
            }

            sym_stack.push(lhs_node);
            state_stack.push( dfa_vec[state_top(&state_stack)].get(&rule[0]).unwrap().1 );
        } // end while 

        // push top character 
        sym_stack.push( ParseNode(Term(Token(lexeme.clone(),kind.clone())), Vec::new()) );
        match dfa_vec[state_top(&state_stack)].get(&kind.to_string()) {
            None => { panic!("Error parsing at: {}", lexeme); },
            Some(red_pair) => { state_stack.push( red_pair.1 ); }
        }

    } // end for

    // now we should have only BOF start_state EOF 
    let first_rule: Vec<String> = grammar_lines[0].clone();
    let mut root_node: ParseNode = ParseNode(Rule(first_rule.clone()), Vec::new());

    for item in first_rule.iter().rev() {
        // we don't care about the LHS!! 
        if *item == first_rule[0] {
            break
        } else {
            match sym_stack.pop() {
                None => { panic!("Error on final derivation, item is {} but stack is empty.", item); },
                Some(node) => {
                    if get_token(&node) == *item {
                        root_node.1.push(node); 
                    } else {
                        panic!("Error on final derivation, item is {}", item);
                    }
                }
            }
        }
    }

    root_node
}