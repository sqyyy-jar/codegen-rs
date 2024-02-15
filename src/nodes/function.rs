use std::fmt::Write;

use crate::generator::GenerateCode;

use super::{expr::Expr, CowStr};

pub struct Function {
    public: bool,
    constant: bool,
    name: CowStr,
    params: Vec<Param>,
    return_type: Option<CowStr>,
    body: Option<Block>,
}

impl Function {
    pub fn new(name: CowStr) -> Self {
        Self {
            public: false,
            constant: false,
            name,
            params: Vec::new(),
            return_type: None,
            body: None,
        }
    }

    pub fn set_public(&mut self, public: bool) -> &mut Self {
        self.public = public;
        self
    }

    pub fn set_const(&mut self, constant: bool) -> &mut Self {
        self.constant = constant;
        self
    }

    pub fn add_self_param(&mut self, reference: bool, mutable: bool) -> &mut Self {
        self.params.push(Param::new_self(reference, mutable));
        self
    }

    pub fn add_param(&mut self, name: impl Into<CowStr>, r#type: impl Into<CowStr>) -> &mut Self {
        self.params.push(Param::new(name.into(), r#type.into()));
        self
    }

    pub fn set_return_type(&mut self, return_type: impl Into<CowStr>) -> &mut Self {
        self.return_type = Some(return_type.into());
        self
    }

    pub fn add_body(&mut self) -> &mut Block {
        self.body = Some(Block::new());
        self.body.as_mut().unwrap()
    }
}

impl GenerateCode for Function {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.public {
            _ = write!(fmt, "pub ");
        }
        _ = write!(fmt, "fn {}(", self.name);
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                _ = write!(fmt, ", ");
            }
            param.generate(fmt);
        }
        _ = write!(fmt, ")");
        if let Some(return_type) = &self.return_type {
            _ = write!(fmt, " -> {return_type}");
        }
        match &self.body {
            Some(body) => {
                _ = write!(fmt, " ");
                body.generate(fmt);
            }
            None => _ = writeln!(fmt, ";"),
        }
    }
}

pub struct Param {
    name: CowStr,
    r#type: Option<CowStr>,
}

impl Param {
    pub fn new(name: CowStr, r#type: CowStr) -> Self {
        Self {
            name,
            r#type: Some(r#type),
        }
    }

    pub fn new_self(reference: bool, mutable: bool) -> Self {
        let name = match (reference, mutable) {
            (false, false) => "self",
            (false, true) => "mut self",
            (true, false) => "&self",
            (true, true) => "&mut self",
        };
        Self {
            name: name.into(),
            r#type: None,
        }
    }
}

impl GenerateCode for Param {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "{}", self.name);
        if let Some(type_) = &self.r#type {
            _ = write!(fmt, ": {type_}");
        }
    }
}

pub struct Block {
    statements: Vec<Statement>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
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

pub enum Statement {
    Return(Option<Expr>),
}

impl GenerateCode for Statement {
    fn generate(&self, _fmt: &mut crate::generator::Formatter) {
        todo!("Implement statements")
    }
}
