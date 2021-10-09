use std::collections::HashMap;

use crate::cnf_parse::ParseNode;

use Connective::*;

enum Connective {
    And(Vec<Connective>),
    Or(Vec<Connective>),
    Literal(bool, String) // NOTs will be handled this way
}

fn cmp_vec<T: std::cmp::PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    return true;
}

fn str_to_String(vec: &Vec<&str>) -> Vec<String> {
    vec.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn make_rule_vec() -> Vec<(Vec<String>, String)> {
    let and_rule: Vec<&str> = vec![ "expr", "term", "AND", "expr" ];
    let or_rule: Vec<&str> = vec![ "disj", "term", "OR", "disj" ];
    let not_rule: Vec<&str> = vec![ "NOT", "LIT" ];
    let lit_rule: Vec<&str> = vec![ "LIT" ];

    vec![
        (str_to_String(&and_rule), "AND".to_string()),
        (str_to_String(&or_rule), "OR".to_string()),
        (str_to_String(&not_rule), "NOT".to_string()),
        (str_to_String(&lit_rule), "LIT".to_string())
    ]
}

fn rule_cntv(rule: &Vec<String>, vecs: Vec<(Vec<String>, String)>) -> Option<String> {
    for item in vecs.iter() {
        if cmp_vec::<String>(rule, &item.0) {
            return Some(item.1.to_string());
        }
    }
    None
}

fn make_ast_helper(node: &ParseNode, cntv: &mut Connective) -> Connective {
}

fn make_ast(node: &ParseNode) -> Connective {
    let mut cntv: Connective = And(Vec::new());
    make_ast_helper(node, &mut cntv)
}
