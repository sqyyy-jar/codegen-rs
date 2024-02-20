#![allow(unused_must_use)]

use std::fmt::Write;

use crate::{generator::GenerateCode, Str};

pub use self::make::*;

pub mod make;

macro_rules! make {
    (unary, $name: ident, $operator: ident) => {
        pub fn $name(self) -> Self {
            UnaryOperation::new(Operator::$operator, self).into()
        }
    };
    (binary, $name: ident, $operator: ident) => {
        pub fn $name(self, other: impl Into<Self>) -> Self {
            BinaryOperation::new(Operator::$operator, self, other.into()).into()
        }
    };
    (into, $type: ident) => {
        impl From<$type> for Expr {
            fn from(value: $type) -> Self {
                Self::$type(Box::new(value))
            }
        }
    };
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Expr {
    Binding(Box<Binding>),
    Literal(Box<Literal>),
    Call(Box<Call>),
    UnaryOperation(Box<UnaryOperation>),
    BinaryOperation(Box<BinaryOperation>),
    Cast(Box<Cast>),
    Tuple(Box<Tuple>),
    Array(Box<Array>),
    StructInitializer(Box<StructInitializer>),
    TupleStructInitializer(Box<TupleStructInitializer>),
}

impl Expr {
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

    pub fn cast(self, r#type: impl Into<Str>) -> Self {
        Cast::new(self, r#type.into()).into()
    }
}

impl GenerateCode for Expr {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        match self {
            Expr::Binding(value) => value.generate(fmt),
            Expr::Literal(value) => value.generate(fmt),
            Expr::Call(value) => value.generate(fmt),
            Expr::UnaryOperation(value) => value.generate(fmt),
            Expr::BinaryOperation(value) => value.generate(fmt),
            Expr::Cast(value) => value.generate(fmt),
            Expr::Tuple(value) => value.generate(fmt),
            Expr::Array(value) => value.generate(fmt),
            Expr::StructInitializer(value) => value.generate(fmt),
            Expr::TupleStructInitializer(value) => value.generate(fmt),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Binding {
    name: Str,
}

impl Binding {
    pub fn new(name: impl Into<Str>) -> Self {
        Self { name: name.into() }
    }
}

impl GenerateCode for Binding {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        fmt.write_str(&self.name);
    }
}

make!(into, Binding);

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Literal {
    Bool(bool),
    Char(char),
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    Str(Str),
}

impl GenerateCode for Literal {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        match self {
            Literal::Bool(value) => write!(fmt, "{value}"),
            Literal::Char(value) => write!(fmt, "{value:?}"),
            Literal::I8(value) => write!(fmt, "{value}i8"),
            Literal::U8(value) => write!(fmt, "{value}u8"),
            Literal::I16(value) => write!(fmt, "{value}i16"),
            Literal::U16(value) => write!(fmt, "{value}u16"),
            Literal::I32(value) => write!(fmt, "{value}i32"),
            Literal::U32(value) => write!(fmt, "{value}u32"),
            Literal::I64(value) => write!(fmt, "{value}i64"),
            Literal::U64(value) => write!(fmt, "{value}u64"),
            Literal::I128(value) => write!(fmt, "{value}i128"),
            Literal::U128(value) => write!(fmt, "{value}u128"),
            Literal::F32(value) => write!(fmt, "{value:?}f32"),
            Literal::F64(value) => write!(fmt, "{value:?}f64"),
            Literal::Str(value) => write!(fmt, "{value:?}"),
        };
    }
}

make!(into, Literal);

#[derive(Clone, Debug)]
pub struct Call {
    name: Str,
    args: Vec<Expr>,
}

impl Call {
    pub fn new(name: impl Into<Str>) -> Self {
        Self {
            name: name.into(),
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, value: impl Into<Expr>) -> Self {
        self.args.push(value.into());
        self
    }
}

impl GenerateCode for Call {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        write!(fmt, "{}(", self.name);
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(fmt, ", ");
            }
            arg.generate(fmt);
        }
        write!(fmt, ")");
    }
}

make!(into, Call);

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub enum Operator {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `!`
    Not,
    /// `|`
    Or,
    /// `&`
    And,
    /// `^`
    Xor,
    /// `||`
    DoubleOr,
    /// `&&`
    DoubleAnd,
    /// `<<`
    ShiftLeft,
    /// `>>`
    ShiftRight,
    /// `==`
    Equals,
    /// `<`
    Less,
    /// `<=`
    LessEqual,
    /// `>`
    Greater,
    ///`>=`
    GreaterEqual,
}

impl Operator {
    pub fn as_str(self) -> &'static str {
        match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Star => "*",
            Operator::Slash => "/",
            Operator::Percent => "%",
            Operator::Not => "!",
            Operator::Or => "|",
            Operator::And => "&",
            Operator::Xor => "^",
            Operator::DoubleOr => "||",
            Operator::DoubleAnd => "&&",
            Operator::ShiftLeft => "<<",
            Operator::ShiftRight => ">>",
            Operator::Equals => "==",
            Operator::Less => "<",
            Operator::LessEqual => "<=",
            Operator::Greater => ">",
            Operator::GreaterEqual => ">=",
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnaryOperation {
    operator: Operator,
    expr: Expr,
}

impl UnaryOperation {
    pub fn new(operator: Operator, expr: impl Into<Expr>) -> Self {
        Self {
            operator,
            expr: expr.into(),
        }
    }
}

impl GenerateCode for UnaryOperation {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        fmt.write_str(self.operator.as_str());
        self.expr.generate(fmt);
    }
}

