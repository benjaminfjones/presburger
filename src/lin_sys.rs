//! Implementation of linear systems: a collection of linear relations

use crate::lin_rel::LinRel;

/// A system of linear relations
#[derive(Debug, PartialEq, Eq)]
pub struct LinSys {
    relations: Vec<LinRel>,
}

impl Default for LinSys {
    fn default() -> Self {
        Self::new()
    }
}

impl LinSys {
    /// Create a new empty linear system
    pub fn new() -> Self {
        Self {
            relations: Vec::new(),
        }
    }

    /// Create a linear system from a vector of linear relations
    pub fn from_relations(relations: Vec<LinRel>) -> Self {
        Self { relations }
    }

    /// Get the number of relations in the system
    pub fn len(&self) -> usize {
        self.relations.len()
    }

    /// Check if the system is empty
    pub fn is_empty(&self) -> bool {
        self.relations.is_empty()
    }

    /// Get a reference to the relations vector
    pub fn relations(&self) -> &[LinRel] {
        &self.relations
    }

    /// Add a linear relation to the system
    pub fn add_relation(&mut self, relation: LinRel) {
        self.relations.push(relation);
    }

    /// Remove a linear relation from the system by index
    pub fn remove_relation(&mut self, index: usize) -> Option<LinRel> {
        if index < self.relations.len() {
            Some(self.relations.remove(index))
        } else {
            None
        }
    }

    /// Clear all relations from the system
    pub fn clear(&mut self) {
        self.relations.clear();
    }

    /// Reduce, if possible, the linear system by substituting some eligible equality in the system
    /// into every relation.
    ///
    /// Returns `true` if an equality was eliminated and `false` if there are no further reductions
    /// possible.
    pub fn reduce_eqs(&mut self) -> bool {
        // Find the first equality that can be used for substitution
        let sub_index = self
            .relations
            .iter()
            .position(|rel| rel.is_subs().is_some());

        if let Some(index) = sub_index {
            // Get the substitution equality
            let subs_eq = self.relations.remove(index);
            let sub_var = subs_eq.is_subs().unwrap();

            // Substitute this equality into all remaining relations
            for i in 0..self.relations.len() {
                if let Ok(substituted) = self.relations[i].clone().subs(sub_var, &subs_eq) {
                    self.relations[i] = substituted;
                }
            }

            true
        } else {
            false
        }
    }

    /// Call `reduce_eqs()` repeatedly until no more equality reductions are possible.
    /// If equalities still remain they are guaranteed to be equalities between constants.
    pub fn eliminate_nontrivial_eqs(&mut self) {
        while self.reduce_eqs() {}
    }

    /// Filter out trivial constant (in)equalities from the system. See `LinRel::is_trivial().`
    pub fn eliminate_trivial_eqs(&mut self) {
        loop {
            let next = self.relations.iter().position(|r| r.is_trivial());
            match next {
                None => break,
                Some(i) => {
                    // could use `swap_remove` here to be more efficient, but it changes the order
                    // of elements and makes testing harder
                    self.relations.remove(i);
                }
            }
        }
    }

