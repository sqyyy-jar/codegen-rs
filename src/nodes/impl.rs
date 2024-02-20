use std::fmt::Write;

use crate::{generator::GenerateCode, Str};

use super::function::Function;

pub struct Impl {
    target: Str,
    r#trait: Option<Str>,
    functions: Vec<Function>,
}

impl Impl {
    pub fn new(target: Str) -> Self {
        Self {
            target,
            r#trait: None,
            functions: Vec::new(),
        }
    }

    /// Set the trait to implement
    pub fn set_trait(&mut self, r#trait: impl Into<Str>) -> &mut Self {
        self.r#trait = Some(r#trait.into());
        self
    }

    pub fn add_function(&mut self, name: impl Into<Str>) -> &mut Function {
        push_mut!(self.functions, Function::new(name.into()))
    }
}

impl GenerateCode for Impl {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        _ = write!(fmt, "impl ");
        if let Some(trait_) = &self.r#trait {
            _ = write!(fmt, "{trait_} for ");
        }
        _ = write!(fmt, "{} ", self.target);
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
