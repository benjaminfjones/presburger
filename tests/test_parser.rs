#[macro_use]
extern crate lalrpop_util;

#[cfg(test)]
mod test_parser {
    use presburger::ast::Term;
    use presburger::types::rbig;

    lalrpop_mod!(
        #[allow(clippy::all)]
        pub grammer
    ); // generated parser

    #[test]
    fn test_numlit() {
        let cases = vec!["222", "(222)", "((((222))))", "2/3", "(2/3)", "((-1/3))"];
        for c in cases {
            assert!(grammer::SumParser::new().parse(c).is_ok(), "case: {}", c);
        }
        assert!(grammer::SumParser::new().parse("((22)").is_err());
    }

    #[test]
    fn test_var() {
        let cases = vec!["x", "(x)", "((((y))))", "2 * z"];
        for c in cases {
            assert!(grammer::SumParser::new().parse(c).is_ok(), "case: {}", c);
        }
        // negative tests:
        assert!(grammer::SumParser::new().parse("7 * z * 7").is_err()); // TODO: not yet supported
        assert!(grammer::SumParser::new().parse("x * z").is_err()); // non-linear
    }

    #[test]
    fn test_sum() {
        let cases = vec![
            "x+1",
            "(x) + 1",
            "y + x + 1",
            "1/3 * x",
            "(1/3) * x",
            "(-1) * x + (-2/3)",
            "x + -5/1 * y + -1/-1 * z",
        ];
        for c in cases {
            assert!(grammer::SumParser::new().parse(c).is_ok(), "case: {}", c);
        }
    }

    #[test]
    fn test_good_atoms() {
        let cases = vec![
            "@T", // truth value
            "@F",
            "P", // logical variable
            "0 <= 1",
            "x+1 = y",
            "x <= y + 1",
            "y + x + 1 + z <= 0",
            "1/2 * x + 3 * y <= 0",
            "(-1) * x = 0",
        ];
        for c in cases {
            assert!(grammer::AtomParser::new().parse(c).is_ok(), "case: {}", c);
        }
    }

    #[test]
    fn test_bad_atoms() {
        let cases = vec![
            "@G",            // not a truth value
            "x+1 ? y",       // ? not a relation
            "(x+1) = y",     // parenthesized sums are not supported (not needed yet)
            "2 * (x+1) = y", // multiplying composite terms not supported (yet)
            "(x + 1) * y",   // non-linear
        ];
        for c in cases {
            assert!(grammer::AtomParser::new().parse(c).is_err(), "case: {}", c);
        }
    }

    ///  TODO: support all these cases
    #[test]
    fn test_atoms_not_yet_supported() {
        let cases = vec![
            "1/2 * x + 3 * y >= 0", // >= isn't supported yet, only <=
            "x > 0",                // strict ineq not supported yet
            "8 > 0",                // constant relations not supported
            "0 <= .123",            // decimals not supported
        ];
        for c in cases {
            assert!(grammer::AtomParser::new().parse(c).is_err(), "case: {}", c);
        }
    }

    #[test]
    fn test_pred() {
        let cases = vec![
            "@T /\\ @F",
            "@F \\/ @T",
            "P ==> Q",
            "P \\/ ~P",                 // Law of excluded middle
            "~(P \\/ Q) <=> ~P /\\ ~Q", // DeMorgan1
            "((P ==> Q) ==> P) ==> P",  // Pierce's Law
            "forall y. exists x. x = y \\/ x <= y",
            "forall y. x <= y ==> x <= y + 1",
            "(exists x. (1/2 <= x)) /\\ (forall y. 0 <= y /\\ 0 = y)",
        ];
        for c in cases {
            assert!(
                grammer::FormulaParser::new().parse(c).is_ok(),
                "case: {}",
                c
            );
        }
        // negative tests
        assert!(grammer::FormulaParser::new().parse("5 ==> x").is_err());
    }

    #[test]
    fn parse_big_rat() {
        let big_rat_good = "922337203685477580700 / 3"; // numerator is 100 * i64::MAX
        assert_eq!(
            grammer::SumParser::new().parse(big_rat_good),
            Ok(Term::Num(rbig!(922337203685477580700 / 3)))
        );

        // TODO: add negative parse_big_rat tests
    }
}
