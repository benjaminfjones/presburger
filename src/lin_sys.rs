//! Implementation of linear systems: a collection of linear relations

use crate::lin_rel::LinRel;

/// A system of linear relations
#[derive(Debug, PartialEq, Eq)]
pub struct LinSys {
    relations: Vec<LinRel>,
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
}
