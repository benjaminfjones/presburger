//! Implemenetation of linear relations: b + \sum_{i=0}^n a_i x_i = 0 (or <= 0)

use crate::lin_expr::{LinExpr, LinExprError};
use crate::types::Rational;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Constraint {
    /// Equality
    Eq,
    /// Less than or equal to
    Le,
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Constraint::Eq => "=",
            Constraint::Le => "<=",
        };
        write!(f, "{symbol}")
    }
}

/// Represents `LinExpr rel 0` where `rel` can be any (in)equality
///
/// Note that the derived equality is only structural, not mathematical equality.
#[derive(Debug, PartialEq, Eq)]
pub struct LinRel {
    lhs: LinExpr,
    constraint: Constraint,
}

impl fmt::Display for LinRel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} 0", self.lhs, self.constraint)
    }
}

impl LinRel {
    pub fn mk_eq(lhs: LinExpr) -> Self {
        Self {
            lhs,
            constraint: Constraint::Eq,
        }
    }

    pub fn mk_le(lhs: LinExpr) -> Self {
        Self {
            lhs,
            constraint: Constraint::Le,
        }
    }

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

    /// An equality is a possible substitution iff. some coeff == +-1.
    /// Return the position of the first substitution coefficient, or None.
    ///
    /// TODO: generalize all the subs methods to rationals
    pub fn is_subs(&self) -> Option<usize> {
        if self.constraint != Constraint::Eq {
            return None;
        }
        self.lhs
            .coeffs()
            .iter()
            .position(|c| !c.is_zero())
            .map(|i| i + 1)
    }

    /// An equality is a possible substitution for x_i iff. coeff(x_i) != 0
    ///
    /// Returns `false` for variable indexes that are out of bounds.
    pub fn is_subs_for(&self, i: usize) -> bool {
        if self.constraint != Constraint::Eq {
            return false;
        }
        if let Ok(c) = self.lhs.coeff(i) {
            !c.is_zero()
        } else {
            false
        }
    }

