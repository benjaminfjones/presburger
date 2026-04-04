//! Implementation of Fourier-Motzkin Elimination
//! <https://en.wikipedia.org/wiki/Fourier%E2%80%93Motzkin_elimination>

use crate::{lin_rel::LinRel, lin_sys::LinSys};

/// FME Solver State
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FMEState {
    /// `check` has not been called
    UNKNOWN,
    /// system of (in)equalities is satisfiable
    SAT,
    /// system of (in)equalities is unsatisfiable
    UNSAT,
}

#[derive(Debug)]
pub struct FMESolver {
    state: FMEState,
    system: LinSys,
}

impl FMESolver {
    /// Create a fresh solver
    pub fn new() -> Self {
        Self {
            state: FMEState::UNKNOWN,
            system: LinSys::new(),
        }
    }

    /// Assert a new relation
    pub fn assert(&mut self, rel: LinRel) {
        self.system.add_relation(rel);
    }

    /// Reset the solver state and clear all assertions
    pub fn reset(&mut self) {
        self.state = FMEState::UNKNOWN;
        self.system.clear();
    }

    /// Check satisfiablility in the current state
    pub fn check(&mut self) -> FMEState {
        loop {
            // reduce system to inequalities and constant equalities
            self.system.eliminate_nontrivial_eqs();
            // remove the trivial equalities
            self.system.eliminate_trivial_eqs();
            // check there are no contradictory (in)equalities
            if self.system.has_trivial_contradiction() {
                return FMEState::UNSAT;
            }
            // If after equality removal there were only trivially SAT equalities then
            // the original system is SAT.
            if self.system.relations().is_empty() {
                return FMEState::SAT;
            }
            break;
        }
        todo!()
    }
}

#[cfg(test)]
mod test_fme {
    use super::*;

    use crate::fme::FMESolver;
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

    #[test]
    fn test_solver_check_only_equalities_2eq_sat() {
        let mut solver = FMESolver::new();
        solver.assert(eq!(0, 1, 1)); // x1 + x2 = 0
        solver.assert(eq!(0, 0, 1)); // x2 = 0
        assert_eq!(solver.check(), FMEState::SAT);
    }

    #[test]
    fn test_solver_check_only_equalities_2eq_unsat() {
        let mut solver = FMESolver::new();
        solver.assert(eq!(1, 1)); // 1 + x1 = 0
        solver.assert(eq!(2, 1)); // 2 + x1 = 0
        assert_eq!(solver.check(), FMEState::UNSAT);
    }
}
