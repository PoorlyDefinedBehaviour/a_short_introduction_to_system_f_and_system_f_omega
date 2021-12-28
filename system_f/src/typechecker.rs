use crate::ast::{Term, Type};
use thiserror::Error;

// Γ(x) = τ
// --------
// Γ ⊢ x: τ
//
//
// Γ ⊢ t1: σ -> τ    Γ ⊢ t2: σ
// ---------------------------
//       Γ ⊢ (t1 t2): τ
//
//
//    Γ, x: σ ⊢ t: τ
// ----------------------
// Γ ⊢ (λx: σ. t): σ -> τ
//
//
// ------------------------
// (λx: τ. t) t' |> t[t'/x]
//
//
// Γ(α) = κ
// ---------
// Γ ⊢ α : κ
//
//
// Γ ⊢ σ    Γ ⊢ τ
// --------------
//   Γ ⊢ σ -> τ
//
//
//  Γ, α : κ ⊢ σ
// --------------
// Γ ⊢ ∀α : κ . σ
//
//
//      Γ, α : κ ⊢ t : τ
// ----------------------------
// Γ ⊢ (Λα : κ. t) : (∀α : κ . τ)
//
//
// Γ ⊢ t : (∀α : κ . τ)   Γ ⊢ σ
// ----------------------------
//      Γ ⊢ t σ : τ[σ/α]
//
// -------------------------
// (Λα : κ . t) σ |> t[σ/α]
//
//
// ------
// t |> t
//
//
//    t1 |> t1'
// ---------------
// t1 t2 |> t1' t2
//
//
//   t2 |> t2'
// ------------
// x t2 |> x t2'
//
//
// t1 |> t2   t2 |> t3
// -------------------
//      t1 |> t3
#[derive(Debug, PartialEq, Error)]
pub enum TypecheckerError {
  #[error("variable {0} is not defined")]
  UndefinedVariable(String),
  #[error("expected term {term:?} to have type {expected:?} but it has type {got:?}")]
  TypeMismatch {
    term: Term,
    expected: Type,
    got: Type,
  },
  #[error("expected {expected:?} but got {got:?}")]
  UnexpectedTerm { expected: String, got: Term },
}

#[derive(Debug, Clone)]
enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

#[derive(Debug)]
struct TypingContext {
  type_assignments: List<(String, Type)>,
}

impl TypingContext {
  pub fn new() -> Self {
    Self {
      type_assignments: List::Nil,
    }
  }

  pub fn assign(&self, var: String, typ: Type) -> TypingContext {
    TypingContext {
      type_assignments: List::Cons((var, typ), Box::new(self.type_assignments.clone())),
    }
  }

  fn get_var(xs: &List<(String, Type)>, var: &String) -> Option<Type> {
    match xs {
      List::Nil => None,
      List::Cons((x, typ), tail) => {
        if x == var {
          Some(typ.clone())
        } else {
          Self::get_var(tail, var)
        }
      }
    }
  }

  pub fn get(&self, var: &String) -> Option<Type> {
    Self::get_var(&self.type_assignments, var)
  }
}

fn subst(type_var: &String, from: &Type, to: Type) -> Type {
  match from {
    Type::Int => Type::Int,
    Type::Arrow(param_type, return_type) => Type::Arrow(
      Box::new(subst(type_var, param_type, to.clone())),
      Box::new(subst(type_var, return_type, to)),
    ),
    Type::TypeVar(x) => {
      // TODO: use capture avoiding substitution.
      if x == type_var {
        to
      } else {
        from.clone()
      }
    }
    Type::Forall {
      param_type,
      return_type,
      kind,
      type_var,
    } => Type::Forall {
      param_type: param_type.clone(),
      kind: kind.clone(),
      type_var: type_var.clone(),
      return_type: Box::new(subst(&type_var, &return_type, to)),
    },
  }
}

fn type_of(ctx: &TypingContext, term: &Term) -> Result<Type, TypecheckerError> {
  match term {
    Term::Int(_) => Ok(Type::Int),
    // Γ(x) = τ
    // --------
    // Γ ⊢ x: τ
    Term::Var(x) => match ctx.get(x) {
      None => Err(TypecheckerError::UndefinedVariable(x.clone())),
      Some(typ) => Ok(typ.clone()),
    },
    // Γ ⊢ t1: σ -> τ    Γ ⊢ t2: σ
    // ---------------------------
    //       Γ ⊢ (t1 t2): τ
    Term::App(f, arg) => match &**f {
      Term::Abs {
        param_name,
        param_type,
        body,
      } => {
        let arg_type = type_of(ctx, arg)?;

        if param_type != &arg_type {
          return Err(TypecheckerError::TypeMismatch {
            term: *arg.clone(),
            expected: param_type.clone(),
            got: arg_type,
          });
        }

        let ctx = ctx.assign(param_name.clone(), param_type.clone());

        let body_typ = type_of(&ctx, body)?;

        Ok(body_typ)
      }
      _ => Err(TypecheckerError::UnexpectedTerm {
        expected: String::from("abstraction"),
        got: *f.clone(),
      }),
    },
    //    Γ, x: σ ⊢ t: τ
    // ----------------------
    // Γ ⊢ (λx: σ. t): σ -> τ
    Term::Abs {
      param_name,
      param_type,
      body,
    } => {
      let ctx = ctx.assign(param_name.clone(), param_type.clone());
      let body_typ = type_of(&ctx, body)?;
      Ok(Type::Arrow(
        Box::new(param_type.clone()),
        Box::new(body_typ),
      ))
    }
    Term::UniversalAbs {
      type_var,
      kind,
      body,
    } => Ok(Type::Forall {
      type_var: type_var.clone(),
      kind: kind.clone(),
      param_type: Box::new(Type::TypeVar(type_var.clone())),
      return_type: Box::new(type_of(ctx, body)?),
    }),
    // Γ ⊢ t : (∀α : κ . τ)   Γ ⊢ σ
    // ----------------------------
    //      Γ ⊢ t σ : τ[σ/α]
    Term::UniversalApp(term, typ) => match type_of(ctx, term)? {
      Type::Forall {
        return_type,
        type_var,
        ..
      } => {
        // (\x. e')[v/x] = \x. e' -- we do not substitute because x is bound by the lambda
        // (\y. e')[v/x] = (\y. e'[v/x]) -- recursively substitute lambda body
        // (\y. x)[z/x] = (\y. z) -- replace x with z
        // (\z. x)[z/x] = (\z. x) -- do not substitute x because it would change the function behaviour
        Ok(subst(&type_var, &return_type, typ.clone()))
      }
      _ => Err(TypecheckerError::UnexpectedTerm {
        expected: String::from("type abstraction"),
        got: *term.clone(),
      }),
    },
  }
}

pub fn infer(term: &Term) -> Result<Type, TypecheckerError> {
  type_of(&TypingContext::new(), term)
}
