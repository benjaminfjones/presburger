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

use std::fmt;

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

/// Implement smart constructors
impl Pred {
    pub fn not(p: Self) -> Self {
        Pred::Not(Box::new(p))
    }

    pub fn and(p: Self, q: Self) -> Self {
        Pred::And(Box::new(p), Box::new(q))
    }

    pub fn or(p: Self, q: Self) -> Self {
        Pred::Or(Box::new(p), Box::new(q))
    }

    pub fn implies(p: Self, q: Self) -> Self {
        Pred::Impl(Box::new(p), Box::new(q))
    }

    pub fn iff(p: Self, q: Self) -> Self {
        Pred::Iff(Box::new(p), Box::new(q))
    }

    pub fn exists(v: Var, p: Self) -> Self {
        Pred::Exists(v, Box::new(p))
    }

    pub fn forall(v: Var, p: Self) -> Self {
        Pred::Forall(v, Box::new(p))
    }

    pub fn atom(a: Atom) -> Self {
        Pred::Atom(Box::new(a))
    }
}

impl fmt::Display for Pred {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pred::Not(x) => write!(f, "-{}", x),
            Pred::And(p, q) => write!(f, "({} /\\ {})", *p, *q),
            Pred::Or(p, q) => write!(f, "({} \\/ {})", *p, *q),
            Pred::Impl(p, q) => write!(f, "({} ==> {})", *p, *q),
            Pred::Iff(p, q) => write!(f, "({} <==> {})", *p, *q),
            Pred::Exists(v, p) => write!(f, "(∃{}. {})", v, *p),
            Pred::Forall(v, p) => write!(f, "(∀{}. {})", v, *p),
            Pred::Atom(a) => write!(f, "({})", *a),
        }
    }
}
/// Implement syntactic equality for Pred
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

/// Implement smart constructors
impl Term {
    pub fn num(x: i64) -> Self {
        Term::Num(x)
    }

    pub fn var(name: &str) -> Self {
        Term::Var(Var::new(name))
    }

    pub fn add(t1: Self, t2: Self) -> Self {
        Term::Add(Box::new(t1), Box::new(t2))
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Num(x) => write!(f, "{}", x),
            Term::Var(x) => write!(f, "{}", x),
            Term::Add(a, b) => write!(f, "({} + {})", *a, *b),
        }
    }
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
        let t0 = Term::num(0);
        let t1 = Term::num(1);
        let t2 = Term::num(2);

        assert_eq!(t0, t0);
        assert!(t0 != t1);
        assert!(t0 != t2);

        let t4 = Term::var("x");  // x
        assert_eq!(t4, t4);
        assert!(t0 != t4);

        let t5 = Term::add(Term::var("x"), Term::num(1));  // x + 1
        assert_eq!(t5, t5);
        assert!(t0 != t5);
        assert!(t4 != t5);
    }

    #[test]
    fn atom_eq() {
        let a1 = Atom::truth(true);
        let a2 = Atom::truth(false);
        let a3 = Atom::var("P");
        let a4 = Atom::equality(Term::num(0), Term::num(0));
        let a5 = Atom::equality(Term::num(0), Term::num(0));  // intentionally same as a4

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
        let p1 = Pred::atom(Atom::truth(true));
        let p2 = Pred::atom(Atom::var("P"));
        let p2_ = Pred::atom(Atom::var("P"));
        let p3 = Pred::not(p1.clone());
        let p4 = Pred::not(p2.clone());  // not P
        let p4_ = Pred::not(p2_.clone());  // also not P
        let p5 = Pred::and(p3.clone(), p4.clone());  // not True AND not P

        assert_eq!(p1, p1);
        assert_eq!(p2, p2);
        assert_eq!(p2, p2_);
        assert_eq!(p3, p3);
        assert_eq!(p4, p4);
        assert_eq!(p4, p4_);
        assert_eq!(p5, p5);
    }
}
