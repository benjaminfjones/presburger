/// Implementation of Fourier-Motzkin Elimination
/// https://en.wikipedia.org/wiki/Fourier%E2%80%93Motzkin_elimination


pub mod util {

    use std::ops::Rem;
    use crate::types::Coeff;
    use proptest::prelude::*;

    /// Symmetric modulo.
    ///
    /// a \hat{mod} b := a - b * \floor{a/b + 1/2}
    ///               := a - b * \floor{(2a + b)/2b}
    pub fn symmod(a: Coeff, b: Coeff) -> Coeff {
        a - b * div_floor(2*a + b, 2*b)
    }

    /// Symmetric modulo alternate definition which is equivalent to `symmod`.
    ///
    /// a \hat{mod} b := (a mod b) if a mod b < b/2 else ((a mod b) - b)
    pub fn symmod_alt(a: Coeff, b: Coeff) -> Coeff {
        let amb = a.rem_euclid(b);
        if 2*amb < b {
            amb
        } else {
            amb - b
        }
    }

    /// Integer quotient of `num` by `den`, rounding towards negative infinity
    pub fn div_floor(num: Coeff, den: Coeff) -> Coeff {
        let d = num / den;
        // this is intentionally `rem` and not `rem_euclid`
        let r = num.rem(den);
        if (r > 0 && den < 0) || (r < 0 && den > 0) {
            d - 1
        } else {
            d
        }
    }

    #[cfg(test)]
    mod test_util {
        use super::*;

        #[test]
        fn test_symmod() {
            assert_eq!(symmod(3, 4), -1);
            assert_eq!(symmod(1, 4), 1);
            assert_eq!(symmod(0, 4), 0);
        }

        #[test]
        fn test_symmod_alt() {
            assert_eq!(symmod_alt(3, 4), -1);
            assert_eq!(symmod_alt(1, 4), 1);
            assert_eq!(symmod_alt(0, 4), 0);
        }

        // proptest found bug: case a % b == b // 2 (int division) exposes the issue that
        // the test a < b/2 needs to be done over Q and not with integer division on the
        // RHS.
        #[test]
        fn test_regression_a_mod_b_vs_b_div_2() {
            let a = 1687;
            let b = 1125;
            assert_eq!(symmod(a, b), symmod_alt(a, b));
        }

        // proptest found bug: `%` is remainder, not euclidean remainder
        // left: `1`,
        // right: `-190459345`.
        // minimal failing input: a = -190459345, b = 190459346
        #[test]
        fn test_regression_b_is_minus_a_plus_one() {
            let a = -190459345;
            let b = 190459346;
            assert_eq!(symmod(a, b), symmod_alt(a, b));
        }

    }

    proptest! {
        /// Check that `symmod` and `symmod_alt` are equivalent. Random i32's
        /// are used since both functions overflow, in general, at values close
        /// to INT_MAX/INT_MIN.
        #[test]
        fn symmod_symmod_alt_equivalent(a: i32, b: i32) {
            prop_assume!(b > 0);
            let a64 = a as i64;
            let b64 = b as i64;
            assert_eq!(symmod(a64, b64), symmod_alt(a64, b64));
        }
    }
}
