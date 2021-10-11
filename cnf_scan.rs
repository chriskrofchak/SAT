#[derive(Clone, Debug, PartialEq, strum_macros::Display)]
pub enum Type {
    LPAREN,
    RPAREN,
    AND,
    OR,
    NOT,
    LIT,
    WHITESPACE,
    BOF,
    EOF,
    FAIL // will never happen...
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token(pub String, pub Type);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    START,
    LPAREN,
    RPAREN,
    AND,
    OR,
    NOT,
    LIT,
    AA,
    AN,
    NN,
    NO,
    OO,
    WHITESPACE,
    FAIL
}

pub struct Transition(State,char,State);

fn exists_transition(trans_vec: &Vec<Transition>, q: &State, e: &char) -> State {
    for Transition(s, c, next_state) in trans_vec.iter() {
        if *s == *q && *c == *e {
            return *next_state;
        }
    }
    State::FAIL
}

fn delta(trans_vec: &Vec<Transition>, q: &State, e: &char) -> State {
    let next_state: State = exists_transition(trans_vec, q, e);
    if !(next_state == State::FAIL) {
        return next_state;
    }
    else if *q != State::LPAREN 
            && *q != State::RPAREN 
            && *q != State::WHITESPACE 
            && e.is_alphabetic()
    {
        return State::LIT;
    } else if (*q == State::START || *q == State::WHITESPACE) && e.is_whitespace() {
        return State::WHITESPACE;
    }
    // else return FAIL.
    State::FAIL
}

fn to_type(state: &State) -> Type {
    match state {
        State::LPAREN     => Type::LPAREN,
        State::RPAREN     => Type::RPAREN,
        State::AND        => Type::AND,
        State::OR         => Type::OR,
        State::NOT        => Type::NOT, 
        State::AA         => Type::LIT, // the on-the-way states to AND          
        State::AN         => Type::LIT, // the on-the-way states to AND 
        State::OO         => Type::LIT, // the on-the-way states to OR 
        State::NN         => Type::LIT, // the on-the-way states to NOT 
        State::NO         => Type::LIT, // the on-the-way states to NOT 
        State::LIT        => Type::LIT,
        State::WHITESPACE => Type::WHITESPACE,
        _                 => Type::FAIL,
    }
}

// thank u CS UWATERLOO CS241 COURSE NOTES
fn simpl_max_munch( tokens: &mut Vec<Token>, 
        trans_vec: &Vec<Transition>, 
        accept_states: &Vec<State>,
        input: &String) {
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
                        tokens.push( Token(t, to_type(&s)) );
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
            None => { break; },
        }
    }
    // leftover characters.
    if accept_states.contains(&s) {
        // accept add token.
        tokens.push( Token(t, to_type(&s)) );
    }
}

pub fn scan(input: &String) -> Vec<Token> {
    let accept: Vec<State> = vec![ 
        State::LPAREN,
        State::RPAREN,
        State::AA,
        State::AN,
        State::AND,
        State::OO,
        State::OR,
        State::NN,
        State::NO,
        State::NOT,
        State::WHITESPACE,
        State::LIT
    ];

    let trans_vec: Vec<Transition> = vec![
        // from start
        Transition(State::START, '(', State::LPAREN),
        Transition(State::START, ')', State::RPAREN),
        // symbols and or not
        Transition(State::START, '|', State::OR),
        Transition(State::START, '&', State::AND),
        Transition(State::START, '!', State::NOT),
        // AND
        Transition(State::START, 'a', State::AA),
        Transition(State::AA,    'n', State::AN),
        Transition(State::AN,    'd', State::AND),
        // NOT
        Transition(State::START, 'n', State::NN),
        Transition(State::NN,    'o', State::NO),
        Transition(State::NO,    't', State::NOT),
        // OR
        Transition(State::START, 'o', State::OO),
        Transition(State::OO,    'r', State::OR)
    ];

    // let temp: String = String::new();
    // let first_parse: Vec<Vec<Token>> = Vec::new();
    let mut temp_vec: Vec<Token> = Vec::new();

    simpl_max_munch(&mut temp_vec, &trans_vec, &accept, &input);

    let fin: Vec<Token> = temp_vec.into_iter()
                                  .filter(|Token(_,tok_type)| *tok_type != Type::WHITESPACE)
                                  .collect();
    // return the vector! 

    fin
}