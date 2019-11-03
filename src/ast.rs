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

#[derive(Debug)]
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

/// `Atom` represents an atomic predicate (with respect to the logical connectives)
#[derive(Debug)]
pub enum Atom {
    /// true <-> "T", false <-> "F"
    TruthValue(bool),
    /// logical variable (must be all uppercase)
    LogicalVar(Var),
    /// t1 = t2
    Equality(Box<Term>, Box<Term>),
    /// t1 < t2
    LessThan(Box<Term>, Box<Term>),
    // XXX finish rest of atoms
}

/// `Term` Represents a base numerical term
#[derive(Debug)]
pub enum Term {
    /// non-negative integer literal
    Num(i64),
    /// numerical variable
    Var(Var),
    /// t1 + t2
    Add(Box<Term>, Box<Term>),
}

/// `Var` represents a variable name, it is a newtype over String
#[derive(Debug)]
pub struct Var(pub String);
