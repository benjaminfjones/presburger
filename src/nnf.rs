/// Implement a reduction of the Presburger AST to Negation Normal Form (NNF).

use crate::ast::Pred;

/// Convert a Pred to NNF.
/// 
/// Traverse the internal graph structure of a predicate, applying reductions
/// along the way, including DeMorgan's laws, cancelation of double negations, and reduction of ==> and <==> to
/// logical AND, OR, and NOT.
///
/// Traversal is top-down.
pub fn to_nnf(p: Pred) -> Pred {
    let p = remove_impl(p);
    match p {
        Pred::Not(bp) => {
            let q: Pred = *(bp.clone());
            match q {
                // double negation: ~(~Q) -> to_nnf(Q)
                Pred::Not(bq) => to_nnf(*(bq.clone())),
                // DeMorgan: ~(Q1 /\\ Q2) -> to_nnf(~Q1) \\/ to_nnf(~Q2)
                Pred::And(bq1, bq2) => Pred::Or(Box::new(to_nnf(Pred::Not(bq1))), Box::new(to_nnf(Pred::Not(bq2)))),
                // DeMorgan: ~(Q1 /\\ Q2) -> to_nnf(~Q1) \\/ to_nnf(~Q2)
                Pred::Or(bq1, bq2) => Pred::And(Box::new(to_nnf(Pred::Not(bq1))), Box::new(to_nnf(Pred::Not(bq2)))),
                _ => Pred::Not(Box::new(to_nnf(q))),
            }
        }

        // descend and to_nnf /\\
        Pred::And(bp, bq) => {
            let p: Pred = *(bp.clone());
            let q: Pred = *(bq.clone());
            Pred::And(Box::new(to_nnf(p)), Box::new(to_nnf(q)))
        }

        // descend and to_nnf \\/
        Pred::Or(bp, bq) => {
            let p: Pred = *(bp.clone());
            let q: Pred = *(bq.clone());
            Pred::Or(Box::new(to_nnf(p)), Box::new(to_nnf(q)))
        }

        // descend and to_nnf Exists
        Pred::Exists(v, bp) => {
            let p: Pred = *(bp.clone());
            Pred::Exists(v, Box::new(to_nnf(p)))
        }

        // descend and to_nnf Forall
        Pred::Forall(v, bp) => {
            let p: Pred = *(bp.clone());
            Pred::Forall(v, Box::new(to_nnf(p)))
        }

        // base case
        aa @ Pred::Atom(_) => aa,
        
        p => panic!(format!("unexpected Pred: {:?}", p)),
    }
}

pub fn remove_impl(p: Pred) -> Pred {
    match p {
        Pred::Not(bp) => Pred::Not(Box::new(remove_impl(*(bp.clone())))),
        
        Pred::And(bp, bq) => {
            let p = remove_impl(*(bp.clone()));
            let q = remove_impl(*(bq.clone()));
            Pred::And(Box::new(p), Box::new(q))
        }

        Pred::Or(bp, bq) => {
            let p = remove_impl(*(bp.clone()));
            let q = remove_impl(*(bq.clone()));
            Pred::Or(Box::new(p), Box::new(q))
        }

        Pred::Impl(bp, bq) => {
            let p = remove_impl(*(bp.clone()));
            let q = remove_impl(*(bq.clone()));
            Pred::Or(Box::new(Pred::Not(Box::new(p))), Box::new(q))
        }

        Pred::Iff(bp, bq) => {
            let p = remove_impl(*(bp.clone()));
            let q = remove_impl(*(bq.clone()));
            let p2 = p.clone();
            let q2 = q.clone();
            let left = Pred::Or(Box::new(Pred::Not(Box::new(p))), Box::new(q));
            let right = Pred::Or(Box::new(Pred::Not(Box::new(q2))), Box::new(p2));
            Pred::And(Box::new(left), Box::new(right))
        }

        Pred::Exists(v, bp) => {
            let p = remove_impl(*(bp.clone()));
            Pred::Exists(v, Box::new(p))
        }

        Pred::Forall(v, bp) => {
            let p = remove_impl(*(bp.clone()));
            Pred::Forall(v, Box::new(p))
        }

        p @ Pred::Atom(_) => p,
    }
}

#[cfg(test)]
mod test {
    use crate::ast::*;
    use super::*;

    #[test]
    fn nnf_basic() {
        let pvar = Pred::Atom(Box::new(Atom::LogicalVar(Var("P".to_string()))));
        let pvar_ = pvar.clone();
        let p = Pred::Not(Box::new(Pred::Not(Box::new(pvar))));
        let n = to_nnf(p);

        // to_nnf(~(~P)) == P 
        assert_eq!(n, pvar_);
    }
}