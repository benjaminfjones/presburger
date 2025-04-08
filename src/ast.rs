//! AST module
//!
//! This module defines the naive AST for Presburger formulas. These
//! formulas are part of the 1st order logic Th(0, 1, +, <=, ==).
//!
//! For example,
//!
//! 1) forall y. y < y + 1
//! 2) 0 <= x /\ x <= 10
//! 3) forall y. (exists x. x <= y ==> x + 1 <= y)
//! 4) ((P ==> Q) ==> P) ==> Q
//!
//! The AST is produced by the parser/grammer defined in `grammer.lalrpop`.
//!
#[allow(unused_imports)]
use crate::types::{Integer, Rational};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Formula {
    /// Negation
    Not(Box<Formula>),
    /// AND
    And(Box<Formula>, Box<Formula>),
    /// Inclusive OR
    Or(Box<Formula>, Box<Formula>),
    /// Implication
    Impl(Box<Formula>, Box<Formula>),
    /// If and only if
    Iff(Box<Formula>, Box<Formula>),
    /// exists y. p(y)
    Exists(Var, Box<Formula>),
    /// forall x. p(x)
    Forall(Var, Box<Formula>),
    /// atomic predicates
    Atom(Box<Atom>),
}

/// Implement smart constructors
impl Formula {
    /// Formula not
    pub fn fnot(p: Self) -> Self {
        Formula::Not(Box::new(p))
    }

    pub fn and(p: Self, q: Self) -> Self {
        Formula::And(Box::new(p), Box::new(q))
    }

    pub fn or(p: Self, q: Self) -> Self {
        Formula::Or(Box::new(p), Box::new(q))
    }

    pub fn implies(p: Self, q: Self) -> Self {
        Formula::Impl(Box::new(p), Box::new(q))
    }

    pub fn iff(p: Self, q: Self) -> Self {
        Formula::Iff(Box::new(p), Box::new(q))
    }

    pub fn exists(v: Var, p: Self) -> Self {
        Formula::Exists(v, Box::new(p))
    }

    pub fn forall(v: Var, p: Self) -> Self {
        Formula::Forall(v, Box::new(p))
    }

    pub fn atom(a: Atom) -> Self {
        Formula::Atom(Box::new(a))
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Not(x) => write!(f, "~{}", x),
            Formula::And(p, q) => write!(f, "({} /\\ {})", *p, *q),
            Formula::Or(p, q) => write!(f, "({} \\/ {})", *p, *q),
            Formula::Impl(p, q) => write!(f, "({} ==> {})", *p, *q),
            Formula::Iff(p, q) => write!(f, "({} <==> {})", *p, *q),
            Formula::Exists(v, p) => write!(f, "(∃{}. {})", v, *p),
            Formula::Forall(v, p) => write!(f, "(∀{}. {})", v, *p),
            Formula::Atom(a) => write!(f, "({})", *a),
        }
    }
}
/// Implement syntactic equality for Formula
impl PartialEq for Formula {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Formula::Not(p1), Formula::Not(p2)) => *p1 == *p2,
            (Formula::And(p1, q1), Formula::And(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Formula::Or(p1, q1), Formula::Or(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Formula::Impl(p1, q1), Formula::Impl(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Formula::Iff(p1, q1), Formula::Iff(p2, q2)) => *p1 == *p2 && *q1 == *q2,
            (Formula::Exists(v1, p1), Formula::Exists(v2, p2)) => v1 == v2 && *p1 == *p2,
            (Formula::Forall(v1, p1), Formula::Forall(v2, p2)) => v1 == v2 && *p1 == *p2,
            (Formula::Atom(a1), Formula::Atom(a2)) => *a1 == *a2,
            _ => false,
        }
    }
}
impl Eq for Formula {}

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

/// Implement smart constructors
impl Atom {
    pub fn truth(val: bool) -> Self {
        Atom::TruthValue(val)
    }

    pub fn var(name: &str) -> Self {
        Atom::LogicalVar(Var::new(name))
    }

    pub fn equality(t1: Term, t2: Term) -> Self {
        Atom::Equality(Box::new(t1), Box::new(t2))
    }

