use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Term {
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

#[derive(Debug, PartialEq, Eq)]
struct Atom {
    predSym: String,
    terms: Vec<Term>,
}

#[derive(Debug)]
struct Rule {
    head: Atom,
    body: Atom,
}

type Program = Vec<Rule>;
type KnowledgeBase = Vec<Atom>;
type Substitution = HashMap<Term, Term>;

impl Atom {
    fn substitute(&self, substitution: Substitution) -> Atom {
        Atom {
            predSym: self.predSym.clone(),
            terms: self
                .terms
                .clone()
                .into_iter()
                .map(|t| match t {
                    Term::Sym(s) => Term::Sym(s.clone()),
                    Term::Var(_) => match substitution.get(&t) {
                        Some(term) => term.clone(),
                        None => panic!("oops"),
                    },
                })
                .collect(),
        }
    }
}

// fn unify(atom: Atom, atom2: Atom) -> Option<Substitution> {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn subs() {
        let a = Atom {
            predSym: "adviser".to_string(),
            terms: vec![Term::var("X"), Term::var("Y")],
        };
        let mut subs = HashMap::new();
        subs.insert(Term::var("X"), Term::var("Y"));
        subs.insert(Term::var("Y"), Term::sym("Dude"));
        let new_a = a.substitute(subs);
        println!("new atom {:?}", new_a);
        assert_eq!(new_a.terms, vec![Term::var("Y"), Term::sym("Dude")])
    }
}
