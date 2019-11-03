use presburger::ast::{Term, Var};

fn main() {
    // let mut raw_input = String::new();
    // println!("Input a Presburger formula:");
    // io::stdin().read_line(&mut raw_input)
    //     .expect("bad formula!");

    // test bool term
    // let test_bool_term_input = String::from("1 + 1 < 3  /\\ 0 = 0");
    // test_parse(&test_bool_term_input[..]);

    // test quantification
    // let test_quant_input = String::from("exists x. x > 0");
    // test_parse(&test_quant_input[..]);

    let t1 = Term::Num(3);
    let t2 = Term::Var(Var(String::from("x")));
    println!("{:?} {:?}", t1, t2);
}

// fn test_parse(_input: &str) { }
