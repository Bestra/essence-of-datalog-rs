use std::collections::HashMap;

fn build_ancestor_program() -> Program {
    // Facts: adviser relationships
    let facts = vec![
        Rule {
            head: Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::sym("Andrew Rice"), Term::sym("Mistral Contrastin")],
            },
            body: vec![],
        },
        Rule {
            head: Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::sym("Dominic Orchard"), Term::sym("Mistral Contrastin")],
            },
            body: vec![],
        },
        Rule {
            head: Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::sym("Andy Hopper"), Term::sym("Andrew Rice")],
            },
            body: vec![],
        },
        Rule {
            head: Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::sym("Alan Mycroft"), Term::sym("Dominic Orchard")],
            },
            body: vec![],
        },
        Rule {
            head: Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::sym("David Wheeler"), Term::sym("Andy Hopper")],
            },
            body: vec![],
        },
        Rule {
            head: Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::sym("Rod Burstall"), Term::sym("Alan Mycroft")],
            },
            body: vec![],
        },
        Rule {
            head: Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::sym("Robin Milner"), Term::sym("Alan Mycroft")],
            },
            body: vec![],
        },
    ];
    
    // Rules: academicAncestor definition
    let rules = vec![
        // academicAncestor(X,Y) :- adviser(X,Y).
        Rule {
            head: Atom {
                pred_sym: "academicAncestor".to_string(),
                terms: vec![Term::var("X"), Term::var("Y")],
            },
            body: vec![Atom {
                pred_sym: "adviser".to_string(),
                terms: vec![Term::var("X"), Term::var("Y")],
            }],
        },
        // academicAncestor(X,Z) :- adviser(X,Y), academicAncestor(Y,Z).
        Rule {
            head: Atom {
                pred_sym: "academicAncestor".to_string(),
                terms: vec![Term::var("X"), Term::var("Z")],
            },
            body: vec![
                Atom {
                    pred_sym: "adviser".to_string(),
                    terms: vec![Term::var("X"), Term::var("Y")],
                },
                Atom {
                    pred_sym: "academicAncestor".to_string(),
                    terms: vec![Term::var("Y"), Term::var("Z")],
                },
            ],
        },
    ];
    
    // Queries
    let queries = vec![
        // Query 1: ?- academicAncestor("Robin Milner", Intermediate), academicAncestor(Intermediate, "Mistral Contrastin")
        Rule {
            head: Atom {
                pred_sym: "query1".to_string(),
                terms: vec![Term::var("Intermediate")],
            },
            body: vec![
                Atom {
                    pred_sym: "academicAncestor".to_string(),
                    terms: vec![Term::sym("Robin Milner"), Term::var("Intermediate")],
                },
                Atom {
                    pred_sym: "academicAncestor".to_string(),
                    terms: vec![Term::var("Intermediate"), Term::sym("Mistral Contrastin")],
                },
            ],
        },
        // Query 2: ?- academicAncestor("Alan Turing", "Mistral Contrastin")
        Rule {
            head: Atom {
                pred_sym: "query2".to_string(),
                terms: vec![],
            },
            body: vec![Atom {
                pred_sym: "academicAncestor".to_string(),
                terms: vec![Term::sym("Alan Turing"), Term::sym("Mistral Contrastin")],
            }],
        },
        // Query 3: ?- academicAncestor("David Wheeler", "Mistral Contrastin")
        Rule {
            head: Atom {
                pred_sym: "query3".to_string(),
                terms: vec![],
            },
            body: vec![Atom {
                pred_sym: "academicAncestor".to_string(),
                terms: vec![Term::sym("David Wheeler"), Term::sym("Mistral Contrastin")],
            }],
        },
    ];
    
    // Combine all into program
    facts.into_iter().chain(rules).chain(queries).collect()
}

