use presburger::ast;
use presburger::ast_strategy::{arb_atom, arb_formula, arb_term};
use presburger::types::{BigRat, One};
use proptest::prelude::*;

// Make a bunch of large random formulas, convert to nnf, and profile memory usage
proptest! {
    #[test]
    fn make_vars(term_var in "[a-z]", logic_var in "[A-Z]") {
        let t = ast::Term::scalar_var(BigRat::one(), &term_var);
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
    fn make_formulas(formula in arb_formula(5, 10)) {
        println!("formula: {}", formula);
        assert_eq!(formula, formula);
    }
}
