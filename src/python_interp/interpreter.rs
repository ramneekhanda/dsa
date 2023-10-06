

pub struct Interpreter {
  code: String,
  vm: rustpython_vm::Interpreter
}

impl Interpreter {
  pub fn compile_code(self: &mut Interpreter, code: String) -> Result<&mut Interpreter, String> {
    Ok(self)
  }

  pub fn add_function(self: &mut Interpreter, code: String)  -> Result<&mut Interpreter, String> {
    Ok(self)
  }
}
