//! Misc useful macros, mostly for concise test code

/// Match a list of integers in parentheses, construct an equality relation from
/// them interpreted as const-coefficients
///
/// For example, `eq!(1,2,3)` represents `1 + 2x1 + 3x2 = 0`
#[macro_export]
macro_rules! eq {
    ($($int:expr),+) => {
        LinRel::mk_eq(LinExpr::new(vec![$($int),+]).unwrap())
    };
}

/// Match a list of integers in parentheses, construct a less than or equal to relation from
/// them interpreted as const-coefficients
///
/// For example, `le!(1,2,3)` represents `1 + 2x1 + 3x2 <= 0`
#[macro_export]
macro_rules! le {
    ($($int:expr),+) => {
        LinRel::mk_le(LinExpr::new(vec![$($int),+]).unwrap())
    };
}
