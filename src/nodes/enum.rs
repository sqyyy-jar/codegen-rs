use std::fmt::Write;

use crate::generator::GenerateCode;

use super::CowStr;

pub struct Enum {
    public: bool,
    name: CowStr,
    variants: Vec<EnumVariant>,
}

impl Enum {
    pub fn new(name: CowStr) -> Self {
        Self {
            public: false,
            name,
            variants: Vec::new(),
        }
    }

    pub fn set_public(&mut self, public: bool) -> &mut Self {
        self.public = public;
        self
    }
}

impl GenerateCode for Enum {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.public {
            _ = write!(fmt, "pub ");
        }
        _ = writeln!(fmt, "enum {} ", self.name);
        fmt.write_block(|fmt| {
            for variant in &self.variants {
                variant.generate(fmt);
                todo!("Implement enum variant separation")
            }
        });
    }
}

pub struct EnumVariant {}

impl GenerateCode for EnumVariant {
    fn generate(&self, _fmt: &mut crate::generator::Formatter) {
        todo!("Implement enum variant")
    }
}
