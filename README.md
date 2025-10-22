# Essence of Datalog - Rust Implementation

A Rust port of a simple Datalog engine based on the tutorial "The Essence of Datalog" (see `src/essence-of-datalog.txt`).

## About

This project is a work-in-progress implementation of a minimal Datalog engine in Rust, following the Haskell implementation described in the included tutorial document. Datalog is a declarative logic programming language that's simpler than Prolog and can be thought of as a principled SQL.

## Project Structure

- `src/essence-of-datalog.txt` - The original tutorial/specification document explaining Datalog semantics and showing a complete Haskell implementation
- `src/main.rs` - The Rust implementation (currently in progress)
- `IMPLEMENTATION_STATUS.md` - Detailed documentation of what's been implemented and what remains to be done

## Current Status

This is an early-stage implementation. Currently implemented:
- Basic data structures (`Term`, `Atom`, `Rule`)
- Partial substitution function

See `IMPLEMENTATION_STATUS.md` for a comprehensive breakdown of:
- What's complete
- What needs to be implemented
- Known issues to fix
- Implementation notes and semantic considerations

## What is Datalog?

From the tutorial:

> Datalog is arguably the simplest logic programming language there is. Depending on your background, you can see it as a principled SQL or a Prolog with manners.

Key features:
- Logic programming with rules and facts
- Recursive queries
- Bidirectional queries (no inherent inputs/outputs)
- Guaranteed termination (not Turing-complete)

## Example Program

The classic example is computing academic ancestry relationships:

```datalog
adviser("Andrew Rice", "Mistral Contrastin").
adviser("Andy Hopper", "Andrew Rice").
adviser("David Wheeler", "Andy Hopper").

academicAncestor(X,Y) :- adviser(X,Y).
academicAncestor(X,Z) :- adviser(X,Y), academicAncestor(Y,Z).

?- academicAncestor("David Wheeler", "Mistral Contrastin").
```

## Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the program
cargo run
```

## Resources

- Original tutorial: `src/essence-of-datalog.txt`
- Implementation status: `IMPLEMENTATION_STATUS.md`
- For more on Datalog:
  - [Soufflé](https://souffle-lang.github.io/) - A modern, fast Datalog variant
  - "Foundations of Databases" by Abiteboul, Hull, and Vianu (Chapters 12-15)

## License

See the original tutorial document for attribution and acknowledgements.
