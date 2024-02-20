use std::fmt::Write;

use crate::{generator::GenerateCode, Str};

use super::function::Function;

pub struct Trait {
    public: bool,
    name: Str,
    functions: Vec<Function>,
}

impl Trait {
    pub fn new(name: Str) -> Self {
        Self {
            public: false,
            name,
            functions: Vec::new(),
        }
    }

    pub fn set_public(&mut self, public: bool) -> &mut Self {
        self.public = public;
        self
    }

    pub fn add_function(&mut self, name: impl Into<Str>) -> &mut Function {
        push_mut!(self.functions, Function::new(name.into()))
    }
}

impl GenerateCode for Trait {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.public {
            _ = write!(fmt, "pub ");
        }
        _ = write!(fmt, "trait {} ", self.name);
        fmt.write_block(|fmt| {
            for (i, function) in self.functions.iter().enumerate() {
                if i > 0 {
                    _ = writeln!(fmt);
                }
                function.generate(fmt);
            }
        });
    }
}
