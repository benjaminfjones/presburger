//! Implementation of Fourier-Motzkin Elimination
//! <https://en.wikipedia.org/wiki/Fourier%E2%80%93Motzkin_elimination>

use itertools::partition;

use crate::lin_expr::{Bound, LinExprBound};
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
        // reduce system to inequalities and constant equalities
        self.system.eliminate_nontrivial_eqs();

        loop {
            println!(
                "{} le relations at start of loop",
                self.system.num_relations()
            );
            // remove the trivial relations
            self.system.eliminate_trivial_relations();

            println!(
                "{} le relations after removing trivial ones",
                self.system.num_relations()
            );
            // If after equality removal there were only trivially SAT equalities then
            // the original system is SAT.
            if self.system.relations().is_empty() {
                return FMEState::SAT;
            }
            // check there are no contradictory (in)equalities
            if self.system.has_trivial_contradiction() {
                return FMEState::UNSAT;
            }

            // At this point, there is guaranteed to be at least one non-constant inequality
            let i = self.system.find_isolatable_variable_in_le().unwrap();
            println!("isolating variable {i}");
            let relations = self.system.relations();
            let mut computed_bounds: Vec<LinExprBound> = relations
                .iter()
                .map(|r| r.compute_bound_from(i))
                .filter(|cb| cb.is_some())
                .map(|cb| cb.unwrap())
                .collect();
            let split_index = partition(&mut computed_bounds, |b| b.bound == Bound::Lower);
            // computed_bounds = [lower1, ... lowerN, upper1, ... upperM]
            //                                      ^-- split_index
            println!(
                "{} lower bounds and {} upper bounds",
                split_index,
                computed_bounds.len() - split_index
            );

            // Remove all relations from the system that have non-zero a_i;
            // these are replaced by the lower,upper bound pairs below
            let mut to_remove = Vec::new();
            for (j, r) in self.system.relations().iter().enumerate() {
                if r.lhs().supported(i) {
                    to_remove.push(j);
                }
            }
            for j in to_remove {
                self.system.remove_relation(j);
            }

            // Form all pairs of <= relations: lower_bound_expr <= upper_bound_expr
            for i in 0..split_index {
                for j in split_index..computed_bounds.len() {
                    self.system.add_relation(LinRel::le_from_lhs_rhs(
                        &computed_bounds[i].expr,
                        &computed_bounds[j].expr,
                    ));
                }
            }
        } // end of FME loop
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
        sys.eliminate_trivial_relations();
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
    fn test_solver_check_only_equalities_4eq_sat() {
        let mut solver = FMESolver::new();
        // system can be transformed by automorphism into one with upper triangular matrix => SAT
        solver.assert(eq!(0, 0, 1, 7, 0));
        solver.assert(eq!(0, 0, 0, 0, 15));
        solver.assert(eq!(0, 0, 0, 11, 13));
        solver.assert(eq!(0, 2, 0, 3, 5));
        assert_eq!(solver.check(), FMEState::SAT);
    }

    #[test]
    fn test_solver_check_only_equalities_2eq_1trivleq_sat() {
        let mut solver = FMESolver::new();
        solver.assert(eq!(0, 1, 1)); // x1 + x2 = 0
        solver.assert(eq!(0, 0, 1)); // x2 = 0
        solver.assert(le!(-1, 0, 0)); // -1 <= 0
        assert_eq!(solver.check(), FMEState::SAT);
    }

    #[test]
    fn test_solver_check_only_equalities_2eq_1trivleq_unsat() {
        let mut solver = FMESolver::new();
        solver.assert(eq!(0, 1, 1)); // x1 + x2 = 0
        solver.assert(eq!(0, 0, 1)); // x2 = 0
        solver.assert(le!(1, 0, 0)); // 1 <= 0 -> UNSAT
        assert_eq!(solver.check(), FMEState::UNSAT);
    }

    #[test]
    fn test_solver_check_only_equalities_2eq_unsat() {
        let mut solver = FMESolver::new();
        solver.assert(eq!(1, 1)); // 1 + x1 = 0
        solver.assert(eq!(2, 1)); // 2 + x1 = 0
        assert_eq!(solver.check(), FMEState::UNSAT);
    }

    #[test]
    fn test_solver_check_one_le() {
        let mut solver = FMESolver::new();
        solver.assert(eq!(1, 1, 1)); // 1  + x1 + x2 = 0
        solver.assert(le!(1, 1, 0)); // 1  + x1 <= 0
        // x1 = -1 -x2
        // ==> 1 + -1 - x2 <= 0 => 0 <= x2 unbounded
        // In this case after the first round there are only upper bounds, no lower bounds
        // thus the system has an empty set of relations => SAT
        assert_eq!(solver.check(), FMEState::SAT);
    }

    // Test from https://en.wikipedia.org/wiki/Fourier%E2%80%93Motzkin_elimination
    #[test]
    fn test_solver_check_wikipedia() {
        let mut solver = FMESolver::new();
        solver.assert(le!(-10, 2, -5, 4));
        solver.assert(le!(-9, 3, -6, 3));
        solver.assert(le!(7, -1, 5, -2));
        solver.assert(le!(-12, -3, 2, 6));
        assert_eq!(solver.check(), FMEState::SAT);
    }

    // Test from Decision Procedures, 2nd ed.
    // 0 ≤ x ≤ 1, 0 ≤ y ≤ 1, 3/4 <= z <= 1
    // ----
    // -x <= 0
    // -1 + x <= 0
    // -y <= 0
    // -1 + y <= 0
    // 3/4 - z <= 0
    // -1 + x <= 0
    #[test]
    fn test_solver_check_dec_proc_1_sat() {
        let mut solver = FMESolver::new();
        solver.assert(le!(0, -1, 0, 0));
        solver.assert(le!(-1, 1, 0, 0));
        solver.assert(le!(0, 0, -1, 0));
        solver.assert(le!(-1, 0, 1, 0));
        solver.assert(le!(rbig!(3 / 4), rbig!(0), rbig!(0), rbig!(1)));
        solver.assert(le!(-1, 0, 0, 1));
        assert_eq!(solver.check(), FMEState::SAT);
    }

    // Test from Decision Procedures, 2nd ed.
    #[test]
    fn test_solver_check_dec_proc_2_unsat() {
        let mut solver = FMESolver::new();
        solver.assert(le!(0, 1, -1, 0));
        solver.assert(le!(0, 1, 0, -1));
        solver.assert(le!(0, -1, 1, 2));
        solver.assert(le!(1, 0, 0, -1));
        assert_eq!(solver.check(), FMEState::UNSAT);
    }
}
