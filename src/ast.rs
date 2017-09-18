//! The syntax of our data description language

use std::fmt;

use source::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Type,
}

/// A type definition
///
/// ```plain
/// Point = {
///     x : u16,
///     y : u16,
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub span: Span,
    pub name: String,
    pub ty: Type,
}

impl Definition {
    pub fn new<Sp, S>(span: Sp, name: S, ty: Type) -> Definition
    where
        Sp: Into<Span>,
        S: Into<String>,
    {
        let span = span.into();
        let name = name.into();

        Definition { span, name, ty }
    }
}

/// A boolean expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolExpr {
    /// A boolean constant: eg. `true`, `false`
    Const(Span, bool),
    /// Not: eg. `!x`
    Not(Span, Box<BoolExpr>),
    /// Boolean disjunction: eg. `x | y`
    Or(Span, Box<BoolExpr>, Box<BoolExpr>),
    /// Boolean conjunction: eg. `x & y`
    And(Span, Box<BoolExpr>, Box<BoolExpr>),
    /// Integer equality: eg. `x == y`
    Eq(Span, Box<Expr>, Box<Expr>),
    /// Integer inequality: eg. `x != y`
    Ne(Span, Box<Expr>, Box<Expr>),
    /// Integer less-than-or-equal-to: eg. `x <= y`
    Le(Span, Box<Expr>, Box<Expr>),
    /// Integer less-than: eg. `x < y`
    Lt(Span, Box<Expr>, Box<Expr>),
    /// Integer greater-than: eg. `x > y`
    Gt(Span, Box<Expr>, Box<Expr>),
    /// Integer greater-than-or-equal: eg. `x >= y`
    Ge(Span, Box<Expr>, Box<Expr>),
}

impl BoolExpr {
    /// A boolean constant: eg. `true`, `false`
    pub fn const_<Sp>(span: Sp, value: bool) -> BoolExpr
    where
        Sp: Into<Span>,
    {
        BoolExpr::Const(span.into(), value)
    }

    /// Not: eg. `!x`
    pub fn not<Sp, T>(span: Sp, value: T) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<BoolExpr>>,
    {
        BoolExpr::Not(span.into(), value.into())
    }

    /// Boolean disjunction: eg. `x | y`
    pub fn or<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<BoolExpr>>,
        U: Into<Box<BoolExpr>>,
    {
        BoolExpr::Or(span.into(), lhs.into(), rhs.into())
    }

    /// Boolean conjunction: eg. `x & y`
    pub fn and<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<BoolExpr>>,
        U: Into<Box<BoolExpr>>,
    {
        BoolExpr::And(span.into(), lhs.into(), rhs.into())
    }

    /// Integer equality: eg. `x == y`
    pub fn eq<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        BoolExpr::Eq(span.into(), lhs.into(), rhs.into())
    }

    /// Integer inequality: eg. `x != y`
    pub fn ne<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        BoolExpr::Ne(span.into(), lhs.into(), rhs.into())
    }

    /// Integer less-than-or-equal-to: eg. `x <= y`
    pub fn le<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        BoolExpr::Le(span.into(), lhs.into(), rhs.into())
    }

    /// Integer less-than: eg. `x < y`
    pub fn lt<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        BoolExpr::Lt(span.into(), lhs.into(), rhs.into())
    }

    /// Integer greater-than: eg. `x > y`
    pub fn gt<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        BoolExpr::Gt(span.into(), lhs.into(), rhs.into())
    }

    /// Integer greater-than-or-equal: eg. `x >= y`
    pub fn ge<Sp, T, U>(span: Sp, lhs: T, rhs: U) -> BoolExpr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        BoolExpr::Ge(span.into(), lhs.into(), rhs.into())
    }
}

/// An expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// An integer constant: eg. `0`, `1`, `2`, ...
    Const(Span, u64),
    /// A variable, referring to an integer that exists in the current
    /// context: eg. `len`, `num_tables`
    Var(Span, String),
    /// Integer negation: eg. `-x`
    Neg(Span, Box<Expr>),
    /// Integer addition: eg. `x + y`
    Add(Span, Box<Expr>, Box<Expr>),
    /// Integer subtraction: eg. `x - y`
    Sub(Span, Box<Expr>, Box<Expr>),
    /// Integer multiplication: eg. `x * y`
    Mul(Span, Box<Expr>, Box<Expr>),
    /// Integer division: eg. `x / y`
    Div(Span, Box<Expr>, Box<Expr>),
}

impl Expr {
    /// An integer constant: eg. `0`, `1`, `2`, ...
    pub fn const_<Sp>(span: Sp, value: u64) -> Expr
    where
        Sp: Into<Span>,
    {
        Expr::Const(span.into(), value)
    }

