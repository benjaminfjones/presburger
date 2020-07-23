#[macro_use] extern crate lalrpop_util;

#[cfg(test)]
mod test_parser {

    lalrpop_mod!(pub grammer); // generated parser

    #[test]
    fn test_numlit() {
        let cases = vec![
            "222",
            "(222)",
            "((((222))))",
        ];
        for c in cases {
            assert!(grammer::TermParser::new().parse(c).is_ok(), "case: {}", c);
        }
        assert!(grammer::TermParser::new().parse("((22)").is_err());
    }

    #[test]
    fn test_var() {
        let cases = vec![
            "x",
            "(x)",
            "((((y))))",
        ];
        for c in cases {
            assert!(grammer::TermParser::new().parse(c).is_ok(), "case: {}", c);
        }
        // negative tests:
        assert!(grammer::TermParser::new().parse("2z").is_err());
    }

    #[test]
    fn test_term() {
        let cases = vec![
            "5",
            "y",
            "x+1",
            "(x) + 1",
            "(x + 1)",
            "y + x + 1",
            "y + (x + 1) + z",
        ];
        for c in cases {
            assert!(grammer::TermParser::new().parse(c).is_ok(), "case: {}", c);
        }
    }

    #[test]
    fn test_atom() {
        let cases = vec![
            "T",
            "F",
            "x+1 = y",
            "x <= y + 1",
            "y + (x + 1) + z <= 0",
        ];
        for c in cases {
            assert!(grammer::AtomParser::new().parse(c).is_ok(), "case: {}", c);
        }
        // negative tests
        assert!(grammer::AtomParser::new().parse("y + (x + 1) + z > 0").is_err());

    }

    #[test]
    fn test_pred() {
        let cases = vec![
            "T /\\ F",
            "F \\/ T",
            "P ==> Q",
            "P \\/ ~P",                  // Law of excluded middle
            "~(P \\/ Q) <=> ~P /\\ ~Q",  // DeMorgan1
            "((P ==> Q) ==> P) ==> P",   // Pierce's Law
            "forall y. exists x. x = y \\/ x <= y",
            "forall y. x <= y ==> x <= y + 1",
            "(exists x. 1 <= x) /\\ (forall y. 0 <= y /\\ 0 = y)",
        ];
        for c in cases {
            assert!(grammer::PredParser::new().parse(c).is_ok(), "case: {}", c);
        }
        // negative tests
        assert!(grammer::PredParser::new().parse("5 ==> x").is_err());
    }
}
