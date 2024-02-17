use std::fmt::{Display, Write};

use crate::generator::GenerateCode;

use super::CowStr;

#[non_exhaustive]
pub enum Expr {
    Raw(CowStr),
    Binding(CowStr),
    Bool(bool),
    Char(char),
    Float(f64),
    UnsignedInt(u64),
    SignedInt(i64),
    String(CowStr),
    Array(Box<Array>),
    Tuple(Box<Tuple>),
    StructInitializer(Box<StructInitializer>),
    TupleInitializer(Box<TupleInitializer>),
    UnaryOp(Box<UnaryOp>),
    BinaryOp(Box<UnaryOp>),
}

impl GenerateCode for Expr {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        match self {
            Expr::Raw(value) => _ = write!(fmt, "{value}"),
            Expr::Binding(value) => _ = write!(fmt, "{value}"),
            Expr::Bool(value) => _ = write!(fmt, "{value}"),
            Expr::Char(value) => _ = write!(fmt, "{value:?}"),
            Expr::Float(value) => _ = write!(fmt, "{value}"),
            Expr::UnsignedInt(value) => _ = write!(fmt, "{value}"),
            Expr::SignedInt(value) => _ = write!(fmt, "{value}"),
            Expr::String(value) => _ = write!(fmt, "{value:?}"),
            Expr::Array(value) => value.generate(fmt),
            Expr::Tuple(value) => value.generate(fmt),
            Expr::StructInitializer(value) => value.generate(fmt),
            Expr::TupleInitializer(value) => value.generate(fmt),
            Expr::UnaryOp(value) => value.generate(fmt),
            Expr::BinaryOp(value) => value.generate(fmt),
        }
    }
}

pub struct Array {
    multiline: bool,
    elements: Vec<Expr>,
}

impl Array {
    pub fn new() -> Self {
        Self {
            multiline: false,
            elements: Vec::new(),
        }
    }

    pub fn set_multiline(&mut self, multiline: bool) -> &mut Self {
        self.multiline = multiline;
        self
    }

    pub fn add_element(&mut self, element: Expr) -> &mut Self {
        self.elements.push(element);
        self
    }
}

impl Default for Array {
    fn default() -> Self {
        Self::new()
    }
}

impl GenerateCode for Array {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "[");
        fmt.indent(|fmt| {
            if self.multiline {
                if !self.elements.is_empty() {
                    _ = writeln!(fmt);
                }
                for (i, element) in self.elements.iter().enumerate() {
                    if i > 0 {
                        _ = writeln!(fmt, ",");
                    }
                    element.generate(fmt);
                }
            } else {
                for (i, element) in self.elements.iter().enumerate() {
                    if i > 0 {
                        _ = write!(fmt, ", ");
                    }
                    element.generate(fmt);
                }
            }
        });
        _ = write!(fmt, "]");
    }
}

pub struct Tuple {
    multiline: bool,
    elements: Vec<Expr>,
}

impl Tuple {
    pub fn new() -> Self {
        Self {
            multiline: false,
            elements: Vec::new(),
        }
    }

    pub fn set_multiline(&mut self, multiline: bool) -> &mut Self {
        self.multiline = multiline;
        self
    }

    pub fn add_element(&mut self, element: Expr) -> &mut Self {
        self.elements.push(element);
        self
    }
}

impl Default for Tuple {
    fn default() -> Self {
        Self::new()
    }
}

impl GenerateCode for Tuple {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "(");
        fmt.indent(|fmt| {
            if self.multiline {
                if !self.elements.is_empty() {
                    _ = writeln!(fmt);
                }
                for (i, element) in self.elements.iter().enumerate() {
                    if i > 0 {
                        _ = writeln!(fmt, ",");
                    }
                    element.generate(fmt);
                }
                if self.elements.len() == 1 {
                    _ = writeln!(fmt, ",");
                }
            } else {
                for (i, element) in self.elements.iter().enumerate() {
                    if i > 0 {
                        _ = write!(fmt, ", ");
                    }
                    element.generate(fmt);
                }
                if self.elements.len() == 1 {
                    _ = write!(fmt, ",");
                }
            }
        });
        _ = write!(fmt, ")");
    }
}

