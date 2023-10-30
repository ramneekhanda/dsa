

pub struct PyInterpreter {
  code: String,
  vm: rustpython_vm::Interpreter
}

impl PyInterpreter {
  pub fn compile_code(self: &mut PyInterpreter, code: String) -> Result<&mut PyInterpreter, String> {
    Ok(self)
  }

  pub fn add_function(self: &mut PyInterpreter, code: String)  -> Result<&mut PyInterpreter, String> {
    Ok(self)
  }
}
