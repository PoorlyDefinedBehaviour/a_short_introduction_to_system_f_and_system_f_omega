// Kind ::=
//  | *
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
  // *
  Star,
}

// Type ::=
//  | *                   -- base type
//  | Type -> Type        -- function type
//  | (Type)              -- grouping
//  | TypeVar
//  | ∀ TypeVar : Kind . Type -> Type
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  // *
  Int,
  // Type -> Type
  Arrow(Box<Type>, Box<Type>),
  // TypeVar
  TypeVar(String),
  // ∀ TypeVar : Kind . Type -> Type
  Forall {
    type_var: String,
    kind: Kind,
    param_type: Box<Type>,
    return_type: Box<Type>,
  },
}

// Term ::=
//  | Int                     -- integer literal
//  | Var                     -- term variable
//  | Term Term               -- term application
//  | λ Var : Type . Term     -- term abstraction
//  | (Term)                  -- grouping
//  | Λ TypeVar : Kind . Term -- universal abstraction
//  | Term Type               -- universal application
#[derive(Debug, Clone, PartialEq)]
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
  // Λ TypeVar : Kind . Term
  UniversalAbs {
    type_var: String,
    kind: Kind,
    body: Box<Term>,
  },
  // Term Type
  UniversalApp(Box<Term>, Type),
}
