//! Implementation of affine linear expressions and equality/inequality relations

use crate::types::Coeff;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LinExprError {
    /// Custom error type returned when linear expression variable indices
    /// are out of bounds.
    IndexOutOfBounds,
    /// Custom error type returned when an linear expression, equality, or
    /// inequality assertion is violated.
    AssertionError,
}

impl fmt::Display for LinExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexOutOfBounds => {
                write!(f, "Coefficient index out of bounds")
            }
            Self::AssertionError => {
                write!(f, "Assertion error")
            }
        }
    }
}

impl Error for LinExprError {}

/// Affine integer-linear expression.
///
/// `LinExpr` are used to represent the left hand side of normalized affine linear
/// relations like equality and inequality with zero, e.g.
///
/// b + \sum_{i=1}^{n} a_i x_i = 0
///
/// or...
///
/// b + \sum_{i=1}^{n} a_i x_i \le 0
#[derive(Debug)]
pub struct LinExpr {
    // Coefficient vector. The 0th element corresponds to the value of the
    // constant term; this is always present, but its value may be 0.
    //
    // Invariant: len(self.coeff) > 0
    coeff: Vec<Coeff>,
}

impl PartialEq for LinExpr {
    /// Custom Eq allows correct comparison of
    /// linear expressions even if the underlying arrays of
    /// coefficients are different length (e.g. additional variables were
    /// added to one of them)
    ///
    /// Example:
    ///
    /// ```
    /// # use presburger::lin_expr::*;
    /// # fn main () {
    /// // Equalities with same and different nvars
    /// let e0 = LinExpr::new(&vec![0i64, 1, 0]);
    /// let e1 = LinExpr::new(&vec![0i64, 1]);
    /// assert_eq!(e0, e0);
    /// assert_eq!(e0, e1);
    /// assert_eq!(e1, e0);
    ///
    /// // x_1 != x_1 + 2 x_2, representations w/ same nvars
    /// // x_1 != x_1 + 2 x_2, representations w/ different nvars
    /// let e2 = LinExpr::new(&vec![0i64, 1, 2]);
    /// assert_ne!(e0, e2);
    /// assert_ne!(e1, e2);
    ///
    /// // x_1 + 2 x_2 != -1 + x1 + 2 x2
    /// let e3 = LinExpr::new(&vec![-1i64, 1, 2]);
    /// assert_ne!(e2, e3);
    /// # }
    ///
    /// ```
    ///
    fn eq(&self, other: &Self) -> bool {
        if self.const_() != other.const_() {
            return false;
        }
        // Compare lengths of self and other after monomials with coeff
        // zero are truncated from the end.
        let sc = self.coeffs();
        let oc = other.coeffs();
        match sc.len().cmp(&oc.len()) {
            Ordering::Less => oc[sc.len()..].iter().all(|a| *a == 0) && sc == &oc[..sc.len()],
            Ordering::Equal => sc == oc,
            Ordering::Greater => sc[oc.len()..].iter().all(|a| *a == 0) && &sc[..oc.len()] == oc,
        }
    }
}

impl Eq for LinExpr {}

/// Display the expression with variables ordered and only monomials with
/// non-zero coefficient.
///
/// Example:
///
/// ```
/// # use presburger::lin_expr::*;
/// # fn main () {
/// let e0 = LinExpr::new(&vec![0i64, 1, 0, 2]);
/// assert_eq!(e0.to_string(), "1 x_1 + 2 x_3");
///
/// let e1 = LinExpr::new(&vec![5i64, 0, 0, -10]);
/// assert_eq!(e1.to_string(), "5 + (-10) x_3");
/// # }
/// ```
impl fmt::Display for LinExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut term_vec = Vec::new();
        if self.const_() != 0 {
            term_vec.push(format!("{}", self.const_()));
        }
        let coeffs = self.coeffs();
        for (i, a) in coeffs.iter().enumerate() {
            if *a > 0 {
                term_vec.push(format!("{} x_{}", a, i + 1));
            } else if *a < 0 {
                term_vec.push(format!("({}) x_{}", a, i + 1));
            }
        }
        write!(f, "{}", term_vec.join(" + "))
    }
}

impl LinExpr {
    /// Create a new `LinExpr` from a slice of `Coeff`
    pub fn new(coeffs: &[Coeff]) -> Self {
        if coeffs.is_empty() {
            panic!("coefficient array must be non-empty")
        }
        Self {
            coeff: coeffs.to_owned(),
        }
    }

