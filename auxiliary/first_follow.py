from read_grammar import check_grammar

def derives_epsilon(X, G, T):
    G_X = [ (rules, prod) for (rules, prod) in G if rules == X ]
    return False

# non terminal X and grammar G (list of rules) and T set of terminals
def first(X, G, T):
    # check if its a terminal.
    if X in T:
        return {X}
    G_X = [ (rules, prod) for (rules, prod) in G if rules == X ]
    first_X = set()

    # to get rid of infinite recursion, remove A -> A x T. 
    # since all other derivs A -> bla will work.
    G_X = [ (rules, prod) for (rules, prod) in G_X if prod[0] != rules ]

    for _, prod in G_X:
        if prod[0] in T:
            first_X = first_X.union({prod[0]})
        else:
            for item in prod:
                first_temp = first(item, G, T)
                first_X = first_X.union(first_temp - {'eps'})
                if 'eps' not in first_temp:
                    break
    return first_X

def dict_eq(d1: dict, d2: dict) -> bool:
    if ( d1.keys() != d2.keys() ):
        return False
    for d in d1.keys():
        if (d1[d] != d2[d]):
            return False 
    return True

def follow_iter(S, G, T, d):
    curr = d
    for X, prod in G:
            # start symbol
            if (X == S):
                curr[X] = curr[X].union({"EOF"})
            # X -> p B q
            for idx, item in enumerate(prod, 1):
                if item not in T:
                    if (idx < len(prod)):
                        curr[item] = curr[item].union(first(prod[idx], G, T) - {'eps'})
                        if 'eps' in first(prod[idx], G, T):
                            curr[item] = curr[item].union(curr[X])
                    else: # last elt.
                        curr[item] = curr[item].union(curr[X])
    return curr

# non terminal X, start S and grammar G and terminals T
def follow(S, G, T) -> 'dict[str, set[str]]':
    curr = { lhs:set() for (lhs, _) in G if lhs not in T }
    while (True):
        prev = curr
        curr = follow_iter(S, G, T, curr)
        # if no change... break!
        if dict_eq(curr, prev):
            break
    curr = follow_iter(S, G, T, curr)
    return curr
    





def main():
    rules, terms = check_grammar("grammar.txt")
    for rule in rules:
        print(rule)
    
    print("---")

    non_terms = list(set([ lhs for (lhs, _) in rules ]))
    for x in non_terms:
        print("first("+x+") =", first(x, rules, terms))

    dd = follow('S', rules, terms)
    for k in dd.keys():
        print(k, dd[k])


if __name__ == "__main__":
    main()
    # d1 = { "b":{'a', 'b'}, "c":{'d', 'dd'} }
    # d2 = { "b":{'a', 'b'}, "c":{'d', 'dd'} }
    # d3 = { "b":{'a', 'b'}, "e":{'d', 'dd'} }
    # d4 = { "b":{'a', 'b'}, "c":{'d', 'de'} }
    # print("d1, d2", dict_eq(d1, d2))
    # print("d1, d3", dict_eq(d1, d3))
    # print("d1, d4", dict_eq(d1, d4))
