//! This module contains several functions to create expressions.
//!
//! ## Conventions
//!
//! - `Into<Str>` instead of `Str`
//! - `Into<Vec<*>` instead of `Vec<*>`
//! - Immutable expressions return `Expr` directly

use crate::Str;

use super::{
    Array, BinaryOperation, Binding, Call, Cast, Expr, Literal, Operator, StructInitializer, Tuple,
    TupleStructInitializer, UnaryOperation,
};

macro_rules! make {
    (unary, $name: ident, $operator: ident) => {
        pub fn $name(expr: impl Into<Expr>) -> Expr {
            UnaryOperation::new(Operator::$operator, expr.into()).into()
        }
    };
    (binary, $name: ident, $operator: ident) => {
        pub fn $name(left: impl Into<Expr>, right: impl Into<Expr>) -> Expr {
            BinaryOperation::new(Operator::$operator, left.into(), right.into()).into()
        }
    };
    (literal, $variant: ident, $type: ident) => {
        pub fn $type(value: $type) -> Expr {
            Expr::Literal(Box::new(Literal::$variant(value)))
        }
    };
}

make!(unary, negate, Minus);
make!(unary, not, Not);
make!(unary, reference, And);
make!(unary, dereference, Star);
make!(binary, add, Plus);
make!(binary, sub, Minus);
make!(binary, mul, Star);
make!(binary, div, Slash);
make!(binary, rem, Percent);
make!(binary, or, DoubleOr);
make!(binary, and, DoubleAnd);
make!(binary, bit_or, Or);
make!(binary, bit_and, And);
make!(binary, bit_xor, Xor);
make!(binary, bit_shl, ShiftLeft);
make!(binary, bit_shr, ShiftRight);
make!(binary, equals, Equals);
make!(binary, less, Less);
make!(binary, less_equal, LessEqual);
make!(binary, greater, Greater);
make!(binary, greater_equal, GreaterEqual);

pub fn cast(expr: impl Into<Expr>, r#type: impl Into<Str>) -> Expr {
    Cast::new(expr.into(), r#type.into()).into()
}

pub fn binding(name: impl Into<Str>) -> Expr {
    Expr::Binding(Box::new(Binding::new(name.into())))
}

make!(literal, Bool, bool);
make!(literal, Char, char);
make!(literal, I8, i8);
make!(literal, U8, u8);
make!(literal, I16, i16);
make!(literal, U16, u16);
make!(literal, I32, i32);
make!(literal, U32, u32);
make!(literal, I64, i64);
make!(literal, U64, u64);
make!(literal, I128, i128);
make!(literal, U128, u128);

pub fn str(value: impl Into<Str>) -> Expr {
    Expr::Literal(Box::new(Literal::Str(value.into())))
}

pub fn call(name: impl Into<Str>) -> Call {
    Call::new(name.into())
}

pub fn tuple() -> Tuple {
    Tuple::new()
}

pub fn tuple_of(values: impl Into<Vec<Expr>>) -> Tuple {
    Tuple {
        multiline: false,
        values: values.into(),
    }
}

pub fn array() -> Array {
    Array::new()
}

pub fn array_of(values: impl Into<Vec<Expr>>) -> Array {
    Array {
        multiline: false,
        values: values.into(),
    }
}

pub fn init_struct(name: impl Into<Str>) -> StructInitializer {
    StructInitializer::new(name.into())
}

pub fn init_tuple_struct(name: impl Into<Str>) -> TupleStructInitializer {
    TupleStructInitializer::new(name.into())
}
