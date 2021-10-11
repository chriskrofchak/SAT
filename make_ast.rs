// from cnf_parse
use crate::cnf_parse::ParseNode;
use crate::cnf_parse::ParseVal;
use crate::cnf_parse::get_token;

// from connective
use crate::connective::Connective::{self, *};

////////////////////
/// ENUMS for making AST

#[derive(Debug, Copy, Clone)]
enum Decision {
    Recur,
    AND,
    OR,
    Term
}

// HELPER FUNCTIONS

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

fn str_to_string(vec: &Vec<&str>) -> Vec<String> {
    vec.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn make_rule_vec() -> Vec<(Vec<String>, Decision, Vec<usize>)> {
    let rule_vec = vec![
        (vec![ "S"   ,  "BOF", "expr", "EOF" ]        , Decision::Recur ,   vec![1]),
        (vec![ "expr",  "term" ]                      , Decision::Recur ,   vec![0]),  
        (vec![ "expr",  "term", "AND", "expr" ]       , Decision::AND   ,   vec![2,0]),  
        (vec![ "term",  "pred" ]                      , Decision::Recur ,   vec![0]),  
        (vec![ "term",  "LPAREN", "disj", "RPAREN" ]  , Decision::Recur ,   vec![1]),  
        (vec![ "disj",  "pred" ]                      , Decision::Recur ,   vec![0]),  
        (vec![ "disj",  "pred", "OR", "disj" ]        , Decision::OR    ,   vec![2,0]),  
        (vec![ "pred",  "NOT", "LIT" ]                , Decision::Term  ,   vec![0]),  
        (vec![ "pred",  "LIT" ]                       , Decision::Term  ,   vec![0])
    ];
    rule_vec.iter()
            .map(|x| (str_to_string(&x.0), x.1, x.2.clone()) )
            .collect::<Vec<(Vec<String>, Decision, Vec<usize>)>>()
}

fn rule_cntv<'a>(rule: &'a Vec<String>, vecs: &'a Vec<(Vec<String>, Decision, Vec<usize>)>) 
    -> Option<&'a(Vec<String>, Decision, Vec<usize>)> {
    for item in vecs.iter() {
        if cmp_vec::<String>(rule, &item.0) {
            return Some(&item);
        }
    }
    None
}

fn make_ast_helper(node: &ParseNode, vec: &Vec<(Vec<String>, Decision, Vec<usize>)>) 
    -> Option<Connective> {
    match &node.0 {
        ParseVal::Rule(rule) => {
            let thrup: &(Vec<String>, Decision, Vec<usize>) = rule_cntv(rule, &vec)?;
            match thrup.1 {
                Decision::Recur => make_ast_helper(&node.1[thrup.2[0]], vec),
                Decision::AND => Some(Connective::And(vec![ 
                    make_ast_helper(&node.1[thrup.2[0]], vec).unwrap(), 
                    make_ast_helper(&node.1[thrup.2[1]], vec).unwrap()
                ])),
                Decision::OR => Some(Connective::Or(vec![ 
                    make_ast_helper(&node.1[thrup.2[0]], vec).unwrap(), 
                    make_ast_helper(&node.1[thrup.2[1]], vec).unwrap()
                ])),
                Decision::Term => {
                    if thrup.0[1] == "NOT" {
                        return Some(Connective::Literal(true, get_token(&node.1[thrup.2[0]])))
                    }
                    else {
                        return Some(Connective::Literal(false, get_token(&node.1[thrup.2[0]])))
                    }
                }
            }
        },
        ParseVal::Term(_) => {
            None
        }
    }
}

pub fn simplify_ast(node: &Connective) -> Connective {
    match node {
        And(vec) => {
            let mut conn: Connective = And(Vec::new());
            for child in vec.iter() {
                let simpl: Connective = simplify_ast(child);
                if let And(inn_vec) = simpl {
                    for sub_child in inn_vec.iter() {
                        conn.add(sub_child);
                    }
                } else {
                    conn.add(&simpl);
                }
            }
            conn
        },
        Or(vec) => {
            let mut conn: Connective = Or(Vec::new());
            for child in vec.iter() {
                let simpl: Connective = simplify_ast(child);
                if let Or(inn_vec) = simpl {
                    for sub_child in inn_vec.iter() {
                        conn.add(sub_child);
                    }
                } else {
                    conn.add(&simpl);
                }
            }
            conn
        },
        Literal(is_not, lex) => Literal(*is_not, lex.clone()),
    }
}

pub fn make_ast(node: &ParseNode) -> Option<Connective> {
    let rule_table: Vec<(Vec<String>, Decision, Vec<usize>)> = make_rule_vec();
    make_ast_helper(node, &rule_table)
}
