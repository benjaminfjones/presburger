/// Implement a reduction of the Presburger AST to Negation Normal Form (NNF).
use crate::ast::Formula;

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
                Formula::And(bq1, bq2) => Formula::Or(
                    Box::new(to_nnf(Formula::Not(bq1))),
                    Box::new(to_nnf(Formula::Not(bq2))),
                ),
                // DeMorgan: ~(Q1 /\\ Q2) -> to_nnf(~Q1) \\/ to_nnf(~Q2)
                Formula::Or(bq1, bq2) => Formula::And(
                    Box::new(to_nnf(Formula::Not(bq1))),
                    Box::new(to_nnf(Formula::Not(bq2))),
                ),
                q => Formula::Not(Box::new(to_nnf(q))),
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
        p => panic!(format!("unexpected Formula: {:?}", p)),
    }
}

/// Verify that a formula is in NNF.
///
/// Used for testing `to_nnf`.
pub fn verify_nnf(p: Formula) -> bool {
    match p {
        // NOT can only appear applied to an Atom
        Formula::Not(bp) => match *bp {
            Formula::Atom(_) => true,
            _ => false,
        },
        Formula::And(bp, bq) => verify_nnf(*bp) && verify_nnf(*bq),
        Formula::Or(bp, bq) => verify_nnf(*bp) && verify_nnf(*bq),
        // Impl cannot appear
        Formula::Impl(_, _) => false,
        // Iff cannot appear
        Formula::Iff(_, _) => false,
        Formula::Exists(_, bp) => verify_nnf(*bp),
        Formula::Forall(_, bp) => verify_nnf(*bp),
        Formula::Atom(_) => true,
    }
}

/// Recursively remove ==> and <==> from the formula, replacing them by equivalent logic in terms of NOT, AND, and OR.
pub fn remove_impl(p: Formula) -> Formula {
    match p {
        Formula::Not(bp) => Formula::Not(Box::new(remove_impl(*bp))),
        Formula::And(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            Formula::And(Box::new(p), Box::new(q))
        }

        Formula::Or(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            Formula::Or(Box::new(p), Box::new(q))
        }

        Formula::Impl(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            Formula::Or(Box::new(Formula::Not(Box::new(p))), Box::new(q))
        }

        Formula::Iff(bp, bq) => {
            let p = remove_impl(*bp);
            let q = remove_impl(*bq);
            let p2 = p.clone();
            let q2 = q.clone();
            let left = Formula::Or(Box::new(Formula::Not(Box::new(p))), Box::new(q));
            let right = Formula::Or(Box::new(Formula::Not(Box::new(q2))), Box::new(p2));
            Formula::And(Box::new(left), Box::new(right))
        }

        Formula::Exists(v, bp) => {
            let p = remove_impl(*bp);
            Formula::Exists(v, Box::new(p))
        }

        Formula::Forall(v, bp) => {
            let p = remove_impl(*bp);
            Formula::Forall(v, Box::new(p))
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
        assert!(verify_nnf(n));
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
        assert!(verify_nnf(n));
        assert!(verify_nnf(expected));
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
        assert!(verify_nnf(n));
        assert!(verify_nnf(expected));
    }
}
