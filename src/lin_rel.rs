//! Linear relations

use crate::lin_expr::{LinExpr, LinExprError};
use crate::types::Rational;
use std::fmt;

/// Special representation for `LinExp == 0`
#[derive(Debug, PartialEq, Eq)]
pub struct LinEq(LinExpr);

impl fmt::Display for LinEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = 0", self.0)
    }
}

impl LinEq {
    pub fn new(e: LinExpr) -> Self {
        LinEq(e)
    }

    pub fn from_coeffs(coeffs: Vec<impl Into<Rational>>) -> Result<Self, LinExprError> {
        Ok(LinEq(LinExpr::new(coeffs)?))
    }

    pub fn nvars(&self) -> usize {
        self.0.nvars()
    }

    pub fn coeffs(&self) -> &[Rational] {
        self.0.coeffs()
    }

    pub fn const_(&self) -> &Rational {
        self.0.const_()
    }

    pub fn lhs(&self) -> &LinExpr {
        &self.0
    }

    /// An equality is a possible substitution iff. some coeff == +-1.
    /// Return the position of the first substitution coefficient, or None.
    ///
    /// TODO: generalize all the subs methods to rationals
    pub fn is_subs(&self) -> Option<usize> {
        self.0
            .coeffs()
            .iter()
            .position(|c| c == &Rational::ONE || c == &Rational::from(-1))
            .map(|i| i + 1)
    }

    /// An equality is a possible substitution for x_i iff. coeff(x_i) == +-1.
    ///
    /// Returns `false` for variable indexes that are out of bounds.
    pub fn is_subs_for(&self, i: usize) -> bool {
        if let Ok(c) = self.0.coeff(i) {
            *c == Rational::ONE || *c == -Rational::ONE
        } else {
            false
        }
    }

