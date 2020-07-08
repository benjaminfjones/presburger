# Presberger

![Rust](https://github.com/benjaminfjones/presburger/workflows/Rust/badge.svg)

This project will *eventually* be a library for Presburger Arithmetic [1],
including symbolic manipulation of formulas and a decision procedure for the
full theory based on quantifies elimination.

The decision to use Rust for implementation was first to experiment with Rust
and second to try and produce a highly efficient decision procedure (e.g. one
that outperforms Omega).

[1] https://en.wikipedia.org/wiki/Presburger_arithmetic

## Progress

  * 2020-07-07 -- got out the dust mop
  * 2019-11-02 -- switched parser generators to lalrpop which seems better
    suited to this task. There are unit tests now that verify parsing for a
    range of presberger expressions.

  * 2019-10-23 -- setup Rust skeleton, experimenting with different parser
    generator options