    /// A variable, referring to an integer that exists in the current context: eg. `len`, `num_tables`
    pub fn var<Sp, S>(span: Sp, name: S) -> Expr
    where
        Sp: Into<Span>,
        S: Into<String>,
    {
        Expr::Var(span.into(), name.into())
    }

    /// Integer negation: eg. `-x`
    pub fn neg<Sp, T>(span: Sp, x: T) -> Expr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
    {
        Expr::Neg(span.into(), x.into())
    }

    /// Integer addition: eg. `x + y`
    pub fn add<Sp, T, U>(span: Sp, x: T, y: U) -> Expr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        Expr::Add(span.into(), x.into(), y.into())
    }

    /// Integer subtraction: eg. `x - y`
    pub fn sub<Sp, T, U>(span: Sp, x: T, y: U) -> Expr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        Expr::Sub(span.into(), x.into(), y.into())
    }

    /// Integer multiplication: eg. `x * y`
    pub fn mul<Sp, T, U>(span: Sp, x: T, y: U) -> Expr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        Expr::Mul(span.into(), x.into(), y.into())
    }

    /// Integer division: eg. `x / y`
    pub fn div<Sp, T, U>(span: Sp, x: T, y: U) -> Expr
    where
        Sp: Into<Span>,
        T: Into<Box<Expr>>,
        U: Into<Box<Expr>>,
    {
        Expr::Div(span.into(), x.into(), y.into())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
    Target,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TypeConst {
    /// Unsigned integer
    U(usize, Endianness),
    /// Signed integer
    I(usize, Endianness),
    /// IEEE 754 floating point
    F(usize, Endianness),
}

impl fmt::Debug for TypeConst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeConst::U(b, e) => write!(f, "U({:?}, {:?})", b, e),
            TypeConst::I(b, e) => write!(f, "I({:?}, {:?})", b, e),
            TypeConst::F(b, e) => write!(f, "F({:?}, {:?})", b, e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// A type constant
    Const(Span, TypeConst),
    /// A type variable: eg. `T`
    Var(Span, String),
    /// An array of the specified type, with a size: eg. `[T; n]`
    Array(Span, Box<Type>, Expr),
    /// A union of types: eg. `union { T, ... }`
    Union(Span, Vec<Type>),
    /// A struct type, with fields: eg. `struct { field : T, ... }`
    Struct(Span, Vec<Field>),
    /// A type constrained by a predicate: eg. `T where x => x == 3`
    Where(Span, Box<Type>, String, BoolExpr),
}

impl Type {
    /// A unsigned integer type
    pub fn u<Sp>(span: Sp, bytes: usize, endianness: Endianness) -> Type
    where
        Sp: Into<Span>,
    {
        Type::Const(span.into(), TypeConst::U(bytes, endianness))
    }

    /// A signed integer type
    pub fn i<Sp>(span: Sp, bytes: usize, endianness: Endianness) -> Type
    where
        Sp: Into<Span>,
    {
        Type::Const(span.into(), TypeConst::I(bytes, endianness))
    }

    /// An IEEE 754 floating point type
    pub fn f<Sp>(span: Sp, bytes: usize, endianness: Endianness) -> Type
    where
        Sp: Into<Span>,
    {
        Type::Const(span.into(), TypeConst::F(bytes, endianness))
    }

    /// A type variable: eg. `T`
    pub fn var<Sp, S>(span: Sp, name: S) -> Type
    where
        Sp: Into<Span>,
        S: Into<String>,
    {
        Type::Var(span.into(), name.into())
    }

    /// An array of the specified type, with a size: eg. `[T; n]`
    pub fn array<Sp, T>(span: Sp, ty: T, size: Expr) -> Type
    where
        Sp: Into<Span>,
        T: Into<Box<Type>>,
    {
        Type::Array(span.into(), ty.into(), size)
    }

    /// A union of types: eg. `union { T, ... }`
    pub fn union<Sp>(span: Sp, tys: Vec<Type>) -> Type
    where
        Sp: Into<Span>,
    {
        Type::Union(span.into(), tys)
    }

    /// A struct type, with fields: eg. `struct { field : T, ... }`
    pub fn struct_<Sp>(span: Sp, fields: Vec<Field>) -> Type
    where
        Sp: Into<Span>,
    {
        Type::Struct(span.into(), fields)
    }

    /// A type constrained by a predicate: eg. `T where x => x == 3`
    pub fn where_<Sp, T, S>(span: Sp, ty: T, param: S, pred: BoolExpr) -> Type
    where
        Sp: Into<Span>,
        T: Into<Box<Type>>,
        S: Into<String>,
    {
        Type::Where(span.into(), ty.into(), param.into(), pred)
    }
}

/// A field in a struct type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub span: Span,
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new<Sp, S>(span: Sp, name: S, ty: Type) -> Field
    where
        Sp: Into<Span>,
        S: Into<String>,
    {
        let span = span.into();
        let name = name.into();

        Field { span, name, ty }
    }
}
