# preberger

This project will *eventually* be a library for Presburger Arithmetic [1],
including symbolic manipulation of formulas and a decision procedure for the
full theory based on quantifies elimination.

The decision to use Rust for implementation was first to experiment with Rust
and second to try and produce a highly efficient decision procedure (e.g. one
that outperforms Omega).

[1] https://en.wikipedia.org/wiki/Presburger_arithmetic

## Progress

  * 2019-10-23 -- setup Rust skeleton, experimenting with different parser
    generator options
