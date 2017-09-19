//! Typechecking for our DDL
//!
//! # Syntax
//!
//! ## Kinds
//!
//! ```plain
//! κ ::=
//!         Type        kind of types
//! ```
//!
//! ## Expressions
//!
//! ```plain
//! e ::=
//!         x           variables
//!         n           integer number
//!         true        true value
//!         false       false value
//!         -e          negation
//!         ¬e          not
//!         e₁ ∨ e₂     disjunction
//!         e₁ ∧ e₂     conjunction
//!         e₁ = e₂     equality
//!         e₁ ≠ e₂     inequality
//!         e₁ < e₂     less than
//!         e₁ ≤ e₂     less than or equal
//!         e₁ > e₂     greater than
//!         e₁ ≥ e₂     greater than or equal
//!         e₁ + e₂     addition
//!         e₁ - e₂     subtraction
//!         e₁ * e₂     multiplication
//!         e₁ / e₂     division
//! ```
//!
//! ## Types
//!
//! ```plain
//! τ ::=
//!         c                   type constants
//!         α                   variables
//!         τ₁ + τ₂             sum
//!         Σ x:τ₁ .τ₂          dependent pair
//!         [τ; e]              array
//!         { x:τ | e }         constrained type
//! ```
//!
//! In the `ast`, we represent the above as the following:
//!
//! - `Type::Var`: variables
//!
//! - `Type::Union`: series of unions
//!
//! - `Type::Struct`: nested dependent pairs
//!
//!   For example, the struct:
//!
//!   ```plain
//!   struct { len : u16, reserved : u16, data : [u16; len] }
//!   ```
//!
//!   Would be desugared into:
//!
//!   ```plain
//!   Σ len:u16 . Σ reserved:u16 . [u16; len]
//!   ```
//!
//!   Note how later fields have access to the data in previous fields.
//!
//! - `Type::Array`: TODO
//!
//! - `Type::Where`: constrained type

