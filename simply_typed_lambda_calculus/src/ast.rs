// Type ::=
//  | *                   -- base type
//  | Type -> Type        -- function type
//  | (Type)              -- grouping
pub enum Type {
  // Type -> Type
  Arrow(Box<Type>, Box<Type>),
}

// Term ::=
//  | Var                 -- term variable
//  | Term Term           -- term application
//  | λ Var : Type . Term -- term abstraction
//  | (Term)              -- grouping
pub enum Term {
  // Var
  Var(String),
  // Term Term
  App {
    Abs: Box<Term>,
    Arg: Box<Term>,
  },
  // λ Var: Type. Term
  Abs {
    param_name: String,
    param_type: Type,
    body: Box<Term>,
  },
}
