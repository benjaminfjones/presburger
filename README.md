# Presberger

![Rust](https://github.com/benjaminfjones/presburger/workflows/Rust/badge.svg)

This project is a work in progress.

This project will *eventually* be a library for Presburger Arithmetic [1],
including symbolic manipulation of formulas and a decision procedure for the
full theory based on quantifier elimination.

The decision to use Rust for implementation was first to experiment with Rust
and second to try and produce a highly efficient decision procedure for
not just Linear Integer Arithmetic, but the full Presburger Arithmetic. The
lofty goal is to outperform Coq Tactics Omega [2] & LIA [3] for some reasonably
common set of benchmark problems.

[1] https://en.wikipedia.org/wiki/Presburger_arithmetic
[2] https://coq.github.io/doc/v8.13/refman/addendum/omega.html
[3] https://coq.github.io/doc/v8.13/refman/addendum/micromega.html#coq:tacn.lia


## Progress

  * 2022-03-10 -- Work in progress on a specific Fourier-Motzkin Elimination
    (FME) implementation. Current plan is tracked in `PLAN.md`.
  * 2020-07-26 -- Ironed out and tested (proptest) `to_nnf` for AST formulas,
    still need to do some memory profiling before committing to the current AST
    representation.
  * 2020-07-07 -- Got out the dust mop
  * 2019-11-02 -- Switched parser generators to lalrpop which seems better
    suited to this task. There are unit tests now that verify parsing for a
    range of presberger expressions.
  * 2019-10-23 -- Setup Rust skeleton, experimenting with different parser
    generator options
