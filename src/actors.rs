use rustpython_vm as py_vm;
use rustpython_vm::{
    pyclass, pymodule, PyObject, PyPayload, PyResult, TryFromBorrowedObject, VirtualMachine,
};

use web_sys::console::log_1;

pub struct NodeImpl {
    node_name: String,
    neighbours: Vec<String>,
    code_handle: u32,
    state: String,
}

pub async fn message_proxy() {

}

#[pymodule]
mod test_mod {
  #[pyfunction]
  pub fn say_hello_rs() {
    //web_sys::console::log_1(&"this is coming from python bitches!!".into());
  }
}

pub fn say_hello() -> py_vm::PyResult<()> {
  log_1(&"starting a new python call".into());
  py_vm::Interpreter::with_init(Default::default(), |vm| {
    log_1(&"Entered Python VM".into());
    vm.add_native_module("testmod", Box::new(test_mod::make_module));
  }).enter(|vm| {
    let scope = vm.new_scope_with_builtins();
    let source = r#"import testmod; testmod.say_hello_rs()"#;
    let code_obj = vm
      .compile(source, py_vm::compiler::Mode::Exec, "<embedded>".to_owned())
      .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;
    log_1(&"Compiled Python".into());

    // log_1(&"running python".into());
    vm.run_code_obj(code_obj, scope);
    Ok(())
  })
}
