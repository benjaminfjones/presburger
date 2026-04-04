//! Implementation of Fourier-Motzkin Elimination
//! <https://en.wikipedia.org/wiki/Fourier%E2%80%93Motzkin_elimination>

#[cfg(test)]
mod test_fme {
    use crate::lin_expr::{Bound, LinExpr, LinExprBound};
    use crate::lin_rel::LinRel;
    use crate::lin_sys::LinSys;
    use crate::{eq, le};
    use dashu::rbig;

    #[test]
    fn test_manual_fme() {
        let mut sys = LinSys::from_relations(vec![
            eq!(0, 1, -2, 0), //     x1 - 2 x2        = 0
            eq!(0, 0, 1, -3), //            x2 - 3 x3 = 0
            le!(5, 1, 1, 1),  // 5 + x1 +   x2 +   x3 <= 0
        ]);

        // reduce system to inequalities and constant equalities
        sys.eliminate_nontrivial_eqs();
        // remove the trivial equalities
        sys.eliminate_trivial_eqs();
        // check there are no contradictory (in)equalities
        assert!(!sys.has_trivial_contradiction());

        // subs x1 = 2 x2  ==>  x2 - 3 x3 = 0
        //                      5 + 3 x2 + x3 <= 0
        // ---------
        // subs x2 = 3 x3  ==>  5 + 10 x3 <= 0
        let remaining = sys.relations()[0].clone();
        assert_eq!(remaining, le!(5, 0, 0, 10));

        let var_to_isolate = 3;
        let upper_bound = remaining.compute_bound_from(var_to_isolate).unwrap();
        // x3 <= -5/10
        // x3 is unbounded
        // SAT

        assert_eq!(
            upper_bound,
            LinExprBound {
                i: 3,
                bound: Bound::Upper,
                expr: LinExpr::new(vec![rbig!(-5 / 10), rbig!(0), rbig!(0), rbig!(0)]).unwrap(),
            }
        )
    }
}