make!(into, UnaryOperation);

#[derive(Clone, Debug)]
pub struct BinaryOperation {
    operator: Operator,
    left: Expr,
    right: Expr,
}

impl BinaryOperation {
    pub fn new(operator: Operator, left: impl Into<Expr>, right: impl Into<Expr>) -> Self {
        Self {
            operator,
            left: left.into(),
            right: right.into(),
        }
    }
}

impl GenerateCode for BinaryOperation {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        write!(fmt, "(");
        self.left.generate(fmt);
        write!(fmt, " {} ", self.operator.as_str());
        self.right.generate(fmt);
        write!(fmt, ")");
    }
}

make!(into, BinaryOperation);

#[derive(Clone, Debug)]
pub struct Cast {
    expr: Expr,
    r#type: Str,
}

impl Cast {
    pub fn new(expr: impl Into<Expr>, r#type: impl Into<Str>) -> Self {
        Self {
            expr: expr.into(),
            r#type: r#type.into(),
        }
    }
}

impl GenerateCode for Cast {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        fmt.write_str("(");
        self.expr.generate(fmt);
        write!(fmt, " as {})", self.r#type);
    }
}

make!(into, Cast);

#[derive(Clone, Debug)]
pub struct Tuple {
    multiline: bool,
    values: Vec<Expr>,
}

impl Tuple {
    pub fn new() -> Self {
        Self {
            multiline: false,
            values: Vec::new(),
        }
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn add(mut self, value: impl Into<Expr>) -> Self {
        self.values.push(value.into());
        self
    }
}

impl GenerateCode for Tuple {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.values.is_empty() {
            write!(fmt, "()");
            return;
        }
        write!(fmt, "(");
        if self.multiline {
            writeln!(fmt);
            fmt.indent(|fmt| {
                for value in &self.values {
                    value.generate(fmt);
                    writeln!(fmt, ",");
                }
            });
        } else {
            for (i, value) in self.values.iter().enumerate() {
                if i > 0 {
                    write!(fmt, ", ");
                }
                value.generate(fmt);
            }
            if self.values.len() == 1 {
                write!(fmt, ",");
            }
        }
        write!(fmt, ")");
    }
}

make!(into, Tuple);

#[derive(Clone, Debug)]
pub struct Array {
    multiline: bool,
    values: Vec<Expr>,
}

impl Array {
    pub fn new() -> Self {
        Self {
            multiline: false,
            values: Vec::new(),
        }
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn add(mut self, value: impl Into<Expr>) -> Self {
        self.values.push(value.into());
        self
    }
}

impl GenerateCode for Array {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.values.is_empty() {
            write!(fmt, "[]");
            return;
        }
        write!(fmt, "[");
        if self.multiline {
            writeln!(fmt);
            fmt.indent(|fmt| {
                for value in &self.values {
                    value.generate(fmt);
                    writeln!(fmt, ",");
                }
            });
        } else {
            for (i, value) in self.values.iter().enumerate() {
                if i > 0 {
                    write!(fmt, ", ");
                }
                value.generate(fmt);
            }
        }
        write!(fmt, "]");
    }
}

make!(into, Array);

#[derive(Clone, Debug)]
pub struct StructInitializer {
    multiline: bool,
    name: Str,
    fields: Vec<(Str, Option<Expr>)>,
}

impl StructInitializer {
    pub fn new(name: impl Into<Str>) -> Self {
        Self {
            multiline: false,
            name: name.into(),
            fields: Vec::new(),
        }
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn field(mut self, name: impl Into<Str>, value: impl Into<Expr>) -> Self {
        self.fields.push((name.into(), Some(value.into())));
        self
    }

    /// Adds a field where the value is inferred by name
    pub fn field_auto(mut self, name: impl Into<Str>) -> Self {
        self.fields.push((name.into(), None));
        self
    }
}

impl GenerateCode for StructInitializer {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.fields.is_empty() {
            write!(fmt, "{} {{}}", self.name);
            return;
        }
        write!(fmt, "{} {{", self.name);
        if self.multiline {
            writeln!(fmt);
            fmt.indent(|fmt| {
                for field in &self.fields {
                    fmt.write_str(&field.0);
                    if let Some(value) = &field.1 {
                        write!(fmt, ": ");
                        value.generate(fmt);
                    }
                    writeln!(fmt, ",");
                }
            });
        } else {
            write!(fmt, " ");
            for (i, field) in self.fields.iter().enumerate() {
                if i > 0 {
                    write!(fmt, ", ");
                }
                fmt.write_str(&field.0);
                if let Some(value) = &field.1 {
                    write!(fmt, ": ");
                    value.generate(fmt);
                }
            }
            write!(fmt, " ");
        }
        write!(fmt, "}}");
    }
}

make!(into, StructInitializer);

#[derive(Clone, Debug)]
pub struct TupleStructInitializer {
    name: Str,
    tuple: Tuple,
}

impl TupleStructInitializer {
    pub fn new(name: impl Into<Str>) -> Self {
        Self {
            name: name.into(),
            tuple: Tuple::new(),
        }
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.tuple.multiline = multiline;
        self
    }

    pub fn arg(mut self, value: impl Into<Expr>) -> Self {
        self.tuple.values.push(value.into());
        self
    }
}

impl GenerateCode for TupleStructInitializer {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        fmt.write_str(&self.name);
        self.tuple.generate(fmt);
    }
}

make!(into, TupleStructInitializer);
