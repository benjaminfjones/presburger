//! Implementation of Fourier-Motzkin Elimination
//! <https://en.wikipedia.org/wiki/Fourier%E2%80%93Motzkin_elimination>

pub mod util {

    use crate::types::Rational;
    use dashu::base::RemEuclid;
    use proptest::prelude::*;
    use std::ops::Rem;

    /// Symmetric modulo.
    ///
    /// a \hat{mod} b := a - b * \floor{a/b + 1/2}
    ///               := a - b * \floor{(2a + b)/2b}
    pub fn symmod(a: Rational, b: Rational) -> Rational {
        let df = div_floor(
            Rational::from(2) * a.clone() + b.clone(),
            Rational::from(2) * b.clone(),
        );
        a - b * df
    }

    /// Symmetric modulo alternate definition which is equivalent to `symmod`.
    ///
    /// a \hat{mod} b := (a mod b) if a mod b < b/2 else ((a mod b) - b)
    pub fn symmod_alt(a: Rational, b: Rational) -> Rational {
        let amb = a.rem_euclid(b.clone());
        if Rational::from(2) * amb.clone() < b {
            amb
        } else {
            amb - b
        }
    }

    /// Integer quotient of `num` by `den`, rounding towards negative infinity
    pub fn div_floor(num: Rational, den: Rational) -> Rational {
        let d = num.clone() / den.clone();
        // this is intentionally `rem` and not `rem_euclid`
        let r = num.rem(den.clone());
        if (r > Rational::ZERO && den < Rational::ZERO)
            || (r < Rational::ZERO && den > Rational::ZERO)
        {
            d - Rational::ONE
        } else {
            d
        }
    }

    #[cfg(test)]
    mod test_util {
        use super::*;

        #[test]
        #[ignore = "TODO: fixme"]
        fn test_symmod() {
            assert_eq!(symmod(3.into(), 4.into()), Rational::from(-1));
            assert_eq!(symmod(1.into(), 4.into()), 1.into());
            assert_eq!(symmod(0.into(), 4.into()), 0.into());
        }

        #[test]
        fn test_symmod_alt() {
            assert_eq!(symmod_alt(3.into(), 4.into()), Rational::from(-1));
            assert_eq!(symmod_alt(1.into(), 4.into()), 1.into());
            assert_eq!(symmod_alt(0.into(), 4.into()), 0.into());
        }

        // proptest found bug: case a % b == b // 2 (int division) exposes the issue that
        // the test a < b/2 needs to be done over Q and not with integer division on the
        // RHS.
        #[test]
        #[ignore = "TODO: fixme"]
        fn test_regression_a_mod_b_vs_b_div_2() {
            let a = Rational::from(1687);
            let b = Rational::from(1125);
            assert_eq!(symmod(a.clone(), b.clone()), symmod_alt(a, b));
        }

        // proptest found bug: `%` is remainder, not euclidean remainder
        // left: `1`,
        // right: `-190459345`.
        // minimal failing input: a = -190459345, b = 190459346
        #[test]
        #[ignore = "TODO: fixme"]
        fn test_regression_b_is_minus_a_plus_one() {
            let a = Rational::from(-190459345);
            let b = Rational::from(190459346);
            assert_eq!(symmod(a.clone(), b.clone()), symmod_alt(a, b));
        }
    }

    proptest! {
        /// Check that `symmod` and `symmod_alt` are equivalent. Random i32's
        /// are used since both functions overflow, in general, at values close
        /// to INT_MAX/INT_MIN.
        #[test]
        #[ignore = "need to use rationals as inputs"]
        fn symmod_symmod_alt_equivalent(a: i32, b: i32) {
            prop_assume!(b > 0);
            let a64 = Rational::from(a);
            let b64 = Rational::from(b);
            assert_eq!(symmod(a64.clone(), b64.clone()), symmod_alt(a64, b64));
        }
    }
}
