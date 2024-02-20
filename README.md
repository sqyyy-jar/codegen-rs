# codegen-rs

This is a work-in-progress code generation library for Rust.

The exact way to create code is not set in stone yet and there are still
[several things](https://github.com/users/sqyyy-jar/projects/6) missing.

Feel free to open an issue if you have any ideas for the library.

Generation currently looks like this:

```rs
let mut module = Module::new(); // Create a new module

let s_vec2 = module.add_struct("Vec2").set_public(true); // Create a struct
s_vec2.add_field("x", "f32").set_public(true);
s_vec2.add_field("y", "f32").set_public(true);

let i_vec2 = module.add_impl("Vec2"); // Create an impl block
let vec2_new = i_vec2 // Create a function in the impl block
    .add_function("new")
    .set_public(true)
    .set_const(true)
    .add_param("x", "f32")
    .add_param("y", "f32")
    .set_return_type("Self");

vec2_new
    .add_body()
    .add_statement(Statement::Return(Box::new(ReturnStatement::new(
        expr::init_struct("Self")
            .field_auto("x")
            .field_auto("y")
            .into(),
    )))); // Add a body to the function

let code = module.to_string();
```

The above will generate this code:

```rs
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        return Self { x, y };
    }
}
```
