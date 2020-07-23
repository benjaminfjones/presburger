/// AST module
///
/// This module defines the naive AST for Presburger expressions. These
/// expressions are part of the 1st order logic Th(0, 1, +, <).
///
/// For example,
///
/// 1) forall y. y < y + 1
/// 2) 0 <= x /\ x < 10
/// 3) forall y. (exists x. x < y ==> x + 1 <= y)
/// 4) ((P ==> Q) ==> P) ==> Q
///
/// The AST is produced by the parser/grammer defined in `grammer.lalrpop`.
///

#[derive(Clone, Debug)]
pub enum Pred {
    /// Negation
    Not(Box<Pred>),
    /// AND
    And(Box<Pred>, Box<Pred>),
    /// Inclusive OR
    Or(Box<Pred>, Box<Pred>),
    /// Implication
    Impl(Box<Pred>, Box<Pred>),
    /// If and only if
    Iff(Box<Pred>, Box<Pred>),
    /// exists y. p(y)
    Exists(Var, Box<Pred>),
    /// forall x. p(x)
    Forall(Var, Box<Pred>),
    /// atomic predicates
    Atom(Box<Atom>),
}

// Implement syntactic equality for Pred
impl PartialEq for Pred {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Pred::Not(p1), Pred::Not(p2)) => *p1 == *p2,
            (Pred::And(p1, q1), Pred::And(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Pred::Or(p1, q1), Pred::Or(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Pred::Impl(p1, q1), Pred::Impl(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Pred::Iff(p1, q1), Pred::Iff(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Pred::Exists(v1, p1), Pred::Exists(v2, p2)) => v1 == v2 && *p1 == *p2,
            (Pred::Forall(v1, p1), Pred::Forall(v2, p2)) => v1 == v2 && *p1 == *p2,
            (Pred::Atom(a1), Pred::Atom(a2)) => *a1 == *a2,
            _ => false,
        }
    }

}
impl Eq for Pred { }

/// `Atom` represents an atomic predicate (with respect to the logical connectives)
#[derive(Clone, Debug)]
pub enum Atom {
    /// true <-> "T", false <-> "F"
    TruthValue(bool),
    /// logical variable (must be all uppercase)
    LogicalVar(Var),
    /// t1 = t2
    Equality(Box<Term>, Box<Term>),
    /// t1 <= t2
    LessEq(Box<Term>, Box<Term>),
    // TODO finish rest of atoms
}

/// Implement syntactic equality for Atoms
impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Atom::TruthValue(b1), Atom::TruthValue(b2)) => b1 == b2,
            (Atom::LogicalVar(v1), Atom::LogicalVar(v2)) => v1 == v2,
            (Atom::Equality(lhs1, rhs1), Atom::Equality(lhs2, rhs2)) => *lhs1 == *lhs2 && *rhs1 == *rhs2,
            (Atom::LessEq(lhs1, rhs1), Atom::LessEq(lhs2, rhs2)) => *lhs1 == *lhs2 && *rhs1 == *rhs2,
            _ => false,
        }
    }
}
impl Eq for Atom { }

/// `Term` Represents a base numerical term
#[derive(Clone, Debug)]
pub enum Term {
    /// non-negative integer literal
    Num(i64),
    /// numerical variable
    Var(Var),
    /// t1 + t2
    Add(Box<Term>, Box<Term>),
}

/// Implement syntactic equality on Terms
impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Term::Num(x), Term::Num(y)) => x == y,
            (Term::Var(x), Term::Var(y)) => x == y,
            (Term::Add(a, b), Term::Add(c, d)) => a == c && b == d,
            _ => false,
        }
    }
}
impl Eq for Term {}

/// `Var` represents a variable name, it is a newtype over String
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var(pub String);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_var_eq() {
        let v1 = Var("x".to_string());
        let v2 = Var("x".to_string());
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_var_neq() {
        let v1 = Var("x".to_string());
        let v2 = Var("y".to_string());
        assert!(v1 != v2);

        let v3 = Var("X".to_string());
        assert!(v1 != v3);
    }

    #[test]
    fn term_eq() {
        let t0 = Term::Num(0);
        let t1 = Term::Num(1);
        let t2 = Term::Num(2);

        assert_eq!(t0, t0);
        assert!(t0 != t1);
        assert!(t0 != t2);

        let t4 = Term::Var(Var("x".to_string()));
        assert_eq!(t4, t4);
        assert!(t0 != t4);

        let t5 = Term::Add(Box::new(Term::Var(Var("x".to_string()))), Box::new(Term::Num(1)));
        assert_eq!(t5, t5);
        assert!(t0 != t5);
        assert!(t4 != t5);
    }

    #[test]
    fn atom_eq() {
        let a1 = Atom::TruthValue(true);
        let a2 = Atom::TruthValue(false);
        let a3 = Atom::LogicalVar(Var("P".to_string()));
        let a4 = Atom::Equality(Box::new(Term::Num(0)), Box::new(Term::Num(0)));
        let a5 = Atom::Equality(Box::new(Term::Num(0)), Box::new(Term::Num(0)));  // intentionally same as a4

        assert_eq!(a1, a1);
        assert_eq!(a2, a2);
        assert_eq!(a3, a3);
        assert_eq!(a4, a4);
        assert_eq!(a4, a5);

        assert!(a1 != a2);
        assert!(a1 != a3);
        assert!(a1 != a4);
    }

    #[test]
    fn pred_eq() {
        // Note: sub-predicates can't be shared since the Box takes ownership.
        let p1 = Pred::Atom(Box::new(Atom::TruthValue(true)));
        let p2 = Pred::Atom(Box::new(Atom::LogicalVar(Var("P".to_string()))));  // P
        let p2_ = Pred::Atom(Box::new(Atom::LogicalVar(Var("P".to_string()))));  // also P
        let p3 = Pred::Not(Box::new(p1.clone()));
        let p4 = Pred::Not(Box::new(p2.clone()));  // not P
        let p4_ = Pred::Not(Box::new(p2_.clone()));  // also not P
        let p5 = Pred::And(Box::new(p3.clone()), Box::new(p4.clone()));  // not True AND not P

        assert_eq!(p1, p1);
        assert_eq!(p2, p2);
        assert_eq!(p2, p2_);
        assert_eq!(p3, p3);
        assert_eq!(p4, p4);
        assert_eq!(p4, p4_);
        assert_eq!(p5, p5);
    }
}
