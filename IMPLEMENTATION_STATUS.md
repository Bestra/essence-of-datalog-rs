# Datalog Implementation Status

This document compares the specification in `src/essence-of-datalog.txt` (a Haskell implementation) with the current Rust implementation in `src/main.rs` to identify what remains to be implemented.

## Overview

The specification describes a simple Datalog engine with the following components:
- Data structures for representing Datalog programs
- Core evaluation functions using substitution-based semantics
- Range restriction checking for safety
- A fixpoint solver
- Example programs and queries

## Data Structures

### Completed ✅
- `Term` enum with `Var` and `Sym` variants
- `Atom` struct with `predSym` and `terms` fields
- Type aliases for `Program`, `KnowledgeBase`, and `Substitution`

### Issues to Fix 🔧
- **`Rule` struct body field**: Currently defined as `body: Atom` but should be `body: Vec<Atom>` (a rule can have multiple atoms in its body)
- **Naming convention**: `predSym` should be `pred_sym` (Rust snake_case convention)
- **`Substitution` type**: Currently `HashMap<Term, Term>` but the spec indicates it should map variables to symbols only (not validated in type)

## Core Functions

### Implemented (Partial) ✅
1. **`substitute`** (Atom method)
   - Currently implemented but has issues:
     - Panics on missing substitution instead of leaving variable unchanged
     - Should gracefully handle unbound variables per the spec

### Not Implemented ❌

2. **`emptySubstitution`**
   - Purpose: Returns an empty substitution
   - Spec: `emptySubstitution = []` (Haskell)
   - Rust equivalent: Should return empty HashMap or use `HashMap::new()`

3. **`unify`**
   - Purpose: Unifies two atoms and returns a substitution if they match
   - Signature: `fn unify(atom1: &Atom, atom2: &Atom) -> Option<Substitution>`
   - Behavior:
     - Returns `None` if predicate symbols differ
     - Returns `None` if terms can't be unified
     - Assumes the second atom is ground (no variables)
     - Handles repeated variables correctly (e.g., `p(X,X)` with `p("a","b")` fails)

4. **`evalAtom`**
   - Purpose: Evaluates an atom against a knowledge base, returning all possible substitutions
   - Signature: `fn eval_atom(kb: &KnowledgeBase, atom: &Atom, substitutions: Vec<Substitution>) -> Vec<Substitution>`
   - Behavior:
     - For each input substitution, substitute it into the atom
     - Try to unify the substituted atom with each fact in the knowledge base
     - Extend the substitution with the unification result

5. **`walk`**
   - Purpose: Walks through a list of body atoms, accumulating substitutions
   - Signature: `fn walk(kb: &KnowledgeBase, body: &[Atom]) -> Vec<Substitution>`
   - Behavior:
     - Folds over the body atoms using `eval_atom`
     - Starts with empty substitution

6. **`evalRule`**
   - Purpose: Evaluates a single rule against a knowledge base
   - Signature: `fn eval_rule(kb: &KnowledgeBase, rule: &Rule) -> KnowledgeBase`
   - Behavior:
     - Walks the body to get all possible substitutions
     - Applies each substitution to the head
     - Returns the resulting atoms (new facts)

7. **`immediateConsequence`**
   - Purpose: Applies all rules to derive new facts in one iteration
   - Signature: `fn immediate_consequence(rules: &Program, kb: &KnowledgeBase) -> KnowledgeBase`
   - Behavior:
     - Evaluates each rule independently
     - Concatenates all newly derived facts
     - Combines with existing knowledge base
     - Removes duplicates (uses `nub` in Haskell, equivalent to deduplication)

8. **`isRangeRestricted`**
   - Purpose: Checks if a rule is range-restricted (safe)
   - Signature: `fn is_range_restricted(rule: &Rule) -> bool`
   - Behavior:
     - Returns true if all variables in the head appear in the body
     - This ensures domain independence and prevents infinite results

9. **`solve`**
   - Purpose: Main solver that computes the fixpoint
   - Signature: `fn solve(rules: &Program) -> KnowledgeBase`
   - Behavior:
     - First checks all rules are range-restricted
     - Starts with empty knowledge base
     - Repeatedly applies `immediate_consequence` until no new facts are derived
     - Returns the final knowledge base

## Example Programs and Queries

### Not Implemented ❌

1. **`ancestor` program**
   - The spec includes a complete example program encoding academic advisor relationships
   - Facts about advisors (7 facts)
   - Rules for computing academic ancestors (2 rules)
   - Three example queries

2. **`query` function**
   - Purpose: Convenience function to execute queries
   - Signature: `fn query(pred_sym: &str, pr: &Program) -> Vec<Substitution>`
   - Behavior:
     - Runs the solver on the program
     - Filters knowledge base for atoms matching the query predicate
     - Returns variable bindings for the query

## Implementation Notes

### Key Semantic Concepts to Address

1. **Range Restriction**
   - Critical for ensuring termination and domain independence
   - Must be checked before evaluation

2. **Fixpoint Computation**
   - Uses monotonic iteration until no new facts are derived
   - Termination guaranteed for range-restricted programs with finite initial facts

3. **Substitution Semantics**
   - The implementation uses substitution rather than relational algebra (joins)
   - This is simpler but potentially less efficient
   - Variables always map to symbols (constants), never to other variables

4. **Deduplication**
   - Essential for termination
   - The order of facts matters for the `nub` function behavior in Haskell
   - Rust should use `HashSet` or similar to ensure proper set semantics

### Differences from Haskell Implementation

1. **Type System**
   - Rust requires more explicit lifetime management
   - Haskell's pattern matching is more concise
   - Rust uses `Option` instead of `Maybe`

2. **Collections**
   - Haskell uses lists everywhere
   - Rust should consider using `Vec` for sequences and `HashSet` for deduplication

3. **Error Handling**
   - Haskell uses `error` for runtime errors
   - Rust should use proper error types or `panic!` for unrecoverable errors

## Testing Status

### Current Tests ✅
- `it_works`: Basic assertion test (placeholder)
- `subs`: Tests the `substitute` method

### Needed Tests ❌
- Unit tests for each core function
- Tests for the complete ancestor example program
- Tests for all three example queries from the spec
- Edge case tests (empty programs, non-range-restricted rules, etc.)

## Summary

### What's Complete
- Basic data structure scaffolding
- One function (`substitute`) partially implemented
- Basic test infrastructure

### What's Needed
- Fix `Rule.body` to be `Vec<Atom>`
- Fix `substitute` to handle missing variables gracefully
- Implement 8 core functions:
  - `unify`
  - `eval_atom`
  - `walk`
  - `eval_rule`
  - `immediate_consequence`
  - `is_range_restricted`
  - `solve`
  - `query` (convenience function)
- Implement the ancestor example program
- Add comprehensive tests
- Improve naming conventions (snake_case)

### Estimated Complexity
- **Core functions**: ~200-300 lines of Rust code
- **Example program**: ~50-80 lines
- **Tests**: ~100-150 lines
- **Total remaining**: ~350-530 lines of code

## Next Steps

1. Fix the `Rule` struct to have `body: Vec<Atom>`
2. Implement `unify` function (most complex function)
3. Implement evaluation functions in order: `eval_atom`, `walk`, `eval_rule`
4. Implement `immediate_consequence` and fixpoint computation
5. Implement `is_range_restricted` checker
6. Implement `solve` function
7. Add the ancestor example program
8. Implement and test the `query` function
9. Add comprehensive tests for all functionality