    /// Create a new zero `LinExpr` with given number of variables
    pub fn new_zeros(nvars: usize) -> Self {
        Self {
            coeff: vec![0; nvars + 1],
        }
    }

    /// If `self` is an expression over x_1 ... x_n, then add new variable x_{n+1}
    /// with coefficient `value`.
    pub fn add_var(&mut self, value: Coeff) {
        self.coeff.push(value);
    }

    /// Total number of variables in the expression, including those w/ coeff 0
    pub fn nvars(&self) -> usize {
        self.coeff.len() - 1
    }

    /// Get the coefficient a_i
    pub fn coeff(&self, i: usize) -> Result<Coeff, LinExprError> {
        if 1 <= i && i <= self.nvars() {
            Ok(self.coeff[i])
        } else {
            Err(LinExprError::IndexOutOfBounds)
        }
    }

    pub fn coeff_unchecked(&self, i: usize) -> Coeff {
        self.coeff[i]
    }

    /// Set the coefficient a_i
    pub fn set_coeff(&mut self, i: usize, value: Coeff) -> Result<(), LinExprError> {
        if 1 <= i && i <= self.nvars() {
            self.coeff[i] = value;
            Ok(())
        } else {
            Err(LinExprError::IndexOutOfBounds)
        }
    }

    /// Set the coefficient a_i
    pub fn set_coeff_unchecked(&mut self, i: usize, value: Coeff) {
        self.coeff[i] = value;
    }

    /// Get a slice of the variable coefficients a_1 ... a_n
    pub fn coeffs(&self) -> &[Coeff] {
        &self.coeff[1..]
    }

    /// Get a mutable slice of the variable coefficients a_1 ... a_n
    pub fn coeffs_mut(&mut self) -> &mut [Coeff] {
        &mut self.coeff[1..]
    }

    /// Get the constant term
    pub fn const_(&self) -> Coeff {
        self.coeff[0]
    }

    /// Set the constant term
    pub fn set_const(&mut self, value: Coeff) {
        self.coeff[0] = value;
    }

    /// Is the variable x_i in the support, i.e. a_i != 0?
    pub fn supported(&self, i: usize) -> bool {
        if 1 <= i && i <= self.nvars() {
            self.coeff[i] != 0
        } else {
            false
        }
    }
}

/// Represents `LinExp == 0`
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

    pub fn from_coeffs(coeffs: &[Coeff]) -> Self {
        LinEq(LinExpr::new(coeffs))
    }

    pub fn nvars(&self) -> usize {
        self.0.nvars()
    }

    pub fn coeffs(&self) -> &[Coeff] {
        self.0.coeffs()
    }

    pub fn const_(&self) -> Coeff {
        self.0.const_()
    }

    pub fn lhs(&self) -> &LinExpr {
        &self.0
    }

    /// An equality is a possible substitution iff. some coeff == +-1.
    /// Return the position of the first substitution coefficient, or None.
    pub fn is_subs(&self) -> Option<usize> {
        self.0
            .coeffs()
            .iter()
            .position(|&c| c == 1 || c == -1)
            .map(|i| i + 1)
    }

    /// An equality is a possible substitution for x_i iff. coeff(x_i) == +-1.
    ///
    /// Returns `false` for variable indexes that are out of bounds.
    pub fn is_subs_for(&self, i: usize) -> bool {
        let Ok(c) = self.0.coeff(i) else { return false };
        c == 1 || c == -1
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
    /// # fn main () -> Result<(), LinExprError> {
    /// let eq = LinEq::new(LinExpr::new(&vec![0, 3, 4, 0]));
    /// let other = LinEq::new(LinExpr::new(&vec![0, -3, 1, 2]));
    /// let res = eq.subs(2, &other)?;
    /// assert_eq!(res, LinEq::new(LinExpr::new(&vec![0, 15, 0, -8])));
    /// # Ok(())
    /// # }
    /// ```
    pub fn subs(self, i: usize, other: &Self) -> Result<Self, LinExprError> {
        let n = self.nvars();
        assert_eq!(n, self.0.nvars());
        assert_eq!(n, other.0.nvars());
        if let Ok(a) = other.0.coeff(i) {
            let m: Coeff;
            if a == 1 {
                // if coeff is 1, subtract other's coeffs from self
                m = -1;
            } else if a == -1 {
                // if coeff is -1, add other's coeffs to self
                m = 1;
            } else {
                // substitution for this variable isn't valid
                return Err(LinExprError::AssertionError);
            }
            // Safe b/c nvars other == nvars self and we know other variable i
            // is valid
            let se_coeff = self.0.coeff_unchecked(i);

            let mut new_lhs = LinExpr::new_zeros(n);
            for j in 1..=n {
                new_lhs.set_coeff_unchecked(
                    j,
                    self.0.coeff_unchecked(j) + m * other.0.coeff_unchecked(j) * se_coeff,
                );
            }
            new_lhs.set_const(self.0.const_() + m * other.0.const_() * se_coeff);
            return Ok(LinEq(new_lhs));
        }
        Err(LinExprError::IndexOutOfBounds)
    }
}

