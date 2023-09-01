# dot plan

## Update 2022-03-11

      - define a common IR that supports integer and rational linear
        expressions and relations
      - implement an LQA solver using FME with arbitrary precision rationals,
        using [rug](https://crates.io/crates/rug) for scalars
      - implement an LIA solver using the Omega Test
      - add pre-processing and translation passes to go from the
        quantifier-free fragment of the front-end AST to the IR for FME/Omega
        problems
      - implement quantifier elimination
      - identify benchmarks and compare against Coq-Omega/LIA
