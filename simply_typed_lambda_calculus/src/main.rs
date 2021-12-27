mod ast;
mod grammar;
mod typechecker;

use ast::Term;

fn parse(input: &str) -> Term {
  grammar::TermParser::new().parse(input).unwrap()
}

fn main() {
  dbg!(typechecker::infer(&parse("(λx: Int. x) 1")));
}
