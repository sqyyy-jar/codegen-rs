use std::fmt::Write;

use crate::{generator::GenerateCode, Str};

pub struct Struct {
    public: bool,
    name: Str,
    fields: Vec<Field>,
}

impl Struct {
    pub fn new(name: Str) -> Self {
        Self {
            public: false,
            name,
            fields: Vec::new(),
        }
    }

    pub fn set_public(&mut self, public: bool) -> &mut Self {
        self.public = public;
        self
    }

    pub fn add_field(&mut self, name: impl Into<Str>, r#type: impl Into<Str>) -> &mut Field {
        push_mut!(self.fields, Field::new(name.into(), r#type.into()))
    }
}

impl GenerateCode for Struct {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.public {
            _ = write!(fmt, "pub ");
        }
        _ = write!(fmt, "struct {} ", self.name);
        fmt.write_block(|fmt| {
            for field in &self.fields {
                field.generate(fmt);
            }
        });
    }
}

pub struct Field {
    public: bool,
    name: Str,
    r#type: Str,
}

impl Field {
    pub fn new(name: Str, r#type: Str) -> Self {
        Self {
            public: false,
            name,
            r#type,
        }
    }

    pub fn set_public(&mut self, public: bool) -> &mut Self {
        self.public = public;
        self
    }
}

impl GenerateCode for Field {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.public {
            _ = write!(fmt, "pub ");
        }
        _ = writeln!(fmt, "{}: {},", self.name, self.r#type);
    }
}
