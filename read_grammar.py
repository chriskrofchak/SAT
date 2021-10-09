# LHS -> RHS
# import sys
from itertools import groupby


def parse_CK(rules: 'list[list[str]]') -> 'list[tuple[str, list[str]]]':
    assert min([ len(rule) for rule in rules ]) > 2
    return [ (rule[0], rule[2:]) for rule in rules ]
    

# LHS ::= RHS | ... | ... 
# so '|' can't be used as a terminal
def parse_BN(rules: 'list[list[str]]') -> 'list[tuple[str, list[str]]]':
    assert min([ len(rule) for rule in rules ]) > 2
    rules = [ (rule[0], rule[2:]) for rule in rules ]
    rules = [ (rule, [item.split("..") for item in "..".join(prod).split("|")] ) for (rule, prod) in rules ]
    rules = [ (rule, [ [ string for string in item if len(string) > 0 ] for item in prod ]) for (rule, prod) in rules ]
    rules0 = rules
    rules = []
    for (lhs, prods) in rules0:
        for prod in prods:
            rules.append((lhs, prod))
    return rules

def parse_CS241(rules: 'list[list[str]]') -> 'list[tuple[str, list[str]]]':
    assert min([ len(rule) for rule in rules ]) > 1
    return [ (rule[0], rule[1:]) for rule in rules ]

def parse_lines(rules: 'list[list[str]]', char: str) -> 'list[tuple[str, list[str]]]':
    return (
        parse_BN(rules) if char == "::=" else (parse_CK(rules) if char == "->" else parse_CS241(rules))
    )


def check_grammar(filename):
    with open(filename, 'r') as f:
        lines = f.readlines()
    
    rules = [ [ item.strip() for item in line.split() ] for line in lines ]

    rules = parse_lines(rules, rules[0][1])

    LHS = set([ rule for (rule, _) in rules ])
    RHS = set.union(*[ set(deriv) for (_, deriv) in rules ])
    terms = set([ x for x in RHS if x not in LHS ])

    return (rules, terms)










def main():
    rules, terms = check_grammar("grammarBN.txt")
    return



if __name__ == "__main__":
    main()