use ast::{Binop, Const, Definition, Expr, Kind, Type, TypeConst, Unop};
use env::Env;
use source::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KindError {
    UnboundType(Span, String),
    ArraySizeExpectedUInt(Span, Type),
    ArraySizeType(Span, TypeError),
    WherePredicateExpectedBool(Span, Type),
    WherePredicateType(Span, TypeError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeError {
    UnboundVariable(Span, String),
}

impl<'parent> Env<'parent> {
    pub fn check_defs<I: IntoIterator<Item = Definition>>(
        &mut self,
        defs: I,
    ) -> Result<(), KindError> {
        for def in defs {
            self.kind_of(&def.ty)?;
            self.add_ty(def.name, def.ty);
        }
        Ok(())
    }

    /// `Γ ⊢ τ : κ`
    ///
    /// ```plain
    /// ―――――――――――――――――――― (K-CONST)
    ///     Γ ⊢ c : Type
    ///
    ///
    ///         α ∈ Γ
    /// ―――――――――――――――――――― (K-VAR)
    ///     Γ ⊢ α : Type
    ///
    ///
    ///     Γ ⊢ τ₁ : Type        Γ ⊢ τ₂ : Type
    /// ―――――――――――――――――――――――――――――――――――――――――― (K-SUM)
    ///              Γ ⊢ τ₁ + τ₂ : Type
    ///
    ///
    ///     Γ ⊢ τ₁ : Type        Γ, x:τ₁ ⊢ τ₂ : Type
    /// ―――――――――――――――――――――――――――――――――――――――――――――――――― (K-DEPENDENT-PAIR)
    ///              Γ ⊢ Σ x:τ₁ .τ₂ : Type
    ///
    ///
    ///     Γ ⊢ τ : Type        Γ ⊢ e : Int
    /// ―――――――――――――――――――――――――――――――――――――――――― (K-ARRAY)
    ///             Γ ⊢ [τ; e] : Type
    ///
    ///
    ///     Γ ⊢ τ : Type      Γ, x:τ ⊢ b : Bool
    /// ―――――――――――――――――――――――――――――――――――――――――― (K-CON)
    ///           Γ ⊢ { x:τ | b } : Type
    /// ```
    pub fn kind_of(&self, ty: &Type) -> Result<Kind, KindError> {
        match *ty {
            // K-CONST
            Type::Const(_) => Ok(Kind::Type), // Easypeasy

            // K-VAR
            Type::Var(span, ref name) => {
                // TODO: kind of var?
                // α ∈ Γ
                match self.lookup_ty(name) {
                    Some(_) => Ok(Kind::Type),
                    None => Err(KindError::UnboundType(span, name.clone())),
                }
            }

            // K-SUM
            Type::Union(_, ref tys) => {
                for ty in tys {
                    // Γ ⊢ τ₁ : Type
                    self.kind_of(&ty)?;
                }
                Ok(Kind::Type)
            }

            // K-DEPENDENT-PAIR
            Type::Struct(_, ref fields) => {
                // TODO: prevent name shadowing?
                let mut inner_env = self.extend();
                for field in fields {
                    // Γ ⊢ τ₁ : Type
                    inner_env.kind_of(&field.ty)?;
                    // Γ, x:τ₁ ⊢ τ₂ : Type
                    inner_env.add_binding(field.name.clone(), field.ty.clone());
                }
                Ok(Kind::Type)
            }

            // K-ARRAY
            Type::Array(span, ref ty, ref size) => {
                self.kind_of(ty)?;
                let expr_ty = self.type_of(size).map_err(
                    |err| KindError::ArraySizeType(span, err),
                )?;

                match expr_ty {
                    Type::Const(TypeConst::UnknownInt) |
                    Type::Const(TypeConst::U(_, _)) => Ok(Kind::Type),
                    ty => Err(KindError::ArraySizeExpectedUInt(span, ty)),
                }
            }

            // K-CON
            Type::Where(span, ref ty, ref param, ref pred) => {
                self.kind_of(ty)?;

                let mut inner_env = self.extend();
                // TODO: prevent name shadowing?
                inner_env.add_binding(param.clone(), (**ty).clone());
                match self.type_of(pred) {
                    Ok(Type::Const(TypeConst::Bool)) => Ok(Kind::Type),
                    Ok(pred_ty) => Err(KindError::WherePredicateExpectedBool(span, pred_ty)),
                    Err(err) => Err(KindError::WherePredicateType(span, err)),
                }
            }
        }
    }

    fn type_of_bool_unop(&self, value: &Expr) -> Result<Type, TypeError> {
        match self.type_of(value)? {
            ty @ Type::Const(TypeConst::Bool) => Ok(ty),
            _ => unimplemented!(), // FIXME: better errors
        }
    }

    fn type_of_int_unop(&self, value: &Expr) -> Result<Type, TypeError> {
        match self.type_of(value)? {
            ty @ Type::Const(TypeConst::UnknownInt) |
            ty @ Type::Const(TypeConst::U(_, _)) |
            ty @ Type::Const(TypeConst::I(_, _)) => Ok(ty),
            _ => unimplemented!(), // FIXME: better errors
        }
    }

    fn type_of_bool_binop(&self, lhs: &Expr, rhs: &Expr) -> Result<Type, TypeError> {
        use ast::TypeConst::Bool;
        use ast::Type::Const;

        let lhs_ty = self.type_of(lhs)?;
        let rhs_ty = self.type_of(rhs)?;

        match (lhs_ty, rhs_ty) {
            (ty @ Const(Bool), Const(Bool)) => Ok(ty),
            (_, _) => unimplemented!(), // FIXME: better errors
        }
    }

    fn type_of_comparison_binop(&self, lhs: &Expr, rhs: &Expr) -> Result<Type, TypeError> {
        use ast::Type::Const;

        let lhs_ty = self.type_of(lhs)?;
        let rhs_ty = self.type_of(rhs)?;

        // FIXME: Ugh
        match (lhs_ty, rhs_ty) {
            // Coerce to LHS if the RHS is less specific
            (Const(TypeConst::U(_, _)), Const(TypeConst::UnknownInt)) |
            (Const(TypeConst::I(_, _)), Const(TypeConst::UnknownInt)) |
            // Coerce to RHS if the LHS is less specific
            (Const(TypeConst::UnknownInt), Const(TypeConst::U(_, _))) |
            (Const(TypeConst::UnknownInt), Const(TypeConst::I(_, _))) => {
                Ok(Type::bool())
            }
            // Same type if LHS == RHS
            (Const(TypeConst::U(ls, le)), Const(TypeConst::U(rs, re))) => {
                if ls == rs && le == re {
                    Ok(Type::bool())
                } else {
                    unimplemented!()
                }
            }
            // Same type if LHS == RHS
            (Const(TypeConst::I(ls, le)), Const(TypeConst::I(rs, re))) => {
                if ls == rs && le == re {
                    Ok(Type::bool())
                } else {
                    unimplemented!()
                }
            }
            // Error!
            (_, _) => unimplemented!(), // FIXME: better errors
        }
    }

    fn type_of_int_binop(&self, lhs: &Expr, rhs: &Expr) -> Result<Type, TypeError> {
        use ast::Type::Const;

        let lhs_ty = self.type_of(lhs)?;
        let rhs_ty = self.type_of(rhs)?;

        // FIXME: Ugh
        match (lhs_ty, rhs_ty) {
            // Coerce to LHS if the RHS is less specific
            (lhs_ty @ Const(TypeConst::U(_, _)), Const(TypeConst::UnknownInt)) |
            (lhs_ty @ Const(TypeConst::I(_, _)), Const(TypeConst::UnknownInt)) => Ok(lhs_ty),
            // Coerce to RHS if the LHS is less specific
            (Const(TypeConst::UnknownInt), rhs_ty @ Const(TypeConst::U(_, _))) |
            (Const(TypeConst::UnknownInt), rhs_ty @ Const(TypeConst::I(_, _))) => Ok(rhs_ty),
            // Same type if LHS == RHS
            (Const(TypeConst::U(ls, le)), Const(TypeConst::U(rs, re))) => {
                if ls == rs && le == re {
                    Ok(Const(TypeConst::U(ls, le)))
                } else {
                    unimplemented!()
                }
            }
            // Same type if LHS == RHS
            (Const(TypeConst::I(ls, le)), Const(TypeConst::I(rs, re))) => {
                if ls == rs && le == re {
                    Ok(Const(TypeConst::I(ls, le)))
                } else {
                    unimplemented!()
                }
            }
            // Error!
            (_, _) => unimplemented!(), // FIXME: better errors
        }
    }

    /// `Γ ⊢ e : τ`
    ///
    /// ```plain
    /// ―――――――――――――――――――――――――――― (T-TRUE)
    ///       Γ ⊢ true : Bool
    ///
    ///
    /// ―――――――――――――――――――――――――――― (T-FALSE)
    ///       Γ ⊢ false : Bool
    ///
    ///
    ///           x : τ ∈ Γ
    /// ―――――――――――――――――――――――――――― (T-VAR)
    ///           Γ ⊢ x : τ
    /// ```
    pub fn type_of(&self, expr: &Expr) -> Result<Type, TypeError> {
        match *expr {
            // T-TRUE, T-FALSE
            Expr::Const(_, Const::Bool(_)) => Ok(Type::bool()),
            // FIXME: T-???
            Expr::Const(_, Const::UInt(_)) => Ok(Type::unknown_int()),
            // T-VAR
            Expr::Var(span, ref name) => {
                match self.lookup_binding(name) {
                    Some(ty) => Ok(ty.clone()),
                    None => Err(TypeError::UnboundVariable(span, name.clone())),
                }
            }
            // FIXME: T-???
            Expr::Unop(_, op, ref value) => {
                match op {
                    Unop::Not => self.type_of_bool_unop(value),
                    Unop::Neg => self.type_of_int_unop(value),
                }
            }
            // FIXME: T-???
            Expr::Binop(_, op, ref lhs, ref rhs) => {
                match op {
                    Binop::Or | Binop::And => self.type_of_bool_binop(lhs, rhs),
                    Binop::Eq | Binop::Ne | Binop::Le | Binop::Lt | Binop::Gt | Binop::Ge => {
                        self.type_of_comparison_binop(lhs, rhs)
                    }
                    Binop::Add | Binop::Sub | Binop::Mul | Binop::Div => {
                        self.type_of_int_binop(lhs, rhs)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use ast::Endianness;
    use parser;
    use source::BytePos as B;
    use super::*;

    #[test]
    fn ty_const() {
        let env = Env::default();
        let ty = Type::i(16, Endianness::Target);

        assert_eq!(env.kind_of(&ty), Ok(Kind::Type));
    }

    #[test]
    fn var() {
        let env = Env::default();
        let ty = parser::parse_ty(&env, "u8").unwrap();

        assert_eq!(env.kind_of(&ty), Ok(Kind::Type));
    }

    #[test]
    fn var_missing() {
        let env = Env::default();
        let ty = parser::parse_ty(&env, "Foo").unwrap();

        assert_eq!(
            env.kind_of(&ty),
            Err(KindError::UnboundType(
                Span::new(B(0), B(3)),
                "Foo".to_owned(),
            ))
        );
    }

    #[test]
    fn union() {
        let env = Env::default();
        let ty = parser::parse_ty(&env, "union { u8, u16, i32 }").unwrap();

        assert_eq!(env.kind_of(&ty), Ok(Kind::Type));
    }

    #[test]
    fn union_element_missing() {
        let env = Env::default();
        let ty = parser::parse_ty(&env, "union { u8, Foo, i32 }").unwrap();

        assert_eq!(
            env.kind_of(&ty),
            Err(KindError::UnboundType(
                Span::new(B(12), B(15)),
                "Foo".to_owned(),
            ))
        );
    }

    #[test]
    fn pair() {
        let env = Env::default();
        let ty = parser::parse_ty(&env, "struct { x: u8, y: u8 }").unwrap();

        assert_eq!(env.kind_of(&ty), Ok(Kind::Type));
    }

    #[test]
    fn dependent_pair() {
        let env = Env::default();
        let ty = parser::parse_ty(&env, "struct { len: u8, data: [u8; len] }").unwrap();

        assert_eq!(env.kind_of(&ty), Ok(Kind::Type));
    }

    #[test]
    fn array() {
        let env = Env::default();
        let ty = parser::parse_ty(&env, "[u8; 16]").unwrap();

        assert_eq!(env.kind_of(&ty), Ok(Kind::Type));
    }

    #[test]
    fn array_len() {
        let mut env = Env::default();
        let len_ty = Type::u(32, Endianness::Target);
        env.add_binding("len", len_ty);
        let ty = parser::parse_ty(&env, "[u8; len]").unwrap();

        assert_eq!(env.kind_of(&ty), Ok(Kind::Type));
    }

    #[test]
    fn array_singned_int_size() {
        let mut env = Env::default();
        let len_ty = parser::parse_ty(&env, "i8").unwrap();
        env.add_binding("len", len_ty.clone());
        let ty = parser::parse_ty(&env, "[u8; len]").unwrap();

        assert_eq!(
            env.kind_of(&ty),
            Err(KindError::ArraySizeExpectedUInt(
                Span::new(B(0), B(9)),
                len_ty,
            ))
        );
    }

    #[test]
    fn array_struct_size() {
        let mut env = Env::default();
        let len_ty = parser::parse_ty(&env, "struct {}").unwrap();
        env.add_binding("len", len_ty.clone());
        let ty = parser::parse_ty(&env, "[u8; len]").unwrap();

        assert_eq!(
            env.kind_of(&ty),
            Err(KindError::ArraySizeExpectedUInt(
                Span::new(B(0), B(9)),
                len_ty,
            ))
        );
    }
}
