#[cfg(test)]
mod test_fme_expr {
    use presburger::fme::expr;

    #[test]
    fn test_expr_support() {
        let e1 = expr::LExpr::new(&[1,0,1]);
        assert!(e1.supported(0));
        assert!(!e1.supported(1));
        assert!(e1.supported(2));

        // out of bounds
        assert!(!e1.supported(3));
    }
}
