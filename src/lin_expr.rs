//! Implementation of affine linear expressions and equality/inequality relations

use crate::types::Coeff;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LinExprError {
    /// Coefficients are invalid (e.g. empty)
    CoeffInvalid,
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
            Self::CoeffInvalid => {
                write!(f, "Coefficients are invalid")
            }
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
    /// let e0 = LinExpr::new(&vec![0i64, 1, 0]).unwrap();
    /// let e1 = LinExpr::new(&vec![0i64, 1]).unwrap();
    /// assert_eq!(e0, e0);
    /// assert_eq!(e0, e1);
    /// assert_eq!(e1, e0);
    ///
    /// // x_1 != x_1 + 2 x_2, representations w/ same nvars
    /// // x_1 != x_1 + 2 x_2, representations w/ different nvars
    /// let e2 = LinExpr::new(&vec![0i64, 1, 2]).unwrap();
    /// assert_ne!(e0, e2);
    /// assert_ne!(e1, e2);
    ///
    /// // x_1 + 2 x_2 != -1 + x1 + 2 x2
    /// let e3 = LinExpr::new(&vec![-1i64, 1, 2]).unwrap();
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
/// let e0 = LinExpr::new(&vec![0i64, 1, 0, 2]).unwrap();
/// assert_eq!(e0.to_string(), "1 x_1 + 2 x_3");
///
/// let e1 = LinExpr::new(&vec![5i64, 0, 0, -10]).unwrap();
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
    pub fn new(coeffs: &[Coeff]) -> Result<Self, LinExprError> {
        if coeffs.is_empty() {
            Err(LinExprError::CoeffInvalid)
        } else {
            Ok(Self {
                coeff: coeffs.to_owned(),
            })
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

#[cfg(test)]
mod test_expr_support {
    use super::*;

    #[test]
    fn lin_expr_basic_api() {
        let e1 = LinExpr::new(&[1, 0, 1]).expect("failed to create linear expression");
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
        let mut e1 = LinExpr::new(&[1, 0, 1]).expect("failed to create linear expression");
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
}
