use DfaInput::*;

#[derive(Clone, Debug, PartialEq, strum_macros::Display)]
pub enum Type {
    LPAREN,
    RPAREN,
    AND,
    NOT,
    OR,
    LIT,
    WHITESPACE,
    BOF,
    EOF,
    FAIL, // will never happen...
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token(pub String, pub Type);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    START,
    LPAREN,
    RPAREN,
    AND,
    ANDC,
    OR,
    ORC,
    NOT,
    NOTC,
    LIT,
    AA,
    AN,
    NN,
    NO,
    OO,
    WHITESPACE,
    FAIL,
}

enum DfaInput {
    CharIn(char),
    PredIn(Box<dyn Fn(char) -> bool>),
}

pub struct Transition(State, DfaInput, State);

fn delta(trans_vec: &Vec<Transition>, q: &State, e: &char) -> State {
    for Transition(s, dfa_in, next_state) in trans_vec.iter() {
        if *s == *q {
            match dfa_in {
                CharIn(c) => {
                    if *c == *e {
                        return *next_state;
                    }
                }
                PredIn(pred) => {
                    if pred(*e) {
                        return *next_state;
                    }
                }
            }
        }
    }
    State::FAIL
}

fn to_type(state: &State) -> Type {
    match state {
        State::LPAREN => Type::LPAREN,
        State::RPAREN => Type::RPAREN,
        State::AND => Type::AND,
        State::OR => Type::OR,
        State::NOT => Type::NOT,
        State::ANDC => Type::AND,
        State::ORC => Type::OR,
        State::NOTC => Type::NOT,
        State::AA => Type::LIT, // the on-the-way states to AND
        State::AN => Type::LIT, // the on-the-way states to AND
        State::OO => Type::LIT, // the on-the-way states to OR
        State::NN => Type::LIT, // the on-the-way states to NOT
        State::NO => Type::LIT, // the on-the-way states to NOT
        State::LIT => Type::LIT,
        State::WHITESPACE => Type::WHITESPACE,
        _ => Type::FAIL,
    }
}

// thank u CS UWATERLOO CS241 COURSE NOTES
fn simpl_max_munch(
    tokens: &mut Vec<Token>,
    trans_vec: &Vec<Transition>,
    accept_states: &Vec<State>,
    input: &String,
) {
    let mut s: State = State::START;
    let mut t: String = String::new();
    let mut iter = input.chars();
    let mut iter_c = iter.next();
    loop {
        match iter_c {
            Some(c) => {
                if delta(trans_vec, &s, &c) == State::FAIL {
                    if accept_states.contains(&s) {
                        // accept add token.
                        tokens.push(Token(t, to_type(&s)));
                    } else {
                        t.push(c);
                        panic!("Scan failed on input '{}'", t);
                    }
                    t = String::new();
                    s = State::START;
                } else {
                    s = delta(trans_vec, &s, &c);
                    t.push(c);
                    iter_c = iter.next();
                }
            }
            None => {
                break;
            }
        }
    }
    // leftover characters.
    if accept_states.contains(&s) {
        // accept add token.
        tokens.push(Token(t, to_type(&s)));
    }
}

pub fn scan(input: &String) -> Vec<Token> {
    let accept: Vec<State> = vec![
        State::LPAREN,
        State::RPAREN,
        State::AA,
        State::AN,
        State::AND,
        State::ANDC,
        State::OO,
        State::OR,
        State::ORC,
        State::NN,
        State::NO,
        State::NOT,
        State::NOTC,
        State::WHITESPACE,
        State::LIT,
    ];

    let mut trans_vec: Vec<Transition> = vec![
        // from start
        Transition(State::START, CharIn('('), State::LPAREN),
        Transition(State::START, CharIn(')'), State::RPAREN),
        // symbols and or not
        Transition(State::START, CharIn('|'), State::ORC),
        Transition(State::START, CharIn('&'), State::ANDC),
        Transition(State::START, CharIn('!'), State::NOTC),
        // AND
        Transition(State::START, CharIn('a'), State::AA),
        Transition(State::AA, CharIn('n'), State::AN),
        Transition(State::AN, CharIn('d'), State::AND),
        // NOT
        Transition(State::START, CharIn('n'), State::NN),
        Transition(State::NN, CharIn('o'), State::NO),
        Transition(State::NO, CharIn('t'), State::NOT),
        // OR
        Transition(State::START, CharIn('o'), State::OO),
        Transition(State::OO, CharIn('r'), State::OR),
        // whitespace
        Transition(
            State::START,
            PredIn(Box::new(|x| x.is_whitespace())),
            State::WHITESPACE,
        ),
        Transition(
            State::WHITESPACE,
            PredIn(Box::new(|x| x.is_whitespace())),
            State::WHITESPACE,
        ),
    ];

    let to_lit: Vec<State> = vec![
        State::START,
        State::AA,
        State::AN,
        State::AND,
        State::NN,
        State::NO,
        State::NOT,
        State::OO,
        State::OR,
        State::LIT,
    ];

    for item in to_lit.iter() {
        trans_vec.push(Transition(
            *item,
            PredIn(Box::new(|x| x.is_alphanumeric())),
            State::LIT,
        ));
    }

    // let temp: String = String::new();
    // let first_parse: Vec<Vec<Token>> = Vec::new();
    let mut temp_vec: Vec<Token> = Vec::new();

    simpl_max_munch(&mut temp_vec, &trans_vec, &accept, &input);

    let fin: Vec<Token> = temp_vec
        .into_iter()
        .filter(|Token(_, tok_type)| *tok_type != Type::WHITESPACE)
        .collect();
    // return the vector!

    fin
}
