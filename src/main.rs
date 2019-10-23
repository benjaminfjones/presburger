extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;
// use std::io;

#[derive(Parser,Debug)]
#[grammar = "presburger.peg"]
pub struct PBParser;

fn main() {
    // println!("Input a Presburger formula:");
    // io::stdin().read_line(&mut raw_input)
    //     .expect("bad formula!");

    // test bool term
    let test_bool_term_input = String::from("1 + 1 < 3  /\\ 0 = 0");
    test_parse(&test_bool_term_input[..]);

    // test quantification
    let test_quant_input = String::from("exists x. x > 0");
    test_parse(&test_quant_input[..]);
}

fn test_parse(input: &str) {
    let preformula: Pair<Rule> = PBParser::parse(Rule::formula, &input)
        .expect("unsuccessful parse")
        .next().unwrap();

    let top_pair = preformula.into_inner().next().unwrap();
    ast_from_preformula(&top_pair);
}

fn ast_from_preformula(p: &Pair<Rule>) {
    match p.as_rule() {
        Rule::quantification => {
            println!("found quantification!");
        },
        Rule::bool_term => {
            println!("found bool term!");
        },
        _ => {
            panic!("not implemented {:?}", p.as_rule());
        },
    }
}
