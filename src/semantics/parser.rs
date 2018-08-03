use moniker::{Embed, Scope};
use std::io;

use syntax::core::{Head, Literal, Neutral, RcTerm, RcType, RcValue, Term, Value};

use super::{normalize, InternalError, TcEnv};

#[derive(Debug)]
pub enum ParseError {
    InvalidType(RcType),
    Internal(InternalError),
    BadArrayIndex(RcValue),
    Io(io::Error),
}

impl From<InternalError> for ParseError {
    fn from(src: InternalError) -> ParseError {
        ParseError::Internal(src)
    }
}

impl From<io::Error> for ParseError {
    fn from(src: io::Error) -> ParseError {
        ParseError::Io(src)
    }
}

pub fn parse<R>(tc_env: &TcEnv, ty: &RcType, bytes: &mut R) -> Result<RcValue, ParseError>
where
    R: io::Read + io::Seek,
{
    use byteorder::{BigEndian as Be, LittleEndian as Le, ReadBytesExt};
    use num_traits::ToPrimitive;

    match **ty {
        Value::Universe(_)
        | Value::IntType(_, _)
        | Value::Literal(_)
        | Value::Pi(_)
        | Value::Lam(_)
        | Value::Record(_)
        | Value::RecordEmpty
        | Value::Array(_) => Err(ParseError::InvalidType(ty.clone())),
        Value::RecordType(ref scope) => {
            let ((label, binder, Embed(ann)), body) = scope.clone().unbind();

            let ann_value = parse(tc_env, &ann, bytes)?;
            let body = body.substs(&[(binder.0.clone(), RcTerm::from(Term::from(&*ann_value)))]);
            let body = normalize(tc_env, &body)?;
            let body_value = parse(tc_env, &body, bytes)?;

            Ok(RcValue::from(Value::Record(Scope::new(
                (label, binder, Embed(ann_value)),
                body_value,
            ))))
        },
        Value::RecordTypeEmpty => Ok(RcValue::from(Value::RecordEmpty)),
        Value::Neutral(ref neutral, ref spine) => match **neutral {
            Neutral::Head(Head::Global(ref n)) => {
                if spine.len() == 0 {
                    #[cfg_attr(rustfmt, rustfmt_skip)]
                    match n.as_str() {
                        "U8" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_u8()?.into())))),
                        "U16Le" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_u16::<Le>()?.into())))),
                        "U16Be" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_u16::<Be>()?.into())))),
                        "U32Le" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_u32::<Le>()?.into())))),
                        "U32Be" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_u32::<Be>()?.into())))),
                        "U64Le" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_u64::<Le>()?.into())))),
                        "U64Be" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_u64::<Be>()?.into())))),
                        "S8" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_i8()?.into())))),
                        "S16Le" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_i16::<Le>()?.into())))),
                        "S16Be" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_i16::<Be>()?.into())))),
                        "S32Le" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_i32::<Le>()?.into())))),
                        "S32Be" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_i32::<Be>()?.into())))),
                        "S64Le" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_i64::<Le>()?.into())))),
                        "S64Be" => Ok(RcValue::from(Value::Literal(Literal::Int(bytes.read_i64::<Be>()?.into())))),
                        "F32Le" => Ok(RcValue::from(Value::Literal(Literal::F32(bytes.read_f32::<Le>()?)))),
                        "F32Be" => Ok(RcValue::from(Value::Literal(Literal::F32(bytes.read_f32::<Be>()?)))),
                        "F64Le" => Ok(RcValue::from(Value::Literal(Literal::F64(bytes.read_f64::<Le>()?)))),
                        "F64Be" => Ok(RcValue::from(Value::Literal(Literal::F64(bytes.read_f64::<Be>()?)))),
                        _ => Err(ParseError::InvalidType(ty.clone())),
                    }
                } else if spine.len() == 2 && *n == "Array" {
                    let len = &spine[0];
                    let elem_ty = &spine[1];
                    match **len {
                        Value::Literal(Literal::Int(ref len)) => Ok(RcValue::from(Value::Array(
                            (0..len.to_usize().unwrap()) // FIXME
                                .map(|_| parse(tc_env, elem_ty, bytes))
                                .collect::<Result<_, _>>()?,
                        ))),
                        _ => Err(ParseError::BadArrayIndex(len.clone())),
                    }
                } else {
                    Err(ParseError::InvalidType(ty.clone()))
                }
            },
            Neutral::Head(Head::Var(ref var)) => Err(InternalError::UnexpectedBoundVar {
                span: None,
                var: var.clone(),
            }.into()),
            Neutral::Head(Head::Extern(_, _))
            | Neutral::If(_, _, _)
            | Neutral::Proj(_, _)
            | Neutral::Case(_, _) => Err(ParseError::InvalidType(ty.clone())),
        },
    }
}