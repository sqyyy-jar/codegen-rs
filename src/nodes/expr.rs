use std::fmt::Write;

use crate::generator::GenerateCode;

use super::CowStr;

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
