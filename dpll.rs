use std::collections::HashMap;

use crate::connective::Connective::{self, *};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bv {
    UND,
    TRUE,
    FALSE,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Res {
    SAT,
    UNSAT,
}

#[allow(dead_code)]
fn unset_literal(conn: &Connective, map: &mut HashMap<String, Bv>) {
    if let Literal(_, lex) = conn {
        map.insert(lex.clone(), Bv::UND);
    }
}

fn set_literal(conn: &Connective, map: &mut HashMap<String, Bv>) {
    if let Literal(is_not, lex) = conn {
        let bv: Bv = if *is_not { Bv::FALSE } else { Bv::TRUE };
        map.insert(lex.clone(), bv);
    }
}

fn eval_literal(conn: &Connective, map: &HashMap<String, Bv>) -> bool {
    match conn {
        Literal(is_not, lex) => {
            (((!*is_not) && (*map.get(lex).unwrap() == Bv::TRUE))
                || (*is_not && (*map.get(lex).unwrap() == Bv::FALSE)))
        }
        _ => false, // since Ors and Ands (not literals/atoms we will say do not eval to true)
    }
}

fn und_literal(conn: &Connective, map: &HashMap<String, Bv>) -> bool {
    match conn {
        Literal(_, lex) => *map.get(lex).unwrap() == Bv::UND,
        _ => false, // since Ors and Ands (not literals/atoms we will say do not eval to true)
    }
}

#[allow(dead_code)]
fn unit_propagation() {}

#[allow(dead_code)]
fn pure_literal_elimination() {}

fn dpll_helper(conn: &Connective, in_map: &HashMap<String, Bv>) -> (Res, HashMap<String, Bv>) {
    let curr_conn = conn.clone();
    let mut map = in_map.clone();
    let mut fin_conn: Option<Connective> = None;

    if let And(mut vec) = curr_conn {
        // empty is good --> satisfiable!
        if vec.is_empty() {
            return (Res::SAT, map);
        }

        // UNIT PROPAGATION
        // check for literal conflicts!
        for item in vec.iter() {
            match item {
                Literal(is_not, lex) => {
                    if !und_literal(item, &map) {
                        if (*map.get(lex).unwrap() == Bv::TRUE && *is_not)
                            || (*map.get(lex).unwrap() == Bv::FALSE && !*is_not)
                        {
                            return (Res::UNSAT, map);
                        }
                    }
                    set_literal(item, &mut map);
                }
                And(inn_vec) | Or(inn_vec) => {
                    if inn_vec.len() == 1 {
                        set_literal(&inn_vec[0], &mut map);
                    }
                }
            }
        }

        // filter out everything!.
        vec.retain(|x| {
            match x {
                Literal(_, _) => false,
                Or(inn_vec) => {
                    for item in inn_vec.iter() {
                        if eval_literal(item, &map) {
                            return false;
                        }
                    }
                    true
                }
                And(_) => false, // shouldn't happen... get rid of it tho!
            }
        });

        // now remove all remaining predicates in Or...
        for item in vec.iter_mut() {
            if let Or(inn_vec) = item {
                // we will have already removed the Or(...)
                // if any evals to true.
                // so this simply trims anything which evaluates to false.
                inn_vec.retain(|x| und_literal(x, &map) || eval_literal(x, &map));
            }
        }

        // exit at any time if we have completed all clauses
        if vec.is_empty() {
            return (Res::SAT, map);
        }

        // PURE LITERAL ELIMINATION
        // we add all pure literals to a set
        // then we remove all pure literals
        let mut pure_lit: HashMap<String, bool> = HashMap::new();
        for item in vec.iter() {
            match item {
                And(inn_vec) | Or(inn_vec) => {
                    for inn_item in inn_vec.iter() {
                        if let Literal(is_not, lex) = inn_item {
                            if pure_lit.contains_key(lex) {
                                if *pure_lit.get(lex).unwrap() != *is_not {
                                    pure_lit.remove(lex);
                                }
                            } else {
                                pure_lit.insert(lex.clone(), *is_not);
                            }
                        }
                    }
                }
                Literal(is_not, lex) => {
                    if pure_lit.contains_key(lex) {
                        if *pure_lit.get(lex).unwrap() != *is_not {
                            pure_lit.remove(lex);
                        }
                    } else {
                        pure_lit.insert(lex.clone(), *is_not);
                    }
                }
            }
        }

        // iterate and add BV for all pure lits
        for key in pure_lit.keys() {
            set_literal(&Literal(*pure_lit.get(key).unwrap(), key.clone()), &mut map);
        }

        // remove top level literals
        vec.retain(|x| {
            matches!(x, Literal(_,lex) if !pure_lit.contains_key(lex))
                || matches!(x, Or(vec) if !vec.iter().any(
                    |y| matches!(y, Literal(_,lex) if pure_lit.contains_key(lex))
                ))
        });

        // exit at any time if we have completed all clauses
        if vec.is_empty() {
            return (Res::SAT, map);
        }

        // EMPTY CLAUSE RETURN UNSAT
        for item in vec.iter() {
            if let Or(inn_vec) = item {
                if inn_vec.is_empty() {
                    return (Res::UNSAT, map);
                }
            }
        }

        fin_conn = Some(And(vec));
    }

    // ELSE. choose a literal. recur.
    // if that is unsat try with (not lit)
    let mut next_lit: Option<String> = None;
    for item in map.keys() {
        if *map.get(item).unwrap() == Bv::UND {
            next_lit = Some(item.clone());
            break;
        }
    }

    // there is no more undecided variables...
    if let None = next_lit {
        return (Res::SAT, map);
    } // else

    let lit_str = next_lit.unwrap();
    let mut final_conn = fin_conn.unwrap();

    final_conn.add(&Literal(false, lit_str.clone()));
    let res = dpll_helper(&final_conn, &map);

    if res.0 == Res::SAT {
        return res;
    } // else

    // try with not false.
    final_conn.remove(&Literal(false, lit_str.clone()));
    final_conn.add(&Literal(true, lit_str.clone()));
    dpll_helper(&final_conn, &map)
}

fn add_literals(conn: &Connective, map: &mut HashMap<String, Bv>) {
    match conn {
        And(vec) | Or(vec) => {
            for item in vec.iter() {
                add_literals(item, map);
            }
        }
        Literal(_, lex) => {
            map.insert(lex.clone(), Bv::UND);
        }
    }
}

pub fn dpll(conn: &Connective) -> (Res, HashMap<String, Bv>) {
    let mut map: HashMap<String, Bv> = HashMap::new();
    add_literals(conn, &mut map);
    match conn {
        And(_) => dpll_helper(conn, &map),
        Or(vec) => {
            if vec.len() > 0 {
                set_literal(&vec[0], &mut map);
            }
            (Res::SAT, map)
        }
        Literal(_, _) => {
            set_literal(conn, &mut map);
            (Res::SAT, map)
        }
    }
}
