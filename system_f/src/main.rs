mod ast;
mod grammar;
mod typechecker;

use ast::Term;

fn parse(input: &str) -> Term {
  grammar::TermParser::new().parse(input).unwrap()
}

fn main() {
  let e = "(ΛX: * . λx: X. x) [Bool]";
  dbg!(typechecker::infer(&parse(e)));
}
