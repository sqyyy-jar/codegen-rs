use codegen::nodes::Module;

fn main() {
    let mut module = Module::new();
    let trait_ = module.add_trait("Instruction");
    let func = trait_.set_public(true).add_function("opcode");
    func.set_return_type("u8");
    module
        .add_struct("Increment")
        .set_public(true)
        .add_field("reg", "i16")
        .set_public(true);
    println!("~~~\n{}\n~~~", module.to_string());
}