    /// Substitute a linear expression for x_i using `other`, which must be a substitution equation,
    /// i.e. other.coeff(x_i) == Some(+-1).
    ///
    /// Because the result is a new equation, resulting from a deductive step, this method
    /// consumes `self` and returns a new equation.
    ///
    /// For example, Suppose vars are `{x_1, x_2, x_3}`,
    /// - `self` is  `3  x_1 + 4 x_2 = 0` and
    /// - `other` is `-3 x_1 +   x_2 + 2 x_3 = 0`,
    ///
    /// then substituting `x_2 = 3 x_1 - 2 x_3` produces `(3 + 3 * 4) x_1 + (0 - 2 * 4) x_3 = 0`,
    /// equivalent to `15 x_1 - 8 x_3 = 0`.
    ///
    /// ```
    /// # use presburger::lin_expr::*;
    /// # use presburger::lin_rel::*;
    /// # fn main () -> Result<(), LinExprError> {
    /// let eq = LinEq::new(LinExpr::new(vec![0, 3, 4, 0])?);
    /// let other = LinEq::new(LinExpr::new(vec![0, -3, 1, 2])?);
    /// let res = eq.subs(2, &other)?;
    /// assert_eq!(res, LinEq::new(LinExpr::new(vec![0, 15, 0, -8])?));
    /// # Ok(())
    /// # }
    /// ```
    pub fn subs(self, i: usize, other: &Self) -> Result<Self, LinExprError> {
        let n = self.nvars();
        debug_assert!(n == self.0.nvars());
        debug_assert!(n == other.0.nvars());
        // if coeff is 1, subtract other's coeffs from self
        // else if coeff is -1, add other's coeffs to self
        let m = if other.0.coeff(i)? == &Rational::ONE {
            -Rational::ONE
        } else {
            Rational::ONE
        };
        // Safe b/c nvars other == nvars self and we know other variable i is valid
        let se_coeff = self.0.coeff_unchecked(i);

        let mut new_lhs = LinExpr::new_zeros(n);
        for j in 1..=n {
            new_lhs.set_coeff_unchecked(
                j,
                self.0.coeff_unchecked(j) + m.clone() * other.0.coeff_unchecked(j) * se_coeff,
            );
        }
        new_lhs.set_const(self.0.const_() + m * other.0.const_() * se_coeff);
        Ok(LinEq(new_lhs))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Constraint {
    /// Equality
    Eq,
    /// Less than or equal to
    Le,
    /// Strictly less than or equal to
    Lt,
    /// Greater than or equal to
    Ge,
    /// Strictly greater than or equal to
    Gt,
}

/// Represents `LinExpr rel 0` where `rel` can be any (in)equality
#[derive(Debug, PartialEq, Eq)]
pub struct LinRel {
    lhs: LinExpr,
    constraint: Constraint,
}

impl LinRel {
    pub fn nvars(&self) -> usize {
        self.lhs.nvars()
    }

    pub fn coeffs(&self) -> &[Rational] {
        self.lhs.coeffs()
    }

    pub fn const_(&self) -> &Rational {
        self.lhs.const_()
    }

    pub fn lhs(&self) -> &LinExpr {
        &self.lhs
    }

    pub fn is_equality(&self) -> bool {
        matches!(self.constraint, Constraint::Eq)
    }

    pub fn to_equality(self) -> LinEq {
        LinEq(self.lhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lin_eq_basic_api() {
        let eq1 =
            LinEq::new(LinExpr::new(vec![0, 1, 2]).expect("failed to create linear expression"));
        assert_eq!(eq1.nvars(), 2);
        assert_eq!(eq1.is_subs(), Some(1));
        assert!(eq1.is_subs_for(1)); // subs: coeff of x_1 is 1
        assert!(!eq1.is_subs_for(2)); // not subs: coeff of x_2 is 2
    }

    // Suppose vars are `[x_1, x_2]`,
    // - `self` is `3 x_1 + 4 x_2 = 0`
    // - `other` is `-3 x_1 + x_2 = 0`,
    // then substituting for `x_2 = 3 x_1` produces `15 x_1 = 0`
    #[test]
    fn lin_eq_subs_2() {
        let eq1 =
            LinEq::new(LinExpr::new(vec![0, 3, 4]).expect("failed to create linear expression"));
        let eq2 =
            LinEq::new(LinExpr::new(vec![0, -3, 1]).expect("failed to create linear expression"));
        assert_eq!(eq1.nvars(), 2);
        assert_eq!(eq2.nvars(), 2);
        let eq3 = eq1.subs(2, &eq2).expect("subs failed");
        assert_eq!(eq3.nvars(), 2);
        assert_eq!(eq3.coeffs(), &[Rational::from(15), Rational::ZERO]);
        assert_eq!(eq3.const_(), &Rational::from(0));
        assert!(!eq3.lhs().supported(2));
        // eq1 was moved: assert_eq!(eq1.nvars(), 2);
    }

    // Suppose vars are {x_1, x_2, x_3}`
    // - self is  3  x_1 + 4 x_2 = 0 and
    // - other is -3 x_1 +   x_2 + 2 x_3 = 0,
    //
    // Substituting for x_2 = 3 x_1 - 2 x_3 produces
    // (3 + 3 * 4) x_1 + (0 - 2 * 4) x_3 = 0
    // ==> 15 x_1 - 8 x_3 = 0.
    #[test]
    fn lin_eq_subs_3() {
        let eq1 = LinEq::from_coeffs(vec![0, 3, 4, 0]).expect("failed to create linear equality");
        let eq2 = LinEq::from_coeffs(vec![0, -3, 1, 2]).expect("failed to create linear equality");
        let eq3 = eq1.subs(2, &eq2).expect("subs failed");
        assert_eq!(eq3.nvars(), 3);
        assert_eq!(eq3.coeffs(), &[15.into(), 0.into(), Rational::from(-8)]);
        assert_eq!(eq3.const_(), &Rational::ZERO);
        assert!(!eq3.lhs().supported(2));
    }

    // Substitution with non-zero constants
    // - self  is -1 + 3 x_1 + 5 x_2 = 0
    // - other is 7  -   x_1 + x_2   = 0
    //
    // Using other to substitute for x_1 in self leaves 20 + 8 x_2 = 0
    #[test]
    fn lin_eq_subs_const() {
        let eq1 = LinEq::from_coeffs(vec![-1, 3, 5]).expect("failed to create linear equality");
        let eq2 = LinEq::from_coeffs(vec![7, -1, 1]).expect("failed to create linear equality");
        let eq3 = eq1.subs(1, &eq2).expect("subs failed");
        assert_eq!(eq3.coeffs(), &[0.into(), 8.into()]);
        assert_eq!(eq3.const_(), &Rational::from(20));
        assert!(!eq3.lhs().supported(1));
        assert!(eq3.lhs().supported(2));
    }
}
