/// Implementation of linear expressions and relations for the FME solver.

// use rug::{Assign, Rational};

type Coeff = i64;

pub struct LExpr {
    coeff: Vec<Coeff>
}

impl LExpr {
    pub fn new(coeffs: &[Coeff]) -> Self {
        Self { coeff: coeffs.to_owned() }
    }

    pub fn nvars(&self) -> usize {
        self.coeff.len()
    }

    pub fn supported(&self, index: usize) -> bool {
        matches!(self.coeff.get(index), Some(&x) if x > 0)
    }
}

pub enum LRel {
    Eq(LExpr, LExpr),
    LessEq(LExpr, LExpr),
}

impl LRel {
    pub fn mk_eq(lhs: LExpr, rhs: LExpr) -> Self {
        LRel::Eq(lhs, rhs)
    }

    pub fn mk_lesseq(lhs: LExpr, rhs: LExpr) -> Self {
        LRel::LessEq(lhs, rhs)
    }
}
