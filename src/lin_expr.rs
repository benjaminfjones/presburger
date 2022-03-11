/// Implementation of affine linear expressions and equality/inequality relations

// TODO: eventually replace with arbitrary precision integers from crate::rug
type Coeff = i64;

pub struct LinExpr {
    // Coefficient vector. The 0th element corresponds to the value of the
    // constant term; this is always present, but its value may be 0.
    //
    // Invariant: len(self.coeff) > 0
    coeff: Vec<Coeff>,
}

/// Affine integer-linear expression.
///
/// `LinExpr` are used to represent the right hand side of normalized affine linear
/// relations like equality and inequality with zero, e.g.
///
/// b + \sum_{i=1}^{n} a_i x_i = 0
///
/// or...
///
/// b + \sum_{i=1}^{n} a_i x_i \le 0
impl LinExpr {
    /// Create a new `LinExpr` from a slice of `Coeff`
    pub fn new(coeffs: &[Coeff]) -> Self {
        if coeffs.len() == 0 {
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
    pub fn coeff(&self, i: usize) -> Result<Coeff, ()> {
        if 1 <= i && i <= self.nvars() {
            Ok(self.coeff[i])
        } else {
            Err(())
        }
    }

    pub fn coeff_unchecked(&self, i: usize) -> Coeff {
        self.coeff[i]
    }

    /// Set the coefficient a_i
    pub fn set_coeff(&mut self, i: usize, value: Coeff) -> Result<(), ()> {
        if 1 <= i && i <= self.nvars() {
            self.coeff[i] = value;
            Ok(())
        } else {
            Err(())
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

pub struct LinEq(LinExpr);

/// Affine linear equation of the form: lin_expr = 0
impl LinEq {
    pub fn new(e: LinExpr) -> Self {
        LinEq(e)
    }

    pub fn nvars(&self) -> usize {
        self.0.nvars()
    }

    pub fn coeffs(&self) -> &[Coeff] {
        self.0.coeffs()
    }

    pub fn rhs(&self) -> &LinExpr {
        &self.0
    }

    // An equality is a possible substitution iff. some coeff = +-1.
    pub fn is_subs(&self) -> bool {
        self.0.coeffs().iter().any(|&c| c == 1 || c == -1)
    }

    // An equality is a possible substitution for x_i iff. coeff(x_i) = +-1.
    //
    // Returns `false` for variable indexes that are out of bounds.
    pub fn is_subs_for(&self, i: usize) -> bool {
        if let Ok(c) = self.0.coeff(i) {
            return c == 1 || c == -1;
        }
        false
    }

    /// Substitute a linear term for x_i using `other`, which must be a substitution equation
    /// (i.e. other.coeff(x_i) == Some(+-1).
    ///
    /// Because the result is a new equation, resulting from a deductive step, this method
    /// consumes `self` and returns a new owned equation.
    ///
    /// For example, Suppose vars are `[x_1, x_2]`,
    /// - `self` is  `3  x_1 + 4 x_2 = 0` and
    /// - `other` is `-3 x_1 +   x_2 + 2 x_3 = 0`,
    ///
    /// then substituting for `x_2 = 3 x_1 - 2 x_3` produces `(3 + 3 * 4) x_1 + (0 - 2 * 4) x_3 = 0`,
    /// equivalent to `15 x_1 + 8 x_3 = 0`.
    pub fn subs(self, i: usize, other: &Self) -> Result<Self, ()> {
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
                return Err(());
            }
            let n = self.nvars();
            assert_eq!(n, self.0.nvars());
            assert_eq!(n, other.0.nvars());

            let mut new_lhs = LinExpr::new_zeros(n);
            for j in 1..=n {
                new_lhs.set_coeff_unchecked(
                    j,
                    self.0.coeff_unchecked(j)
                        + m * other.0.coeff_unchecked(j) * self.0.coeff_unchecked(i),
                );
            }
            return Ok(LinEq(new_lhs));
        }
        Err(())
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
        assert_eq!(e1.coeff(1), Ok(0));
        assert_eq!(e1.coeff(2), Ok(1));

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
        assert_eq!(e1.coeff(2), Ok(1));
        assert_eq!(e1.coeff(3), Ok(3));
        assert!(e1.supported(3));

        let mut e2 = LinExpr::new_zeros(0);
        assert_eq!(e2.nvars(), 0);
        assert_eq!(e2.const_(), 0);
        assert_eq!(e2.coeff(1), Err(()));
        assert!(!e2.supported(1));
        e2.add_var(-1);
        assert_eq!(e2.nvars(), 1);
        assert_eq!(e2.const_(), 0);
        assert_eq!(e2.coeff(1), Ok(-1));
        assert!(e2.supported(1));
    }

    #[test]
    fn lin_eq_basic_api() {
        let eq1 = LinEq::new(LinExpr::new(&[0, 1, 2]));
        assert_eq!(eq1.nvars(), 2);
        assert!(eq1.is_subs());
        assert!(eq1.is_subs_for(1)); // coeff of x_1 is 1
        assert!(!eq1.is_subs_for(2)); // coeff of x_2 is 2
    }

    /// Suppose vars are `[x_1, x_2]`,
    /// - `self` is `3 x_1 + 4 x_2 = 0`
    /// - `other` is `-3 x_1 + x_2 = 0`,
    /// then substituting for `x_2 = 3 x_1` produces `15 x_1 = 0`
    #[test]
    fn lin_eq_subs_2() {
        let eq1 = LinEq::new(LinExpr::new(&[0, 3, 4]));
        let eq2 = LinEq::new(LinExpr::new(&[0, -3, 1]));
        assert_eq!(eq1.nvars(), 2);
        assert_eq!(eq2.nvars(), 2);
        let eq3 = eq1.subs(2, &eq2).expect("subs failed");
        assert_eq!(eq3.nvars(), 2);
        assert_eq!(eq3.coeffs(), &[15, 0]);
        assert!(!eq3.rhs().supported(2));
        // eq1 was moved: assert_eq!(eq1.nvars(), 2);
    }
}
