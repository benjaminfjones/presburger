// build script that runs the LALRPOP preprocessor

extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
