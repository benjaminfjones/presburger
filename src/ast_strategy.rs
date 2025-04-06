//! Property-based testing strategies for [`ast::Formula`], [`ast::Term`], etc.

use crate::ast;
use crate::types::{BigRat, FromPrimitive};
use proptest::prelude::*;

pub fn arb_logic_var() -> impl Strategy<Value = ast::Var> {
    // use uppercase for logical vars
    "[A-Z]{1,3}".prop_map(|s| ast::Var::new(&s)).boxed()
}

pub fn arb_term(max_depth: u32, max_size: u32) -> impl Strategy<Value = ast::Term> {
    let leaf = prop_oneof![
        // use lowercase for term vars
        (any::<i64>(), "[a-z]{1,3}")
            .prop_map(|(a, x)| ast::Term::scalar_var(BigRat::from_integer(a.into()), &x)),
        any::<i64>().prop_map(|x| ast::Term::num(BigRat::from_i64(x).unwrap())),
    ];
    leaf.prop_recursive(max_depth, max_size, max_size, |inner| {
        let inner_copy = inner.clone();
        prop_oneof![(inner, inner_copy).prop_map(|(t1, t2)| ast::Term::tadd(t1, t2)),]
    })
}

pub fn arb_atom(max_depth: u32, max_size: u32) -> impl Strategy<Value = ast::Atom> {
    let new_depth = if max_depth > 0 { max_depth - 1 } else { 0 };
    prop_oneof![
        any::<bool>().prop_map(ast::Atom::truth),
        // use uppercase for logical vars
        "[A-Z]{1,3}".prop_map(|s| ast::Atom::var(&s)),
        (arb_term(new_depth, max_size), arb_term(new_depth, max_size))
            .prop_map(|(t1, t2)| ast::Atom::equality(t1, t2)),
        (arb_term(new_depth, max_size), arb_term(new_depth, max_size))
            .prop_map(|(t1, t2)| ast::Atom::less_eq(t1, t2)),
    ]
}

pub fn arb_formula(max_depth: u32, max_size: u32) -> impl Strategy<Value = ast::Formula> {
    let leaf = prop_oneof![arb_atom(max_depth, max_size).prop_map(ast::Formula::atom),];
    leaf.prop_recursive(max_depth, max_size, max_size, |inner| {
        prop_oneof![
            inner.clone().prop_map(ast::Formula::fnot),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Formula::and(p, q)),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Formula::or(p, q)),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Formula::implies(p, q)),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Formula::iff(p, q)),
            (arb_logic_var(), inner.clone()).prop_map(|(v, p)| ast::Formula::exists(v, p)),
            (arb_logic_var(), inner).prop_map(|(v, p)| ast::Formula::forall(v, p)),
        ]
    })
}
