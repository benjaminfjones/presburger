//! Implemenetation of linear relations: b + \sum_{i=0}^n a_i x_i = 0 (or <= 0)

use crate::lin_expr::{Bound, LinExpr, LinExprBound, LinExprError};
use crate::types::Rational;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
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

/// Represents `LinExpr rel 0` where `rel` can be = or <= (in)equality
///
/// Note that the derived equality is only structural, not mathematical equality.
#[derive(Debug, PartialEq, Eq, Clone)]
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

    /// Create a normalized expr <= 0 relation from a non-normalized one: lhs <= rhs
    pub fn le_from_lhs_rhs(lhs: &LinExpr, rhs: &LinExpr) -> Self {
        let n = lhs.nvars();
        debug_assert!(n == rhs.nvars());
        let mut expr = LinExpr::new_zeros(n);
        expr.set_const(lhs.const_() - rhs.const_());
        for i in 1..=n {
            expr.set_coeff(i, lhs.coeff_unchecked(i) - rhs.coeff_unchecked(i))
                .unwrap();
        }
        Self::mk_le(expr)
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

    /// An equality is a possible substitution iff. some coeff is non-zero.
    /// Return the position of the first substitution coefficient, or None.
    ///
    /// # Examples
    ///
    /// Positive test: x_1 + 2 x_2 = 0 is a substitution (first non-zero coefficient at position 1)
    /// ```
    /// # use presburger::lin_expr::*;
    /// # use presburger::lin_rel::*;
    /// # fn main() -> Result<(), LinExprError> {
    /// let eq = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2])?);
    /// assert_eq!(eq.is_subs(), Some(1));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Negative test: 0 = 0 is not a substitution (all coefficients are zero)
    /// ```
    /// # use presburger::lin_expr::*;
    /// # use presburger::lin_rel::*;
    /// # fn main() -> Result<(), LinExprError> {
    /// let eq = LinRel::mk_eq(LinExpr::new(vec![0, 0, 0])?);
    /// assert_eq!(eq.is_subs(), None);
    /// # Ok(())
    /// # }
    /// ```
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
    ///
    /// # Examples
    ///
    /// Positive test: x_1 + 2 x_2 = 0 is a substitution for x_1
    /// ```
    /// # use presburger::lin_expr::*;
    /// # use presburger::lin_rel::*;
    /// # fn main() -> Result<(), LinExprError> {
    /// let eq = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2])?);
    /// assert!(eq.is_subs_for(1));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Negative test: x_1 + 2 x_2 = 0 is not a substitution for x_3 (coefficient is 0)
    /// ```
    /// # use presburger::lin_expr::*;
    /// # use presburger::lin_rel::*;
    /// # fn main() -> Result<(), LinExprError> {
    /// let eq = LinRel::mk_eq(LinExpr::new(vec![0, 1, 2])?);
    /// assert!(!eq.is_subs_for(3));
    /// # Ok(())
    /// # }
    /// ```
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
        let m = -Rational::ONE / other.lhs.coeff(i)?.clone();
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

    /// Determine if an inequality has a variable that can be isolated; return the index of that
    /// variable or None if none exists.
    ///
    /// TODO: lin_rel::is_isolatable_le: refactor to share code with `is_subs`
    pub fn is_isolatable_le(&self) -> Option<usize> {
        if self.constraint != Constraint::Le {
            return None;
        }
        self.lhs
            .coeffs()
            .iter()
            .position(|c| !c.is_zero())
            .map(|i| i + 1)
    }

    /// Determine if `self` is a trivially true (in)equality between constants,
    /// e.g. 0 = 0, or -1 <= 0
    pub fn is_trivial(&self) -> bool {
        let coeffs_zero = self.lhs.coeffs().iter().all(|c| c.is_zero());
        coeffs_zero
            && match self.constraint {
                Constraint::Eq => self.const_().is_zero(),
                Constraint::Le => self.const_() <= &Rational::ZERO,
            }
    }

    /// Determine if `self` is a trivially false (in)equality between constants,
    /// e.g. 1 = 0, or 2 <= 0
    pub fn is_trivial_contradiction(&self) -> bool {
        let coeffs_zero = self.lhs.coeffs().iter().all(|c| c.is_zero());
        coeffs_zero
            && match self.constraint {
                Constraint::Eq => !self.const_().is_zero(),
                Constraint::Le => self.const_() > &Rational::ZERO,
            }
    }

    /// Find the first variable that is possible to eliminate by finding the first non-zero
    /// coefficient of `self.lhs`.
    pub fn find_variable_to_eliminate(&self) -> Option<usize> {
        self.lhs
            .coeffs()
            .iter()
            .enumerate()
            .find(|(_i, c)| !c.is_zero())
            .map(|(i, _c)| i)
    }

    /// Isolate variable `i` (> 0) and return an upper or lower bound depending on the sign of its
    /// coefficient `a_i`.
    ///
    /// If `a_i` = 0, or `i` is out of bounds, return None
    ///
    /// Example: `1 + x1 + 3x2 <= 0` with `i = 1` results in `x1 <= -1 + (-3)x2`, an Upper bound
    pub fn compute_bound_from(&self, i: usize) -> Option<LinExprBound> {
        debug_assert!(i > 0);
        let mut coeffs = vec![self.lhs.const_()];
        coeffs.extend(self.lhs.coeffs());
        let ai = *coeffs.get(i)?;
        if ai.is_zero() {
            return None;
        }
        let bound = if ai > &Rational::ZERO {
            Bound::Upper
        } else {
            Bound::Lower
        };
        let mut new_coeffs: Vec<Rational> = coeffs.iter().map(|&c| -c.clone() / ai).collect();
        new_coeffs[i] = Rational::ZERO;
        let expr = LinExpr::new(new_coeffs)
            .expect("unreachable because the constant makes coeffs/new_coeffs non-empty");
        Some(LinExprBound { i, bound, expr })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{eq, le};

    #[test]
    fn lin_eq_basic_api() {
        let eq1 = eq!(0, 1, 2, 0);
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
        let slf = le!(0, 3, 4);
        let other = eq!(0, -3, 1);
        assert_eq!(slf.nvars(), 2);
        assert_eq!(other.nvars(), 2);
        let result = slf.subs(2, &other).expect("subs failed");
        assert_eq!(result.nvars(), 2);
        assert_eq!(result, le!(0, 15, 0));
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
        let slf = le!(0, 3, 4, 0);
        let other = eq!(0, -3, 1, 2);
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
        let eq1 = eq!(-1, 3, 5);
        let eq2 = eq!(7, -1, 1);
        let eq3 = eq1.subs(1, &eq2).expect("subs failed");
        assert_eq!(eq3, eq!(20, 0, 8));
        assert!(!eq3.lhs().supported(1));
        assert!(eq3.lhs().supported(2));
    }

    #[test]
    fn test_is_trivial() {
        // Positive test: 0 = 0 is trivial (equality)
        let trivial_eq = eq!(0, 0, 0);
        assert!(trivial_eq.is_trivial());

        // Positive test: -1 <= 0 is trivial (inequality)
        let trivial_le = le!(-1, 0, 0);
        assert!(trivial_le.is_trivial());

        // Positive test: 0 <= 0 is trivial (inequality)
        let trivial_le_zero = le!(0, 0, 0);
        assert!(trivial_le_zero.is_trivial());

        // Negative test: x1 = 0 is not trivial (has variables)
        let non_trivial_eq = eq!(0, 1, 0);
        assert!(!non_trivial_eq.is_trivial());

        // Negative test: x1 + x2 <= 0 is not trivial (has variables)
        let non_trivial_le = le!(0, 1, 1);
        assert!(!non_trivial_le.is_trivial());

        // Negative test: 1 = 0 is not trivial (it's a contradiction)
        let contradiction_eq = eq!(1, 0, 0);
        assert!(!contradiction_eq.is_trivial());

        // Negative test: 1 <= 0 is not trivial (it's a contradiction)
        let contradiction_le = le!(1, 0, 0);
        assert!(!contradiction_le.is_trivial());
    }

    #[test]
    fn test_is_trivial_contradiction() {
        // Positive test: 1 = 0 is a contradiction (equality)
        let contradiction_eq = eq!(1, 0, 0);
        assert!(contradiction_eq.is_trivial_contradiction());

        // Positive test: 2 <= 0 is a contradiction (inequality)
        let contradiction_le = le!(2, 0, 0);
        assert!(contradiction_le.is_trivial_contradiction());

        // Positive test: 0.5 <= 0 is a contradiction (inequality)
        let half = Rational::from(1) / Rational::from(2);
        let contradiction_le_half = le!(half, Rational::ZERO, Rational::ZERO);
        assert!(contradiction_le_half.is_trivial_contradiction());

        // Negative test: 0 = 0 is not a contradiction (it's trivial)
        let trivial_eq = eq!(0, 0, 0);
        assert!(!trivial_eq.is_trivial_contradiction());

        // Negative test: -1 <= 0 is not a contradiction (it's trivial)
        let trivial_le = le!(-1, 0, 0);
        assert!(!trivial_le.is_trivial_contradiction());

        // Negative test: x1 = 0 is not a contradiction (has variables)
        let non_contradiction_eq = eq!(0, 1, 0);
        assert!(!non_contradiction_eq.is_trivial_contradiction());

        // Negative test: x1 + x2 <= 0 is not a contradiction (has variables)
        let non_contradiction_le = le!(0, 1, 1);
        assert!(!non_contradiction_le.is_trivial_contradiction());

        // Negative test: 0 <= 0 is not a contradiction (it's trivial)
        let trivial_le_zero = le!(0, 0, 0);
        assert!(!trivial_le_zero.is_trivial_contradiction());
    }

    #[test]
    fn test_compute_bound_from() {
        // Test case 1: Upper bound from positive coefficient
        // 3x1 + 2x2 <= 0 should give x1 <= (-2x2)/3
        let le1 = le!(0, 3, 2);
        let bound1 = le1.compute_bound_from(1).unwrap();
        assert_eq!(bound1.i, 1);
        assert!(matches!(bound1.bound, Bound::Upper));
        assert_eq!(bound1.expr.const_(), &Rational::ZERO);
        assert_eq!(bound1.expr.coeff(1).unwrap(), &Rational::ZERO);
        let expected_coeff1 = Rational::from(-2) / Rational::from(3);
        assert_eq!(bound1.expr.coeff(2).unwrap(), &expected_coeff1);

        // Test case 2: Lower bound from negative coefficient
        // -3x1 + 2x2 <= 0 should give 2x2/3 <= x1
        let le2 = le!(0, -3, 2);
        let bound2 = le2.compute_bound_from(1).unwrap();
        assert_eq!(bound2.i, 1);
        assert!(matches!(bound2.bound, Bound::Lower));
        assert_eq!(bound2.expr.const_(), &Rational::ZERO);
        assert_eq!(bound2.expr.coeff(1).unwrap(), &Rational::ZERO);
        let expected_coeff2 = Rational::from(2) / Rational::from(3);
        assert_eq!(bound2.expr.coeff(2).unwrap(), &expected_coeff2);

        // Test case 3: With constant term
        // 5 + 2x1 - 3x2 <= 0 should give x1 <= (3x2 - 5)/2
        let le3 = le!(5, 2, -3);
        let bound3 = le3.compute_bound_from(1).unwrap();
        assert_eq!(bound3.i, 1);
        assert!(matches!(bound3.bound, Bound::Upper));
        let expected_const3 = Rational::from(-5) / Rational::from(2);
        assert_eq!(bound3.expr.const_(), &expected_const3);
        assert_eq!(bound3.expr.coeff(1).unwrap(), &Rational::ZERO);
        let expected_coeff3 = Rational::from(3) / Rational::from(2);
        assert_eq!(bound3.expr.coeff(2).unwrap(), &expected_coeff3);

        // Test case 4: Zero coefficient should return None
        let le4 = le!(0, 0, 2);
        let bound4 = le4.compute_bound_from(1);
        assert!(bound4.is_none());

        // Test case 5: Out of bounds should return None
        let le5 = le!(0, 1, 2);
        let bound5 = le5.compute_bound_from(3);
        assert!(bound5.is_none());
    }
}
