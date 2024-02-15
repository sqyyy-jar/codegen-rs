macro_rules! push_mut {
    ($vec: expr, $value: expr) => {{
        $vec.push($value);
        $vec.last_mut().unwrap()
    }};
    (@[$variant: path] $vec: expr, $value: expr) => {{
        let $variant(it) = push_mut!($vec, $variant($value)) else {
            unreachable!();
        };
        it
    }};
}

pub mod r#const;
pub mod r#enum;
pub mod expr;
pub mod function;
pub mod r#impl;
pub mod r#static;
pub mod r#struct;
pub mod r#trait;

use std::{
    borrow::Cow,
    fmt::{Display, Write},
};

use crate::generator::GenerateCode;

use self::{
    expr::Expr, function::Function, r#const::Const, r#enum::Enum, r#impl::Impl, r#static::Static,
    r#struct::Struct, r#trait::Trait,
};

pub type CowStr = Cow<'static, str>;

pub struct Module {
    nodes: Vec<ModuleNode>,
}

impl Module {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_submodule(&mut self, name: impl Into<CowStr>) -> &mut Submodule {
        push_mut!(@[ModuleNode::Submodule] self.nodes, Submodule::new(name.into()))
    }

    pub fn add_const(
        &mut self,
        name: impl Into<CowStr>,
        r#type: impl Into<CowStr>,
        value: impl Into<Expr>,
    ) -> &mut Const {
        push_mut!(@[ModuleNode::Const] self.nodes, Const::new(name.into(), r#type.into(), value.into()))
    }

    pub fn add_static(
        &mut self,
        name: impl Into<CowStr>,
        r#type: impl Into<CowStr>,
        value: impl Into<Expr>,
    ) -> &mut Static {
        push_mut!(@[ModuleNode::Static] self.nodes, Static::new(name.into(), r#type.into(), value.into()))
    }

    pub fn add_struct(&mut self, name: impl Into<CowStr>) -> &mut Struct {
        push_mut!(@[ModuleNode::Struct] self.nodes, Struct::new(name.into()))
    }

    pub fn add_enum(&mut self, name: impl Into<CowStr>) -> &mut Enum {
        push_mut!(@[ModuleNode::Enum] self.nodes, Enum::new(name.into()))
    }

    pub fn add_trait(&mut self, name: impl Into<CowStr>) -> &mut Trait {
        push_mut!(@[ModuleNode::Trait] self.nodes, Trait::new(name.into()))
    }

    pub fn add_impl(&mut self, target: impl Into<CowStr>) -> &mut Impl {
        push_mut!(@[ModuleNode::Impl] self.nodes, Impl::new(target.into()))
    }

    pub fn add_function(&mut self, name: impl Into<CowStr>) -> &mut Function {
        push_mut!(@[ModuleNode::Function] self.nodes, Function::new(name.into()))
    }
}

impl Default for Module {
    fn default() -> Self {
        Self::new()
    }
}

impl GenerateCode for Module {
    fn generate(&self, formatter: &mut crate::generator::Formatter) {
        let len = self.nodes.len();
        for (i, node) in self.nodes.iter().enumerate() {
            node.generate(formatter);
            let j = i + 1;
            if j < len
                && !matches!(
                    (node, &self.nodes[j]),
                    (ModuleNode::Const(_), ModuleNode::Const(_))
                        | (ModuleNode::Static(_), ModuleNode::Static(_))
                )
            {
                _ = writeln!(formatter);
            }
        }
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_code_string())
    }
}

#[non_exhaustive]
pub enum ModuleNode {
    Submodule(Submodule),
    Const(Const),
    Static(Static),
    Struct(Struct),
    Enum(Enum),
    Trait(Trait),
    Impl(Impl),
    Function(Function),
}

impl GenerateCode for ModuleNode {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        match self {
            ModuleNode::Submodule(submodule) => submodule.generate(fmt),
            ModuleNode::Const(const_) => const_.generate(fmt),
            ModuleNode::Static(static_) => static_.generate(fmt),
            ModuleNode::Struct(struct_) => struct_.generate(fmt),
            ModuleNode::Enum(enum_) => enum_.generate(fmt),
            ModuleNode::Trait(trait_) => trait_.generate(fmt),
            ModuleNode::Impl(impl_) => impl_.generate(fmt),
            ModuleNode::Function(function) => function.generate(fmt),
        }
    }
}

pub struct Submodule {
    public: bool,
    name: CowStr,
    module: Module,
}

impl Submodule {
    pub fn new(name: CowStr) -> Self {
        Self {
            public: false,
            name,
            module: Module::new(),
        }
    }

    pub fn set_public(&mut self, public: bool) -> &mut Self {
        self.public = public;
        self
    }

    pub fn add_submodule(&mut self, name: impl Into<CowStr>) -> &mut Submodule {
        self.module.add_submodule(name)
    }

    pub fn add_const(
        &mut self,
        name: impl Into<CowStr>,
        r#type: impl Into<CowStr>,
        value: impl Into<Expr>,
    ) -> &mut Const {
        self.module.add_const(name, r#type, value)
    }

    pub fn add_static(
        &mut self,
        name: impl Into<CowStr>,
        r#type: impl Into<CowStr>,
        value: impl Into<Expr>,
    ) -> &mut Static {
        self.module.add_static(name, r#type, value)
    }

    pub fn add_struct(&mut self, name: impl Into<CowStr>) -> &mut Struct {
        self.module.add_struct(name)
    }

    pub fn add_enum(&mut self, name: impl Into<CowStr>) -> &mut Enum {
        self.module.add_enum(name)
    }

    pub fn add_trait(&mut self, name: impl Into<CowStr>) -> &mut Trait {
        self.module.add_trait(name)
    }

    pub fn add_impl(&mut self, target: impl Into<CowStr>) -> &mut Impl {
        self.module.add_impl(target)
    }

    pub fn add_function(&mut self, name: impl Into<CowStr>) -> &mut Function {
        self.module.add_function(name)
    }
}

impl GenerateCode for Submodule {
    fn generate(&self, fmt: &mut crate::generator::Formatter) {
        if self.public {
            _ = write!(fmt, "pub ");
        }
        _ = write!(fmt, "mod {} ", self.name);
        fmt.write_block(|fmt| {
            self.module.generate(fmt);
        });
    }
}
