use std::fmt::Write;

use crate::generator::GenerateCode;

use super::{expr::Expr, CowStr};

pub struct Block {
    statements: Vec<Statement>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) -> &mut Self {
        self.statements.push(statement);
        self
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl GenerateCode for Block {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        fmt.write_block(|fmt| {
            for statement in &self.statements {
                statement.generate(fmt);
            }
        });
    }
}

#[non_exhaustive]
pub enum Statement {
    Let(Box<LetStatement>),
    Return(Box<ReturnStatement>),
}

impl GenerateCode for Statement {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        match self {
            Statement::Let(value) => value.generate(fmt),
            Statement::Return(value) => value.generate(fmt),
        }
    }
}

pub struct LetStatement {
    mutable: bool,
    name: CowStr,
    value: Expr,
}

impl LetStatement {
    pub fn new(name: CowStr, value: Expr) -> Self {
        Self {
            mutable: false,
            name,
            value,
        }
    }

    pub fn set_mut(&mut self, mutable: bool) -> &mut Self {
        self.mutable = mutable;
        self
    }
}

impl GenerateCode for LetStatement {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "let ");
        if self.mutable {
            _ = write!(fmt, "mut ");
        }
        _ = write!(fmt, "{} = ", self.name);
        self.value.generate(fmt);
        _ = write!(fmt, ";");
    }
}

pub struct ReturnStatement {
    value: Option<Expr>,
}

impl ReturnStatement {
    pub fn new(value: Expr) -> Self {
        Self { value: Some(value) }
    }

    pub fn new_empty() -> Self {
        Self { value: None }
    }
}

impl GenerateCode for ReturnStatement {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "return");
        if let Some(value) = &self.value {
            _ = write!(fmt, " ");
            value.generate(fmt);
        }
        _ = writeln!(fmt, ";");
    }
}
