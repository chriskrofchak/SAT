use Connective::*;

#[derive(Debug, Clone)]
pub enum Connective {
    And(Vec<Connective>),
    Or(Vec<Connective>),
    Literal(bool, String) // NOTs will be handled this way
}

// don't NEED to use it yet... so we allow dead code
#[allow(dead_code)]
impl Connective {
    // add clause 
    pub fn add(&mut self, item: &Connective) {
        match self {
            And(vec) | Or(vec) => {vec.push(item.clone()); },
            _ => {}
        }
    }

    // check is connective is empty clause aka ()
    pub fn empty(&self) -> bool {
        match self {
            And(vec) | Or(vec) => vec.is_empty(),
            Literal(_,_)       => false,
        }
    }

    // removes predicate...
    pub fn remove(&mut self, pred: &Connective) {
        // calling remove on a Literal does nothing.
        match self {
            And(vec) | Or(vec) => {
                if let Some(index) = vec.iter().position(|x| *x == *pred) {
                    vec.remove(index);
                }
            },
            Literal(_,_) => {}
        }
    }
}

fn conn_vec_eq(lhs: &Vec<Connective>, rhs: &Vec<Connective>) -> bool {
    if lhs.len() == rhs.len() {
        for (l, r) in lhs.iter().zip(rhs) {
            if l != r {
                return false;
            }
        }
        return true;
    } else {
        return false;
    }
}

impl PartialEq for Connective {
    fn eq(&self, other: &Self) -> bool {
        match self {
            // individual arms unavoidable...
            And(vec) => {
                match other {
                    And(rhs_vec) => conn_vec_eq(vec, rhs_vec),
                    _ => false,
                }
            },
            Or(vec) => {
                match other {
                    Or(rhs_vec) => conn_vec_eq(vec, rhs_vec),
                    _ => false
                }
            },
            Literal(is_not, lex) => {
                match other {
                    Literal(rhs_not, rhs_lex) => ((is_not == rhs_not) 
                                                  && (lex == rhs_lex)),
                    _ => false
                }
            }
        }
    }

    fn ne(&self, other: &Self) -> bool {
        return !self.eq(other);
    }
}


fn ast_print_helper(node: &Connective, s: u16) {
    for _ in 0..s {
        print!(" ");
    }
    match node {
        // to prevent repeated code
        And(vec) | Or(vec) => { 
            // to print appropriately
            match node {
                And(_) => {println!("And(");},
                Or(_) => {println!("Or(");},
                _ => {}
            }
            // then rest printing
            for child in vec.iter() {
                ast_print_helper(child, s+2);
            }
            for _ in 0..s {
                print!(" ");
            }
            println!(")");
        },
        Connective::Literal(is_not, lit) => {
            if *is_not {
                print!("NOT ");
            }
            println!("{}", lit);
        }
    }
}

pub fn ast_print(node: &Connective) {
    ast_print_helper(node, 0);
}