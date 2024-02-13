use std::fmt::Write;

pub trait GenerateCode {
    fn generate(&self, fmt: &mut Formatter);

    fn to_code_string(&self) -> String {
        let mut dst = String::new();
        self.generate(&mut Formatter::new(&mut dst));
        dst
    }
}

pub struct Formatter<'a> {
    dst: &'a mut String,
    indent: usize,
}

impl<'a> Formatter<'a> {
    pub fn new(dst: &'a mut String) -> Self {
        Self { dst, indent: 0 }
    }

    pub fn write_block<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        _ = self.write_str("{\n");
        self.indent(f);
        _ = self.write_str("}\n");
    }

    pub fn indent<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        self.indent += 4;
        f(self);
        self.indent -= 4;
    }

    pub fn is_start_of_line(&self) -> bool {
        self.dst.is_empty() || self.dst.ends_with('\n')
    }

    pub fn push_spaces(&mut self) {
        for _ in 0..self.indent {
            self.dst.push(' ');
        }
    }
}

impl<'a> std::fmt::Write for Formatter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut first = true;
        let mut should_indent = self.is_start_of_line();
        for line in s.lines() {
            if !first {
                self.dst.push('\n');
            }
            first = false;
            if should_indent && !line.is_empty() {
                self.push_spaces();
            }
            should_indent = true;
            self.dst.push_str(line);
        }
        if s.ends_with('\n') {
            self.dst.push('\n');
        }
        Ok(())
    }
}
