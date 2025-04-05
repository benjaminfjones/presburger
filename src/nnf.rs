/// Implement a reduction of the Presburger AST to Negation Normal Form (NNF).
use crate::ast::Formula;
#[allow(unused_imports)]
use crate::ast_strategy;
use proptest::prelude::*;

/// Convert a Formula to NNF.
///
/// Traverse the internal graph structure of a predicate, applying reductions
/// along the way, including DeMorgan's laws, cancelation of double negations, and reduction of ==> and <==> to
/// logical AND, OR, and NOT.
///
/// Traversal is top-down.
pub fn to_nnf(p: Formula) -> Formula {
    let p = remove_impl(p);
    match p {
        Formula::Not(bp) => {
            match *bp {
                // double negation: ~(~Q) -> to_nnf(Q)
                Formula::Not(bq) => to_nnf(*bq),
                // DeMorgan: ~(Q1 /\\ Q2) -> to_nnf(~Q1) \\/ to_nnf(~Q2)
                Formula::And(bq1, bq2) => {
                    Formula::or(to_nnf(Formula::fnot(*bq1)), to_nnf(Formula::fnot(*bq2)))
                }
                // DeMorgan: ~(Q1 /\\ Q2) -> to_nnf(~Q1) \\/ to_nnf(~Q2)
                Formula::Or(bq1, bq2) => {
                    Formula::and(to_nnf(Formula::fnot(*bq1)), to_nnf(Formula::fnot(*bq2)))
                }
                // ~∃x. P(x) <==> ∀x. ~P(x)
                Formula::Exists(v, bp) => Formula::forall(v, to_nnf(Formula::fnot(*bp))),
                // ~∀x. P(x) <==> ∃x. ~P(x)
                Formula::Forall(v, bp) => Formula::exists(v, to_nnf(Formula::fnot(*bp))),
                a @ Formula::Atom(_) => Formula::fnot(a),
                _ => panic!("unpexpected Impl or Iff in formula after remove_impl"),
            }
        }

        // descend and to_nnf /\\
        Formula::And(bp, bq) => Formula::And(Box::new(to_nnf(*bp)), Box::new(to_nnf(*bq))),

        // descend and to_nnf \\/
        Formula::Or(bp, bq) => Formula::Or(Box::new(to_nnf(*bp)), Box::new(to_nnf(*bq))),

        // descend and to_nnf Exists
        Formula::Exists(v, bp) => Formula::Exists(v, Box::new(to_nnf(*bp))),

        // descend and to_nnf Forall
        Formula::Forall(v, bp) => Formula::Forall(v, Box::new(to_nnf(*bp))),

        // base case
        aa @ Formula::Atom(_) => aa,
        p => panic!("unexpected Formula: {:?}", p),
    }
}

/// Verify that a formula is in NNF.
///
/// Used for testing `to_nnf`.
pub fn verify_nnf(p: &Formula) -> bool {
    match p {
        // NOT can only appear applied to an Atom
        Formula::Not(bp) => matches!(**bp, Formula::Atom(_)),
        Formula::And(bp, bq) => verify_nnf(bp) && verify_nnf(bq),
        Formula::Or(bp, bq) => verify_nnf(bp) && verify_nnf(bq),
        // Impl cannot appear
        Formula::Impl(_, _) => false,
        // Iff cannot appear
        Formula::Iff(_, _) => false,
        Formula::Exists(_, bp) => verify_nnf(bp),
        Formula::Forall(_, bp) => verify_nnf(bp),
        Formula::Atom(_) => true,
    }
}

/// Recursively remove ==> and <==> from the formula, replacing them by equivalent logic in terms of NOT, AND, and OR.
pub fn remove_impl(p: Formula) -> Formula {
    match p {
        Formula::Not(bp) => Formula::fnot(remove_impl(*bp)),
        Formula::And(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            Formula::and(p, q)
        }

        Formula::Or(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            Formula::or(p, q)
        }

        Formula::Impl(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            Formula::or(Formula::fnot(p), q)
        }

        Formula::Iff(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            let p2 = p.clone();
            let q2 = q.clone();
            let left = Formula::or(Formula::fnot(p), q);
            let right = Formula::or(Formula::fnot(q2), p2);
            Formula::and(left, right)
        }

        Formula::Exists(v, bp) => {
            let p = remove_impl(*bp);
            Formula::exists(v, p)
        }

        Formula::Forall(v, bp) => {
            let p = remove_impl(*bp);
            Formula::forall(v, p)
        }

        p @ Formula::Atom(_) => p,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::*;

    #[test]
    fn nnf_double_not() {
        let pvar = Formula::atom(Atom::var("P"));
        let p = Formula::fnot(Formula::fnot(pvar.clone()));
        let n = to_nnf(p);

        // to_nnf(~(~P)) == P
        assert_eq!(n, pvar);
        assert!(verify_nnf(&n));
    }

    #[test]
    fn nnf_not_and() {
        let pvar = Formula::atom(Atom::var("P"));
        let qvar = Formula::atom(Atom::var("Q"));
        let p = Formula::fnot(Formula::and(pvar.clone(), qvar.clone()));
        let n = to_nnf(p);
        let expected = Formula::or(Formula::fnot(pvar), Formula::fnot(qvar));

        // to_nnf(~(P /\\ Q)) equals ~P \\/ ~Q
        assert_eq!(n, expected);
        assert!(verify_nnf(&n));
        assert!(verify_nnf(&expected));
    }

    #[test]
    fn nnf_impl() {
        let pvar = Formula::atom(Atom::var("P"));
        let qvar = Formula::atom(Atom::var("Q"));
        let i = Formula::implies(pvar.clone(), qvar.clone());
        let n = to_nnf(i);
        let expected = Formula::or(Formula::fnot(pvar), qvar);

        // to_nnf(P ==> Q) equals ~P \\/ Q
        assert_eq!(n, expected);
        assert!(verify_nnf(&n));
        assert!(verify_nnf(&expected));

        // regression test generated by proptest
        //
        // ((∃A. false) ==> false) is equivalent to ...
        // (~(∃A. false) \\/ false) ...
        // (∀A. ~false) \\/ false)
        let formula = Formula::implies(
            Formula::exists(Var::new("A"), Formula::atom(Atom::truth(false))),
            Formula::atom(Atom::truth(false)),
        );
        let n = to_nnf(formula);
        let expected = Formula::or(
            Formula::forall(
                Var::new("A"),
                Formula::fnot(Formula::atom(Atom::truth(false))),
            ),
            Formula::atom(Atom::truth(false)),
        );
        assert!(verify_nnf(&n));
        assert_eq!(n, expected);
    }
}

proptest! {
    /// Generate random formulas, convert to nnf, and verify they are in NNF
    #[test]
    fn nnf_arb_formula(formula in ast_strategy::arb_formula(6, 20)) {
        let n = to_nnf(formula);
        assert!(verify_nnf(&n));
    }
}
