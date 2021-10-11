# CK CNF-SAT 

It's a bit of a misnomer to call it my own since I didn't invent anything but I do like the ring of CK-\[ ... \], so forgive the name. 

The crux of it is that I implemented an SLR parser (thank you to [CS444/CS241 course notes](https://student.cs.uwaterloo.ca/~cs444/parse.pdf) for reference) to take input and make an abstract syntax tree (AST) which I then traverse using either DPLL or CDCL. There are command line arguments to choose which. The DPLL algorithm I followed using the [Wikipedia pseudocode](https://en.wikipedia.org/wiki/DPLL_algorithm#The_algorithm) and the CDCL algorithm is from the published [MiniSAT paper](http://minisat.se/downloads/MiniSat.pdf) on the very heuristic. I go through the topics more in depth below.

## Scanner

So this at least I did completely on my own. We learned about DFAs and Simplified Maximal Munch in CS241, and so I made a simple DFA which accepts `and`, `or`, `not`, `!`, `|`, and `&` as keywords, I think you know what for. You can use them interchangeably as well, so `p and q & not s and !t` will tokenize as expected. Also, if you continue typing characters not separated by whitespace the keyword (not the symbols though) will turn into a literal. So `andine & notabene` does not tokenize as `AND LIT AND NOT LIT` but simply as `LIT AND LIT`.

## Parser 

The grammar is a very simple one since conjunctive normal form (CNF) has stricter properties than just any propositional logic formula. There cannot be any nested disjuncts inside a `NOT` of a clause, or a nested conjunct inside a disjunct clause... So we are already accepting very simple structured input (the parser will reject incorrect CNF). The grammar I ended up using is virtually verbatim from Carlo Tomasi's [course notes](https://courses.cs.duke.edu//compsci230/cps102/fall06/notes/logic.pdf) (found on page 27) on logic.

This is the grammar.

```
S    ::= BOF expr EOF
expr ::= term | term AND expr
term ::= pred | ( disj ) 
disj ::= pred | pred OR disj 
pred ::= NOT LIT | LIT
```

Then, the SLR Automaton for the above grammar is as follows. I numbered the states when I drew out the automaton by hand but I didn't add the numbers here. 

![](/Flowchart.jpeg)

The grey states are accepting states (reduce) and then the bolded lines in non grey states are when we should reduce. Luckily we have no shift reduce conflicts and so we can easily parse using SLR. I used the LR1 DFA format as described on the [CS241 course page](https://student.cs.uwaterloo.ca/~cs241/parsing/lr1.html) as I think it is an efficient way to write out the DFA. This is normally a shift reduce table but the format once the file read is as follows:

```Rust
Vec<std::collections::HashMap<String, (bool, usize)>>.
```

It converts the DFA into a vector of maps which use characters as keys. So we check to ensure we never exceed the size of the vector, but the input state is how we index the vector. Then we retrieve a pair of `bool` and `usize` which correspond with whether to reduce and either the state to shift to or the rule to reduce by. 

I was going to write a DFA table generator as well but that's a bit too much to chew right now. The reason I didn't use lex or yacc for this project is given I wanted to learn Rust / improve my Rust skills and lex and yacc produce C code. Next time I will do something in C, I want to familiarize myself with parser generator tools like Bison...

## Making the AST 

Most of this is hardcoded unfortunately, I have a vector which contains each rule and a "decision". 
I love `enum`s in `Rust` and so I use

```Rust
enum Decision {
    Recur,
    AND,
    OR,
    Term
}

enum Connective {
    And(Vec<Connective>),
    Or(Vec<Connective>),
    Literal(bool, String)
}
```

then depending on the rule I convert the AST of rules and tokens, into an `And(_)` variant of `Connective` and recur, or an `Or(_)` or it terminates at `Literal(_,_)` and depending on the rule sets the boolean to true and returns the lexeme of the token to `String`.

This will result in a nested list for conjuncts and disjuncts of length greater than two, since something like `a and b and c` will parse to `And([a, And([b, c])])`. So we have to then simplify the tree so that we flatten any nesting of the same type. Finally for an input like 

```
p and q and r and s and (a or b or c)
```

we return a `Connective` as follows

```
And([p, q, r, s, Or([a, b, c])]).
```

This is much easier to pass to DPLL/CDCL for satisfiability checking.

## DPLL 

## CDCL 