    pub fn less_eq(t1: Term, t2: Term) -> Self {
        Atom::LessEq(Box::new(t1), Box::new(t2))
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::TruthValue(x) => write!(f, "{}", x),
            Atom::LogicalVar(x) => write!(f, "{}", x),
            Atom::Equality(a, b) => write!(f, "{} == {}", *a, *b),
            Atom::LessEq(a, b) => write!(f, "{} <= {}", *a, *b),
        }
    }
}
/// Implement syntactic equality for Atoms
impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Atom::TruthValue(b1), Atom::TruthValue(b2)) => b1 == b2,
            (Atom::LogicalVar(v1), Atom::LogicalVar(v2)) => v1 == v2,
            (Atom::Equality(lhs1, rhs1), Atom::Equality(lhs2, rhs2)) => {
                *lhs1 == *lhs2 && *rhs1 == *rhs2
            }
            (Atom::LessEq(lhs1, rhs1), Atom::LessEq(lhs2, rhs2)) => {
                *lhs1 == *lhs2 && *rhs1 == *rhs2
            }
            _ => false,
        }
    }
}
impl Eq for Atom {}

/// `Term` Represents a base numerical term
///
/// TODO: missing scalar multiple of variable
#[derive(Clone, Debug)]
pub enum Term {
    /// non-negative integer literal
    Num(Rational),
    /// numerical variable
    ScalarVar(Rational, Var),
    /// t1 + t2
    Add(Box<Term>, Box<Term>),
}

/// Implement smart constructors
impl Term {
    pub fn num(x: impl Into<Rational>) -> Self {
        Term::Num(x.into())
    }

    pub fn scalar_var(s: Rational, name: &str) -> Self {
        Term::ScalarVar(s, Var::new(name))
    }

    /// Term add
    pub fn tadd(t1: Self, t2: Self) -> Self {
        Term::Add(Box::new(t1), Box::new(t2))
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Num(x) => write!(f, "{}", x),
            Term::ScalarVar(a, x) => write!(f, "{} {}", a, x),
            Term::Add(a, b) => write!(f, "({} + {})", *a, *b),
        }
    }
}
/// Implement syntactic equality on Terms
impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Term::Num(x), Term::Num(y)) => x == y,
            (Term::ScalarVar(a, x), Term::ScalarVar(b, y)) => a == b && x == y,
            (Term::Add(a, b), Term::Add(c, d)) => a == c && b == d,
            _ => false,
        }
    }
}
impl Eq for Term {}

/// `Var` represents a variable name, it is a newtype over String
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var(pub String);

/// Implement smart constructor
impl Var {
    pub fn new(name: &str) -> Self {
        Var(name.to_string())
    }
}
impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_var_eq() {
        let v1 = Var::new("x");
        let v2 = Var::new("x");
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_var_neq() {
        let v1 = Var::new("x");
        let v2 = Var::new("y");
        assert!(v1 != v2);

        let v3 = Var::new("X");
        assert!(v1 != v3);
    }

    #[test]
    fn term_eq() {
        let t0 = Term::num(Rational::from(0));
        let t1 = Term::num(Rational::from(1));
        let t2 = Term::num(Rational::from(2));

        assert_eq!(t0, t0);
        assert!(t0 != t1);
        assert!(t0 != t2);

        let t4 = Term::scalar_var(Rational::from(1), "x"); // x
        assert_eq!(t4, t4);
        assert_ne!(t0, t4);
        let t5 = Term::tadd(
            Term::scalar_var(Rational::from(1), "x"),
            Term::num(Rational::from(1)),
        ); // x + 1
        assert_eq!(t5, t5);
        assert_ne!(t0, t5);
        assert_ne!(t4, t5);
    }

    #[test]
    fn atom_eq() {
        let zero = Term::num(Rational::from(0));
        let a1 = Atom::truth(true);
        let a2 = Atom::truth(false);
        let a3 = Atom::var("P");
        let a4 = Atom::equality(zero.clone(), zero.clone());
        let a5 = Atom::equality(zero.clone(), zero.clone()); // intentionally same as a4

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
        let p1 = Formula::atom(Atom::truth(true));
        let p2 = Formula::atom(Atom::var("P"));
        let p2_ = Formula::atom(Atom::var("P"));
        let p3 = Formula::fnot(p1.clone());
        let p4 = Formula::fnot(p2.clone()); // not P
        let p4_ = Formula::fnot(p2_.clone()); // also not P
        let p5 = Formula::and(p3.clone(), p4.clone()); // not True AND not P

        assert_eq!(p1, p1);
        assert_eq!(p2, p2);
        assert_eq!(p2, p2_);
        assert_eq!(p3, p3);
        assert_eq!(p4, p4);
        assert_eq!(p4, p4_);
        assert_eq!(p5, p5);
    }
}
