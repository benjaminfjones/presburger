extern crate presburger;

use presburger::ast;
use rand::prelude::*;

/// Return a random Pred of depth at most `size`.
fn random_pred(size: usize) -> ast::Pred {
    if size == 0 {
        return ast::Pred::atom(random_atom(0))
    } else {
        let mut rng = thread_rng();
        return match rng.gen_range(0, 8) {
            0 => ast::Pred::not(random_pred(size-1)),
            1 => ast::Pred::and(random_pred(size-1), random_pred(size-1)),
            2 => ast::Pred::or(random_pred(size-1), random_pred(size-1)),
            3 => ast::Pred::implies(random_pred(size-1), random_pred(size-1)),
            4 => ast::Pred::iff(random_pred(size-1), random_pred(size-1)),
            5 => ast::Pred::exists(ast::Var::new(&random_var_name(rng)), random_pred(size-1)),
            6 => ast::Pred::forall(ast::Var::new(&random_var_name(rng)), random_pred(size-1)),
            _ => ast::Pred::atom(random_atom(size-1)),
        }
    }
}

/// Return a random ast::Atom of depth at most `size`.
fn random_atom(size: usize) -> ast::Atom {
    let mut rng = thread_rng();
    let select: usize;
    if size == 0 {
        select = rng.gen_range(2, 4);
    } else {
        select = rng.gen_range(0, 4);
    }
    match select {
        0 => ast::Atom::equality(random_term(size-1), random_term(size-1)),
        1 => ast::Atom::less_eq(random_term(size-1), random_term(size-1)),
        2 => ast::Atom::truth(random()),
        _ => ast::Atom::var(&random_var_name(rng)),
    }
}

/// Return a random ast::Term of depth at most `size`.
fn random_term(size: usize) -> ast::Term {
    let mut rng = thread_rng();
    let select: usize;
    if size == 0 {
        select = rng.gen_range(0, 2);
    } else {
        select = rng.gen_range(0, 3);
    }
    match select {
        0 => ast::Term::num(rng.gen()),
        1 => ast::Term::var(&random_var_name(rng)),
        _ => ast::Term::add(random_term(size-1), random_term(size-1)),
    }
}

/// Return a random variable name between 'a' ... 'z'
fn random_var_name(rng: ThreadRng) -> String {
    let mut rng = rng;
    (b'a' ..= b'z').map(char::from).choose(&mut rng).unwrap().to_string()
}

#[test]
fn make_random_pred() {
    let p = random_pred(5);
    println!("{:?}", p);
    assert_eq!(p, p);
}

// TODO: make a bunch of large random predicates, convert to nnf, and profile memory usage