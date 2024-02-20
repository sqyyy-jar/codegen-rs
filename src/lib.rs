pub mod expr;
pub mod generator;
pub mod nodes;
pub mod visibility;

/// A copy-on-write immutable string slice
pub type Str = std::borrow::Cow<'static, str>;
