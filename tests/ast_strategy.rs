use proptest::prelude::*;
use presburger::ast;

pub fn arb_term_var() -> impl Strategy<Value = ast::Var> {
    "[a-z]{1,3}".prop_map(|s| ast::Var::new(&s)).boxed()
}

pub fn arb_logic_var() -> impl Strategy<Value = ast::Var> {
    "[A-Z]{1,3}".prop_map(|s| ast::Var::new(&s)).boxed()
}

pub fn arb_term(max_depth: u32, max_size: u32) -> impl Strategy<Value = ast::Term> {
    let leaf = prop_oneof![
        "[a-z]{1,3}".prop_map(|s| ast::Term::var(&s)),
        any::<i64>().prop_map(ast::Term::num),
    ];
    leaf.prop_recursive(
        max_depth,
        max_size,
        max_size,
        |inner| prop_oneof![
            (inner.clone(), inner.clone()).prop_map(|(t1, t2)| ast::Term::add(t1, t2)),
        ])
}

pub fn arb_atom(max_depth: u32, max_size: u32) -> impl Strategy<Value = ast::Atom> {
    let new_depth = if max_depth > 0 { max_depth-1 } else { 0 };
    prop_oneof![
        any::<bool>().prop_map(ast::Atom::truth),
        "[A-Z]{1,3}".prop_map(|s| ast::Atom::var(&s)),
        (arb_term(new_depth, max_size), arb_term(new_depth, max_size)).prop_map(
            |(t1, t2)| ast::Atom::equality(t1, t2)),
        (arb_term(new_depth, max_size), arb_term(new_depth, max_size)).prop_map(
            |(t1, t2)| ast::Atom::less_eq(t1, t2)),
    ]
}

pub fn arb_pred(max_depth: u32, max_size: u32) -> impl Strategy<Value = ast::Pred> {
    let leaf = prop_oneof![
        arb_atom(max_depth, max_size).prop_map(ast::Pred::atom),
    ];
    leaf.prop_recursive(
        max_depth,
        max_size,
        max_size,
        |inner| prop_oneof![
            inner.clone().prop_map(|p| ast::Pred::not(p)),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Pred::and(p, q)),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Pred::or(p, q)),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Pred::implies(p, q)),
            (inner.clone(), inner.clone()).prop_map(|(p, q)| ast::Pred::iff(p, q)),
            (arb_logic_var(), inner.clone()).prop_map(|(v, p)| ast::Pred::exists(v, p)),
            (arb_logic_var(), inner.clone()).prop_map(|(v, p)| ast::Pred::forall(v, p)),
        ])
}
proptest! {
    #[test]
    fn make_vars(term_var in "[a-z]", logic_var in "[A-Z]") {
        let t = ast::Term::var(&term_var);
        let l = ast::Atom::var(&logic_var);
        println!("term var: {}, logic var: {}", t, l);
        assert_eq!(t, t);
        assert_eq!(l, l);
    }

    #[test]
    fn make_terms(term in arb_term(5, 10)) {
        println!("term: {}", term);
        assert_eq!(term, term);
    }

    #[test]
    fn make_atoms(atom in arb_atom(5, 10)) {
        println!("atom: {}", atom);
        assert_eq!(atom, atom);
    }

    #[test]
    fn make_preds(pred in arb_pred(5, 10)) {
        println!("pred: {}", pred);
        assert_eq!(pred, pred);
    }
}