    /// Substitute a linear expression for x_i using `other`, which must be a substitution equation,
    /// i.e. other.coeff(x_i) != 0
    ///
    /// Because the result is a new relation, resulting from a deductive step, this method
    /// consumes `self` and returns a new equation.
    ///
    /// For example, Suppose vars are `{x_1, x_2, x_3}`,
    /// - `self` is  `3  x_1 + 4 x_2 <= 0` and
    /// - `other` is `-3 x_1 +   x_2 + 2 x_3 = 0`,
    ///
    /// then substituting `x_2 = 3 x_1 - 2 x_3` produces `(3 + 3 * 4) x_1 + (0 - 2 * 4) x_3 <= 0`,
    /// equivalent to `15 x_1 - 8 x_3 <= 0`.
    ///
    /// ```
    /// # use presburger::lin_expr::*;
    /// # use presburger::lin_rel::*;
    /// # fn main () -> Result<(), LinExprError> {
    /// let le = LinRel::mk_le(LinExpr::new(vec![0, 3, 4, 0])?);
    /// let other = LinRel::mk_eq(LinExpr::new(vec![0, -3, 1, 2])?);
    /// let res = le.subs(2, &other)?;
    /// assert_eq!(res, LinRel::mk_le(LinExpr::new(vec![0, 15, 0, -8])?));
    /// # Ok(())
    /// # }
    /// ```
    pub fn subs(self, i: usize, other: &Self) -> Result<Self, LinExprError> {
        if other.constraint != Constraint::Eq {
            // only equalities can be substituted
            return Err(LinExprError::AssertionError);
        }
        let n = self.nvars();
        debug_assert!(n == self.lhs.nvars());
        debug_assert!(n == other.lhs.nvars());
        // if coeff is 1, subtract other's coeffs from self
        // else if coeff is -1, add other's coeffs to self
        let m = -Rational::ONE / other.lhs.coeff(i)?.clone();
        // let m = if other.lhs.coeff(i)? == &Rational::ONE {
        //     -Rational::ONE
        // } else {
        //     Rational::ONE
        // };
        // Safe b/c nvars other == nvars self and we know other variable i is valid
        let se_coeff = self.lhs.coeff_unchecked(i);

        let mut new_lhs = LinExpr::new_zeros(n);
        for j in 1..=n {
            new_lhs.set_coeff_unchecked(
                j,
                self.lhs.coeff_unchecked(j) + m.clone() * other.lhs.coeff_unchecked(j) * se_coeff,
            );
        }
        new_lhs.set_const(self.lhs.const_() + m * other.lhs.const_() * se_coeff);
        Ok(Self {
            lhs: new_lhs,
            constraint: self.constraint,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lin_eq_basic_api() {
        let eq1 = LinRel::mk_eq(
            LinExpr::new(vec![0, 1, 2, 0]).expect("failed to create linear equality"),
        );
        assert_eq!(eq1.nvars(), 3);
        assert_eq!(eq1.is_subs(), Some(1));
        assert!(eq1.is_subs_for(1)); // subs: coeff of x_1 is 1
        assert!(eq1.is_subs_for(2)); // subs: coeff of x_2 is 2
        assert!(!eq1.is_subs_for(3)); // not subs: coeff of x_3 is 0
    }

    // Suppose vars are `[x_1, x_2]`,
    // - `self` is `3 x_1 + 4 x_2 <= 0`
    // - `other` is `-3 x_1 + x_2 = 0`,
    // then substituting for `x_2 = 3 x_1` produces `15 x_1 <= 0`
    #[test]
    fn lin_eq_subs_2() {
        let slf =
            LinRel::mk_le(LinExpr::new(vec![0, 3, 4]).expect("failed to create linear expression"));
        let other = LinRel::mk_eq(
            LinExpr::new(vec![0, -3, 1]).expect("failed to create linear expression"),
        );
        assert_eq!(slf.nvars(), 2);
        assert_eq!(other.nvars(), 2);
        let result = slf.subs(2, &other).expect("subs failed");
        assert_eq!(result.nvars(), 2);
        assert_eq!(result.coeffs(), &[Rational::from(15), Rational::ZERO]);
        assert_eq!(result.const_(), &Rational::from(0));
        assert!(!result.lhs().supported(2));
    }

    // Suppose vars are {x_1, x_2, x_3}`
    // - self is  3  x_1 + 4 x_2 <= 0 and
    // - other is -3 x_1 +   x_2 + 2 x_3 = 0,
    //
    // Substituting for x_2 = 3 x_1 - 2 x_3 produces
    // (3 + 3 * 4) x_1 + (0 - 2 * 4) x_3 <= 0
    // ==> 15 x_1 - 8 x_3 <= 0.
    #[test]
    fn lin_eq_subs_3() {
        let slf = LinRel::mk_le(
            LinExpr::new(vec![0, 3, 4, 0]).expect("failed to create linear equality"),
        );
        let other = LinRel::mk_eq(
            LinExpr::new(vec![0, -3, 1, 2]).expect("failed to create linear equality"),
        );
        let result = slf.subs(2, &other).expect("subs failed");
        assert_eq!(result.nvars(), 3);
        assert_eq!(result.coeffs(), &[15.into(), 0.into(), Rational::from(-8)]);
        assert_eq!(result.const_(), &Rational::ZERO);
        assert!(!result.lhs().supported(2));
    }

    // Substitution with non-zero constants
    // - self  is -1 + 3 x_1 + 5 x_2 = 0
    // - other is 7  -   x_1 + x_2   = 0
    //
    // Using other to substitute for x_1 in self leaves 20 + 8 x_2 = 0
    #[test]
    fn lin_eq_subs_const() {
        let eq1 = LinRel::mk_eq(LinExpr::new(vec![-1, 3, 5]).unwrap());
        let eq2 = LinRel::mk_eq(LinExpr::new(vec![7, -1, 1]).unwrap());
        let eq3 = eq1.subs(1, &eq2).expect("subs failed");
        assert_eq!(eq3.coeffs(), &[0.into(), 8.into()]);
        assert_eq!(eq3.const_(), &Rational::from(20));
        assert!(!eq3.lhs().supported(1));
        assert!(eq3.lhs().supported(2));
    }
}