    /// Determine if `self` contains a trivial contradiction between constants, e.g. 1 = 0, or
    /// 2 <= 0.
    pub fn has_trivial_contradiction(&self) -> bool {
        self.relations.iter().any(|r| r.is_trivial_contradiction())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lin_expr::LinExpr;
    use crate::lin_rel::LinRel;

    #[test]
    fn test_new_empty_system() {
        let system = LinSys::new();
        assert!(system.is_empty());
        assert_eq!(system.len(), 0);
    }

    #[test]
    fn test_from_relations() {
        let eq1 = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2]).unwrap());
        let eq2 = LinRel::mk_le(LinExpr::new(vec![0, 3, 4]).unwrap());
        let system = LinSys::from_relations(vec![eq1.clone(), eq2.clone()]);
        assert_eq!(system.len(), 2);
        assert_eq!(system.relations(), &[eq1, eq2]);
    }

    #[test]
    fn test_add_and_remove_relation() {
        let mut system = LinSys::new();
        let eq1 = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2]).unwrap());
        let eq2 = LinRel::mk_le(LinExpr::new(vec![0, 3, 4]).unwrap());

        system.add_relation(eq1.clone());
        assert_eq!(system.len(), 1);

        system.add_relation(eq2.clone());
        assert_eq!(system.len(), 2);

        let removed = system.remove_relation(0);
        assert_eq!(removed, Some(eq1));
        assert_eq!(system.len(), 1);
        assert_eq!(system.relations(), &[eq2]);
    }

    #[test]
    fn test_clear() {
        let mut system = LinSys::new();
        let eq1 = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2]).unwrap());
        let eq2 = LinRel::mk_le(LinExpr::new(vec![0, 3, 4]).unwrap());

        system.add_relation(eq1);
        system.add_relation(eq2);
        assert_eq!(system.len(), 2);

        system.clear();
        assert!(system.is_empty());
        assert_eq!(system.len(), 0);
    }

    #[test]
    fn test_system_operations() {
        let eq1 = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2]).unwrap());
        let eq2 = LinRel::mk_le(LinExpr::new(vec![0, 3, 4]).unwrap());

        let mut system = LinSys::new();
        system.add_relation(eq1);
        system.add_relation(eq2);

        assert_eq!(system.len(), 2);
        assert!(!system.is_empty());
        assert_eq!(system.relations().len(), 2);

        let removed = system.remove_relation(0);
        assert!(removed.is_some());
        assert_eq!(system.len(), 1);
    }

    #[test]
    fn test_reduce_eqs() {
        // Test case: x1 + 2x2 = 0 and 3x1 + 4x2 <= 0
        // is_subs() returns position 1 (x1) since it's the first non-zero coefficient
        // Substituting x1 = -2x2 into 3x1 + 4x2 <= 0 gives -2x2 <= 0
        let eq1 = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2]).unwrap());
        let eq2 = LinRel::mk_le(LinExpr::new(vec![0, 3, 4]).unwrap());

        let mut system = LinSys::new();
        system.add_relation(eq1);
        system.add_relation(eq2);

        assert_eq!(system.len(), 2);

        // Should be able to reduce (returns true)
        let reduced = system.reduce_eqs();
        assert!(reduced);
        assert_eq!(system.len(), 1);

        // The remaining relation should be -2x2 <= 0 (after substituting x1 = -2x2)
        let remaining = system.relations()[0].clone();
        assert_eq!(
            remaining,
            LinRel::mk_le(LinExpr::new(vec![0, 0, -2]).unwrap())
        );
    }

    #[test]
    fn test_reduce_eqs_no_substitution() {
        // Test case with no eligible substitutions
        let eq1 = LinRel::mk_le(LinExpr::new(vec![0, 3, 4]).unwrap());
        let eq2 = LinRel::mk_le(LinExpr::new(vec![0, 5, 6]).unwrap());

        let mut system = LinSys::new();
        system.add_relation(eq1);
        system.add_relation(eq2);

        assert_eq!(system.len(), 2);

        // Should not be able to reduce (returns false)
        let reduced = system.reduce_eqs();
        assert!(!reduced);
        assert_eq!(system.len(), 2); // No relations removed
    }

    #[test]
    fn test_eliminate_nontrivial_eqs() {
        // Test case: x1 + 3x3 = 0, 1 + 2x2 = 0, and 3x1 + 4x2 <= 0
        // first reduction:  x1 = -3x3 => 1 + 2x2 = 0, 4x2 + (-9)x3 <= 0
        // second reduction: x2 = -1/2 => -2 + (-9)x3 <= 0

        let mut system = LinSys::from_relations(vec![
            LinRel::mk_eq(LinExpr::new(vec![0, 1, 0, 3]).unwrap()),
            LinRel::mk_eq(LinExpr::new(vec![1, 0, 2, 0]).unwrap()),
            LinRel::mk_le(LinExpr::new(vec![0, 3, 4, 0]).unwrap()),
        ]);

        assert_eq!(system.relations().len(), 3);
        system.eliminate_nontrivial_eqs();
        assert!(!system.has_trivial_contradiction());
        assert_eq!(system.relations().len(), 1);

        // The remaining relation should be -2 + (-9)x3 <= 0 after elimination
        let remaining = system.relations()[0].clone();
        assert_eq!(
            remaining,
            LinRel::mk_le(LinExpr::new(vec![-2, 0, 0, -9]).unwrap())
        );
    }

    #[test]
    fn test_eliminate_trivial_eqs_single_equality() {
        // Test case: 0 = 0 (trivial), x1 + x2 <= 0 (non-trivial)
        // After elimination: only x1 + x2 <= 0 should remain
        let mut system = LinSys::from_relations(vec![
            LinRel::mk_eq(LinExpr::new(vec![0, 0, 0]).unwrap()), // 0 = 0 (trivial)
            LinRel::mk_le(LinExpr::new(vec![0, 1, 1]).unwrap()), // x1 + x2 <= 0 (non-trivial)
        ]);

        assert_eq!(system.len(), 2);
        assert!(!system.has_trivial_contradiction());

        system.eliminate_trivial_eqs();

        assert_eq!(system.len(), 1);

        // The remaining relation should be x1 + x2 <= 0
        let remaining = system.relations()[0].clone();
        assert_eq!(
            remaining,
            LinRel::mk_le(LinExpr::new(vec![0, 1, 1]).unwrap())
        );
    }

    #[test]
    fn test_eliminate_trivial_eqs_both_types() {
        // Test case: 0 = 0 (trivial equality), -1 <= 0 (trivial inequality), x1 + x2 <= 0 (non-trivial)
        // After elimination: only x1 + x2 <= 0 should remain
        let mut system = LinSys::from_relations(vec![
            LinRel::mk_eq(LinExpr::new(vec![0, 0, 0]).unwrap()), // 0 = 0 (trivial equality)
            LinRel::mk_le(LinExpr::new(vec![-1, 0, 0]).unwrap()), // -1 <= 0 (trivial inequality)
            LinRel::mk_le(LinExpr::new(vec![2, 0, 0]).unwrap()), // 2 <= 0 (trivial contradiction)
            LinRel::mk_le(LinExpr::new(vec![0, 1, 1]).unwrap()), // x1 + x2 <= 0 (non-trivial)
        ]);

        assert_eq!(system.len(), 4);

        system.eliminate_trivial_eqs();

        assert_eq!(system.len(), 2);

        // The first remaining relation should be 2 <= 0
        let remaining = system.relations()[0].clone();
        assert_eq!(
            remaining,
            LinRel::mk_le(LinExpr::new(vec![2, 0, 0]).unwrap())
        );

        // The system still contains a trivial contradiction
        assert!(system.has_trivial_contradiction());

        // The remaining relation should be x1 + x2 <= 0
        let remaining = system.relations()[1].clone();
        assert_eq!(
            remaining,
            LinRel::mk_le(LinExpr::new(vec![0, 1, 1]).unwrap())
        );
    }
}
