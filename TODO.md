# TODO: Implementation Checklist

Quick reference for what needs to be implemented. See `IMPLEMENTATION_STATUS.md` for detailed explanations.

## Data Structure Fixes

- [ ] Fix `Rule` struct: change `body: Atom` to `body: Vec<Atom>`
- [ ] Fix `substitute` method: handle missing variables gracefully instead of panicking
- [ ] Consider using `HashSet` for `KnowledgeBase` instead of `Vec<Atom>` for better deduplication

## Core Functions (in suggested implementation order)

1. [ ] `fn empty_substitution() -> Substitution` - Return empty HashMap
2. [ ] `fn unify(atom1: &Atom, atom2: &Atom) -> Option<Substitution>` - Most complex function
3. [ ] `fn eval_atom(kb: &KnowledgeBase, atom: &Atom, substitutions: Vec<Substitution>) -> Vec<Substitution>`
4. [ ] `fn walk(kb: &KnowledgeBase, body: &[Atom]) -> Vec<Substitution>`
5. [ ] `fn eval_rule(kb: &KnowledgeBase, rule: &Rule) -> KnowledgeBase`
6. [ ] `fn is_range_restricted(rule: &Rule) -> bool`
7. [ ] `fn immediate_consequence(rules: &Program, kb: &KnowledgeBase) -> KnowledgeBase`
8. [ ] `fn solve(rules: &Program) -> KnowledgeBase` - Main fixpoint solver

## Example Programs & Utilities

- [ ] Implement the `ancestor` program with:
  - 7 adviser facts
  - 2 academicAncestor rules
  - 3 example queries
- [ ] `fn query(pred_sym: &str, pr: &Program) -> Vec<Substitution>` - Convenience query function

## Testing

- [ ] Unit tests for `unify` (including edge cases like repeated variables)
- [ ] Unit tests for `eval_atom`
- [ ] Unit tests for `walk`
- [ ] Unit tests for `eval_rule`
- [ ] Unit tests for `is_range_restricted`
- [ ] Integration test: full ancestor program with all 3 queries
- [ ] Test for non-range-restricted rules (should error)
- [ ] Test for empty programs
- [ ] Test fixpoint convergence

## Code Quality

- [ ] Follow Rust naming conventions (snake_case for functions/fields)
- [ ] Add proper documentation comments (`///`)
- [ ] Consider error handling strategy (proper error types vs panic!)
- [ ] Add `#[derive(Clone)]` where needed for ergonomics
- [ ] Implement `Display` trait for nice printing of Atoms, Rules, etc.

## Estimated Lines of Code

- Core functions: ~200-300 lines
- Example program: ~50-80 lines  
- Tests: ~100-150 lines
- **Total: ~350-530 lines**

## Key Implementation Notes

### unify() is the most complex
- Must handle predicate symbol matching
- Must handle symbol-to-symbol equality
- Must handle variable-to-symbol binding
- Must handle repeated variables (e.g., `p(X,X)` with `p("a","b")` must fail)
- Assumes second atom is ground (no variables)

### solve() fixpoint algorithm
- Start with empty knowledge base
- Repeatedly call `immediate_consequence` until stable
- Check range restriction before starting
- Termination guaranteed for valid programs

### Range restriction is critical
- Ensures domain independence
- Prevents infinite results
- Must check: all head variables appear in body