pub struct StructInitializer {
    multiline: bool,
    r#type: CowStr,
    fields: Vec<(CowStr, Option<Expr>)>,
}

impl StructInitializer {
    pub fn new(r#type: CowStr) -> Self {
        Self {
            multiline: false,
            r#type,
            fields: Vec::new(),
        }
    }

    pub fn set_multiline(&mut self, multiline: bool) -> &mut Self {
        self.multiline = multiline;
        self
    }

    pub fn add_field(&mut self, name: CowStr, value: Option<Expr>) -> &mut Self {
        self.fields.push((name, value));
        self
    }
}

impl GenerateCode for StructInitializer {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.multiline {
            _ = writeln!(fmt, "{} ", self.r#type);
            fmt.write_block(|fmt| {
                for field in &self.fields {
                    _ = write!(fmt, "{}", field.0);
                    if let Some(value) = &field.1 {
                        _ = write!(fmt, ": ");
                        value.generate(fmt);
                    }
                    _ = writeln!(fmt, ",");
                }
            });
        } else {
            _ = write!(fmt, "{} {{", self.r#type);
            for (i, field) in self.fields.iter().enumerate() {
                if i > 0 {
                    _ = write!(fmt, ",");
                }
                _ = write!(fmt, " {}", field.0);
                if let Some(value) = &field.1 {
                    _ = write!(fmt, ": ");
                    value.generate(fmt);
                }
            }
            _ = write!(fmt, " }}");
        }
    }
}

pub struct TupleInitializer {
    multiline: bool,
    r#type: CowStr,
    args: Vec<Expr>,
}

impl TupleInitializer {
    pub fn new(r#type: CowStr) -> Self {
        Self {
            multiline: false,
            r#type,
            args: Vec::new(),
        }
    }

    pub fn set_multiline(&mut self, multiline: bool) -> &mut Self {
        self.multiline = multiline;
        self
    }

    pub fn add_arg(&mut self, value: Expr) -> &mut Self {
        self.args.push(value);
        self
    }
}

impl GenerateCode for TupleInitializer {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.multiline {
            _ = writeln!(fmt, "{}(", self.r#type);
            fmt.indent(|fmt| {
                for arg in &self.args {
                    arg.generate(fmt);
                    _ = writeln!(fmt, ",");
                }
            });
            _ = write!(fmt, ")");
        } else {
            _ = write!(fmt, "{}(", self.r#type);
            for (i, arg) in self.args.iter().enumerate() {
                if i > 0 {
                    _ = write!(fmt, ", ");
                }
                arg.generate(fmt);
            }
            if self.args.len() == 1 {
                _ = write!(fmt, ",");
            }
            _ = write!(fmt, ")");
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Not,
    LogicOr,
    LogicAnd,
    BitOr,
    BitAnd,
    BitLeft,
    BitRight,
    Xor,
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
            Operator::LogicOr => "||",
            Operator::LogicAnd => "&&",
            Operator::BitOr => "|",
            Operator::BitAnd => "&",
            Operator::BitLeft => "<<",
            Operator::BitRight => ">>",
            Operator::Xor => "^",
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct UnaryOp {
    operator: Operator,
    value: Expr,
}

impl UnaryOp {
    pub fn new(operator: Operator, value: Expr) -> Self {
        Self { operator, value }
    }
}

impl GenerateCode for UnaryOp {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "{}", self.operator);
        self.value.generate(fmt);
    }
}

pub struct BinaryOp {
    operator: Operator,
    left: Expr,
    right: Expr,
}

impl BinaryOp {
    pub fn new(operator: Operator, left: Expr, right: Expr) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }
}

impl GenerateCode for BinaryOp {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "(");
        self.left.generate(fmt);
        _ = write!(fmt, " {} ", self.operator);
        self.right.generate(fmt);
        _ = write!(fmt, ")");
    }
}
