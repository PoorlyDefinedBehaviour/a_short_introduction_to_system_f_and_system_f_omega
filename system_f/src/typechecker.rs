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
// Γ ⊢ (Λα : κ. t) : ∀α : κ . τ
//
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

        if &arg_type != param_type {
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
  }
}

pub fn infer(term: &Term) -> Result<Type, TypecheckerError> {
  type_of(&TypingContext::new(), term)
}