#[cfg(test)]
mod test_expr_support {
    use super::*;

    #[test]
    fn lin_expr_basic_api() {
        let e1 = LinExpr::new(&[1, 0, 1]);
        assert_eq!(e1.nvars(), 2);
        assert_eq!(e1.const_(), 1);
        assert_eq!(e1.coeff(1).unwrap(), 0);
        assert_eq!(e1.coeff(2).unwrap(), 1);

        assert!(!e1.supported(1));
        assert!(e1.supported(2));

        // out of bounds
        assert!(!e1.supported(0)); // index 0 is the constant
        assert!(!e1.supported(3));
    }

    #[test]
    fn lin_expr_add_var() {
        let mut e1 = LinExpr::new(&[1, 0, 1]);
        e1.add_var(3);
        assert_eq!(e1.nvars(), 3);
        assert_eq!(e1.coeff(2).unwrap(), 1);
        assert_eq!(e1.coeff(3).unwrap(), 3);
        assert!(e1.supported(3));

        let mut e2 = LinExpr::new_zeros(0);
        assert_eq!(e2.nvars(), 0);
        assert_eq!(e2.const_(), 0);
        assert!(e2.coeff(1).is_err());
        assert!(!e2.supported(1));
        e2.add_var(-1);
        assert_eq!(e2.nvars(), 1);
        assert_eq!(e2.const_(), 0);
        assert_eq!(e2.coeff(1).unwrap(), -1);
        assert!(e2.supported(1));
    }

    #[test]
    fn lin_eq_basic_api() {
        let eq1 = LinEq::new(LinExpr::new(&[0, 1, 2]));
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
        let eq1 = LinEq::new(LinExpr::new(&[0, 3, 4]));
        let eq2 = LinEq::new(LinExpr::new(&[0, -3, 1]));
        assert_eq!(eq1.nvars(), 2);
        assert_eq!(eq2.nvars(), 2);
        let eq3 = eq1.subs(2, &eq2).expect("subs failed");
        assert_eq!(eq3.nvars(), 2);
        assert_eq!(eq3.coeffs(), &[15, 0]);
        assert_eq!(eq3.const_(), 0);
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
        let eq1 = LinEq::from_coeffs(&[0, 3, 4, 0]);
        let eq2 = LinEq::from_coeffs(&[0, -3, 1, 2]);
        let eq3 = eq1.subs(2, &eq2).expect("subs failed");
        assert_eq!(eq3.nvars(), 3);
        assert_eq!(eq3.coeffs(), &[15, 0, -8]);
        assert_eq!(eq3.const_(), 0);
        assert!(!eq3.lhs().supported(2));
    }

    // Substitution with non-zero constants
    // - self  is -1 + 3 x_1 + 5 x_2 = 0
    // - other is 7  -   x_1 + x_2   = 0
    //
    // Using other to substitute for x_1 in self leaves 20 + 8 x_2 = 0
    #[test]
    fn lin_eq_subs_const() {
        let eq1 = LinEq::from_coeffs(&[-1, 3, 5]);
        let eq2 = LinEq::from_coeffs(&[7, -1, 1]);
        let eq3 = eq1.subs(1, &eq2).expect("subs failed");
        assert_eq!(eq3.coeffs(), &[0, 8]);
        assert_eq!(eq3.const_(), 20);
        assert!(!eq3.lhs().supported(1));
        assert!(eq3.lhs().supported(2));
    }
}
