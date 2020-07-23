#[macro_use] extern crate lalrpop_util;

use std::io;

use presburger::{nnf};
lalrpop_mod!(pub grammer); // generated parser

fn main() {
    let mut raw_input = String::new();
    println!("Input a Presburger formula:");
    io::stdin().read_line(&mut raw_input)
        .expect("input failed!");

    let p1 = grammer::PredParser::new().parse(&raw_input).unwrap();
    println!("p1: {:?}", p1);

    let p2 = nnf::to_nnf(p1);
    println!("{:?}", p2);
}
