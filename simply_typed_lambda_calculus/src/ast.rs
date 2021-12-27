// Type ::=
//  | *                   -- base type
//  | Type -> Type        -- function type
//  | (Type)              -- grouping
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  // *
  Int,
  // Type -> Type
  Arrow(Box<Type>, Box<Type>),
}

// Term ::=
//  | Int                 -- integer literal
//  | Var                 -- term variable
//  | Term Term           -- term application
//  | λ Var : Type . Term -- term abstraction
//  | (Term)              -- grouping
#[derive(Debug)]
pub enum Term {
  Int(i32),
  // Var
  Var(String),
  // Term Term
  App(Box<Term>, Box<Term>),
  // λ Var: Type. Term
  Abs {
    param_name: String,
    param_type: Type,
    body: Box<Term>,
  },
}