fn main() {
    println!("=== Essence of Datalog - Rust Implementation ===\n");
    
    // Build the ancestor program
    let program = build_ancestor_program();
    
    println!("Running the ancestor program...\n");
    
    // Solve the program
    let kb = solve(&program);
    
    println!("Knowledge base after solving ({} facts):", kb.len());
    println!("(Showing academicAncestor relationships only)\n");
    
    // Display academicAncestor relationships
    for atom in kb.iter().filter(|a| a.pred_sym == "academicAncestor") {
        if let [Term::Sym(from), Term::Sym(to)] = &atom.terms[..] {
            println!("  {} -> {}", from, to);
        }
    }
    
    println!("\n=== Running Queries ===\n");
    
    // Query 1: Who is an intermediate ancestor between Robin Milner and Mistral Contrastin?
    println!("Query 1: Who connects Robin Milner to Mistral Contrastin?");
    let results1 = query("query1", &program);
    if results1.is_empty() {
        println!("  No results found.");
    } else {
        for sub in &results1 {
            if let Some(Term::Sym(name)) = sub.get(&Term::var("Intermediate")) {
                println!("  Intermediate: {}", name);
            }
        }
    }
    
    // Query 2: Is Alan Turing an ancestor of Mistral Contrastin?
    println!("\nQuery 2: Is Alan Turing an ancestor of Mistral Contrastin?");
    let results2 = query("query2", &program);
    if results2.is_empty() {
        println!("  No");
    } else {
        println!("  Yes");
    }
    
    // Query 3: Is David Wheeler an ancestor of Mistral Contrastin?
    println!("\nQuery 3: Is David Wheeler an ancestor of Mistral Contrastin?");
    let results3 = query("query3", &program);
    if results3.is_empty() {
        println!("  No");
    } else {
        println!("  Yes");
    }
    
    println!("\n=== Complete! ===");
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Term {
    Var(String),
    Sym(String),
}

impl Term {
    fn var(s: &str) -> Term {
        Term::Var(s.to_string())
    }
    fn sym(s: &str) -> Term {
        Term::Sym(s.to_string())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Atom {
    pub pred_sym: String,
    pub terms: Vec<Term>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub head: Atom,
    pub body: Vec<Atom>,
}

pub type Program = Vec<Rule>;
pub type KnowledgeBase = Vec<Atom>;
pub type Substitution = HashMap<Term, Term>;

impl Atom {
    fn substitute(&self, substitution: &Substitution) -> Atom {
        Atom {
            pred_sym: self.pred_sym.clone(),
            terms: self
                .terms
                .iter()
                .map(|t| match t {
                    Term::Sym(_) => t.clone(),
                    Term::Var(_) => substitution.get(t).cloned().unwrap_or_else(|| t.clone()),
                })
                .collect(),
        }
    }
}

fn empty_substitution() -> Substitution {
    HashMap::new()
}

fn unify(atom1: &Atom, atom2: &Atom) -> Option<Substitution> {
    // Check if predicate symbols match
    if atom1.pred_sym != atom2.pred_sym {
        return None;
    }
    
    // Check if arity matches
    if atom1.terms.len() != atom2.terms.len() {
        return None;
    }
    
    let mut substitution = empty_substitution();
    
    // Go through each pair of terms
    for (t1, t2) in atom1.terms.iter().zip(atom2.terms.iter()) {
        match (t1, t2) {
            // Symbol-to-symbol: must be equal
            (Term::Sym(s1), Term::Sym(s2)) => {
                if s1 != s2 {
                    return None;
                }
            }
            // Variable-to-symbol: add to substitution
            (Term::Var(_), Term::Sym(_)) => {
                // Check if we already have a binding for this variable
                if let Some(existing) = substitution.get(t1) {
                    // If we do, it must match the current symbol
                    if existing != t2 {
                        return None;
                    }
                } else {
                    // Add new binding
                    substitution.insert(t1.clone(), t2.clone());
                }
            }
            // Variable in second atom: not allowed (second atom should be ground)
            (_, Term::Var(_)) => {
                panic!("Cannot unify with a variable in the second position. Expected a concrete value.");
            }
        }
    }
    
    Some(substitution)
}

fn eval_atom(kb: &KnowledgeBase, atom: &Atom, substitutions: Vec<Substitution>) -> Vec<Substitution> {
    let mut result = Vec::new();
    
    for substitution in substitutions {
        // Apply the substitution to the atom
        let grounded_atom = atom.substitute(&substitution);
        
        // Try to unify with each fact in the knowledge base
        for fact in kb {
            if let Some(extension) = unify(&grounded_atom, fact) {
                // Extend the substitution
                let mut extended = substitution.clone();
                extended.extend(extension);
                result.push(extended);
            }
        }
    }
    
    result
}

fn walk(kb: &KnowledgeBase, body: &[Atom]) -> Vec<Substitution> {
    let mut substitutions = vec![empty_substitution()];
    
    for atom in body {
        substitutions = eval_atom(kb, atom, substitutions);
    }
    
    substitutions
}

fn eval_rule(kb: &KnowledgeBase, rule: &Rule) -> KnowledgeBase {
    let substitutions = walk(kb, &rule.body);
    
    substitutions
        .iter()
        .map(|sub| rule.head.substitute(sub))
        .collect()
}

fn is_range_restricted(rule: &Rule) -> bool {
    // Get all variables from the head
    let head_vars: Vec<&Term> = rule.head.terms.iter()
        .filter(|t| matches!(t, Term::Var(_)))
        .collect();
    
    // Get all variables from the body
    let body_vars: Vec<&Term> = rule.body.iter()
        .flat_map(|atom| &atom.terms)
        .filter(|t| matches!(t, Term::Var(_)))
        .collect();
    
    // Check if all head variables appear in the body
    head_vars.iter().all(|hv| body_vars.contains(hv))
}

fn immediate_consequence(rules: &Program, kb: &KnowledgeBase) -> KnowledgeBase {
    // Evaluate each rule independently
    let mut new_facts: Vec<Atom> = rules.iter()
        .flat_map(|rule| eval_rule(kb, rule))
        .collect();
    
    // Add existing facts
    new_facts.extend(kb.iter().cloned());
    
    // Remove duplicates using a HashSet
    let unique_facts: std::collections::HashSet<_> = new_facts.into_iter().collect();
    unique_facts.into_iter().collect()
}

fn solve(rules: &Program) -> KnowledgeBase {
    // Check if all rules are range-restricted
    if !rules.iter().all(is_range_restricted) {
        panic!("The input program is not range-restricted. All variables in a rule's head must appear in its body to ensure termination.");
    }
    
    let mut kb = Vec::new();
    
    loop {
        let next_kb = immediate_consequence(rules, &kb);
        
        // Check if we've reached a fixpoint
        if next_kb.len() == kb.len() {
            // Need to check if they're actually the same
            let kb_set: std::collections::HashSet<_> = kb.iter().collect();
            let next_set: std::collections::HashSet<_> = next_kb.iter().collect();
            
            if kb_set == next_set {
                return next_kb;
            }
        }
        
        kb = next_kb;
    }
}

fn query(pred_sym: &str, pr: &Program) -> Vec<Substitution> {
    let kb = solve(pr);
    
    // Find all atoms in the knowledge base that match the query predicate
    let matching_atoms: Vec<&Atom> = kb.iter()
        .filter(|atom| atom.pred_sym == pred_sym)
        .collect();
    
    // Find the query rule(s) with this predicate
    let query_rules: Vec<&Rule> = pr.iter()
        .filter(|rule| rule.head.pred_sym == pred_sym)
        .collect();
    
    if query_rules.is_empty() {
        let mut available_predicates: Vec<_> = pr.iter()
            .map(|r| r.head.pred_sym.as_str())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        available_predicates.sort();
        panic!("The query '{}' doesn't exist. Available predicates: {:?}", pred_sym, available_predicates);
    }
    
    if query_rules.len() > 1 {
        panic!("The query '{}' has multiple clauses. Queries must be defined with a single rule.", pred_sym);
    }
    
    let query_rule = query_rules[0];
    let query_vars = &query_rule.head.terms;
    
    // Build substitutions from the matching atoms
    matching_atoms.iter()
        .map(|atom| {
            let mut sub = empty_substitution();
            for (var, sym) in query_vars.iter().zip(atom.terms.iter()) {
                sub.insert(var.clone(), sym.clone());
            }
            sub
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_substitute() {
        let a = Atom {
            pred_sym: "adviser".to_string(),
            terms: vec![Term::var("X"), Term::var("Y")],
        };
        let mut subs = HashMap::new();
        subs.insert(Term::var("X"), Term::var("Y"));
        subs.insert(Term::var("Y"), Term::sym("Dude"));
        let new_a = a.substitute(&subs);
        assert_eq!(new_a.terms, vec![Term::var("Y"), Term::sym("Dude")]);
        
        // Test that unbound variables are left unchanged
        let a2 = Atom {
            pred_sym: "test".to_string(),
            terms: vec![Term::var("Z")],
        };
        let new_a2 = a2.substitute(&subs);
        assert_eq!(new_a2.terms, vec![Term::var("Z")]);
    }

    #[test]
    fn test_unify_same_predicate() {
        let atom1 = Atom {
            pred_sym: "adviser".to_string(),
            terms: vec![Term::var("X"), Term::var("Y")],
        };
        let atom2 = Atom {
            pred_sym: "adviser".to_string(),
            terms: vec![Term::sym("Alice"), Term::sym("Bob")],
        };
        
        let result = unify(&atom1, &atom2);
        assert!(result.is_some());
        let sub = result.unwrap();
        assert_eq!(sub.get(&Term::var("X")), Some(&Term::sym("Alice")));
        assert_eq!(sub.get(&Term::var("Y")), Some(&Term::sym("Bob")));
    }

    #[test]
    fn test_unify_different_predicate() {
        let atom1 = Atom {
            pred_sym: "adviser".to_string(),
            terms: vec![Term::var("X")],
        };
        let atom2 = Atom {
            pred_sym: "student".to_string(),
            terms: vec![Term::sym("Alice")],
        };
        
        let result = unify(&atom1, &atom2);
        assert!(result.is_none());
    }

    #[test]
    fn test_unify_repeated_variables() {
        // p(X, X) should unify with p("a", "a")
        let atom1 = Atom {
            pred_sym: "p".to_string(),
            terms: vec![Term::var("X"), Term::var("X")],
        };
        let atom2 = Atom {
            pred_sym: "p".to_string(),
            terms: vec![Term::sym("a"), Term::sym("a")],
        };
        
        let result = unify(&atom1, &atom2);
        assert!(result.is_some());
        
        // p(X, X) should NOT unify with p("a", "b")
        let atom3 = Atom {
            pred_sym: "p".to_string(),
            terms: vec![Term::sym("a"), Term::sym("b")],
        };
        
        let result2 = unify(&atom1, &atom3);
        assert!(result2.is_none());
    }

    #[test]
    fn test_unify_symbols() {
        // Symbols must match exactly
        let atom1 = Atom {
            pred_sym: "p".to_string(),
            terms: vec![Term::sym("a"), Term::sym("b")],
        };
        let atom2 = Atom {
            pred_sym: "p".to_string(),
            terms: vec![Term::sym("a"), Term::sym("b")],
        };
        
        let result = unify(&atom1, &atom2);
        assert!(result.is_some());
        
        let atom3 = Atom {
            pred_sym: "p".to_string(),
            terms: vec![Term::sym("a"), Term::sym("c")],
        };
        
        let result2 = unify(&atom1, &atom3);
        assert!(result2.is_none());
    }

    #[test]
    fn test_is_range_restricted() {
        // Range-restricted: all head variables appear in body
        let rule1 = Rule {
            head: Atom {
                pred_sym: "p".to_string(),
                terms: vec![Term::var("X")],
            },
            body: vec![Atom {
                pred_sym: "q".to_string(),
                terms: vec![Term::var("X")],
            }],
        };
        assert!(is_range_restricted(&rule1));
        
        // Not range-restricted: Y in head but not in body
        let rule2 = Rule {
            head: Atom {
                pred_sym: "p".to_string(),
                terms: vec![Term::var("X"), Term::var("Y")],
            },
            body: vec![Atom {
                pred_sym: "q".to_string(),
                terms: vec![Term::var("X")],
            }],
        };
        assert!(!is_range_restricted(&rule2));
        
        // Range-restricted with multiple body atoms
        let rule3 = Rule {
            head: Atom {
                pred_sym: "p".to_string(),
                terms: vec![Term::var("X"), Term::var("Z")],
            },
            body: vec![
                Atom {
                    pred_sym: "q".to_string(),
                    terms: vec![Term::var("X"), Term::var("Y")],
                },
                Atom {
                    pred_sym: "r".to_string(),
                    terms: vec![Term::var("Y"), Term::var("Z")],
                },
            ],
        };
        assert!(is_range_restricted(&rule3));
    }

    #[test]
    fn test_ancestor_program_solve() {
        let program = build_ancestor_program();
        let kb = solve(&program);
        
        // Check that we have adviser facts
        assert!(kb.iter().any(|a| a.pred_sym == "adviser" 
            && a.terms == vec![Term::sym("Andrew Rice"), Term::sym("Mistral Contrastin")]));
        
        // Check that we derived academicAncestor facts
        assert!(kb.iter().any(|a| a.pred_sym == "academicAncestor" 
            && a.terms == vec![Term::sym("Andrew Rice"), Term::sym("Mistral Contrastin")]));
        
        // Check transitive relationship: David Wheeler -> Andy Hopper -> Andrew Rice -> Mistral Contrastin
        assert!(kb.iter().any(|a| a.pred_sym == "academicAncestor" 
            && a.terms == vec![Term::sym("David Wheeler"), Term::sym("Mistral Contrastin")]));
    }

    #[test]
    fn test_query1() {
        let program = build_ancestor_program();
        let results = query("query1", &program);
        
        // Query 1 should find intermediate ancestors between Robin Milner and Mistral Contrastin
        // Robin Milner -> Alan Mycroft -> Dominic Orchard -> Mistral Contrastin
        assert!(!results.is_empty());
        
        // Check that Alan Mycroft is one of the intermediates
        assert!(results.iter().any(|sub| {
            sub.get(&Term::var("Intermediate")) == Some(&Term::sym("Alan Mycroft"))
        }));
        
        // Check that Dominic Orchard is one of the intermediates
        assert!(results.iter().any(|sub| {
            sub.get(&Term::var("Intermediate")) == Some(&Term::sym("Dominic Orchard"))
        }));
    }

    #[test]
    fn test_query2() {
        let program = build_ancestor_program();
        let results = query("query2", &program);
        
        // Query 2: Alan Turing is NOT an ancestor of Mistral Contrastin
        assert!(results.is_empty());
    }

    #[test]
    fn test_query3() {
        let program = build_ancestor_program();
        let results = query("query3", &program);
        
        // Query 3: David Wheeler IS an ancestor of Mistral Contrastin
        assert!(!results.is_empty());
    }

    #[test]
    #[should_panic(expected = "not range-restricted")]
    fn test_non_range_restricted_rule() {
        let program = vec![
            Rule {
                head: Atom {
                    pred_sym: "p".to_string(),
                    terms: vec![Term::var("X"), Term::var("Y")],
                },
                body: vec![Atom {
                    pred_sym: "q".to_string(),
                    terms: vec![Term::var("X")],
                }],
            },
        ];
        
        solve(&program);
    }

    #[test]
    fn test_empty_program() {
        let program: Program = vec![];
        let kb = solve(&program);
        assert!(kb.is_empty());
    }
}
