use std::fmt::Write;

use crate::generator::GenerateCode;

use super::{expr::Expr, CowStr};

pub struct Static {
    public: bool,
    name: CowStr,
    r#type: CowStr,
    value: Expr,
}

impl Static {
    pub fn new(name: CowStr, r#type: CowStr, value: Expr) -> Self {
        Self {
            public: false,
            name,
            r#type,
            value,
        }
    }

    pub fn set_public(&mut self, public: bool) -> &mut Self {
        self.public = public;
        self
    }
}

impl GenerateCode for Static {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.public {
            _ = write!(fmt, "pub ");
        }
        _ = write!(fmt, "static {}: {} = ", self.name, self.r#type);
        self.value.generate(fmt);
        _ = writeln!(fmt, ";");
    }
}
