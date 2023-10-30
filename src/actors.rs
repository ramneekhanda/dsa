use py_vm::VirtualMachine;
use py_vm::builtins::{PyIntRef, PyInt};
use py_vm::convert::ToPyObject;
use rustpython_vm as py_vm;
use rustpython_vm::{
    pymodule, pyclass, compiler::parser as parser
};

use crate::python_interp::interpreter;
use crate::shimmer::*;

use crate::ui::NodeDepsMap;

#[pyclass(name = "NodeImpl", module = false)]
pub struct NodeImpl {
    node_name: String,
    neighbours: Vec<String>,
    code_handle: u32,
    state: String,
}

#[pymodule]
mod test_mod {
  #[pyfunction]
  pub fn say_hello_rs() {
    use super::*;
    log("this is coming from python bitches!!".into());
  }
}

pub fn parse_source(code: &String) {
  let result = parser::parse(&code, parser::Mode::Module, "<embedded>")
                .map_err(|e| e.to_string());
  match result {
    Ok(val) => {
      log("compilation successful".into());
    },
    Err(e) => {
      log("compilation failed".into());
    }
  }
}

pub fn compile_source(code: &String) {
  log("starting a new python call".into());


  let interpreter = py_vm::Interpreter::with_init(Default::default(), |vm| {
    log("Entered Python VM".into());
    vm.add_native_module("testmod", Box::new(test_mod::make_module));
  });

  let compiler_result = interpreter.enter(|vm| {
    let scope = vm.new_scope_with_builtins();
    vm.compile(code, py_vm::compiler::Mode::Exec, "<embedded>".to_owned())
  });

  match compiler_result {
    Ok(val) => {
      log("compilation successful".into());
    },
    Err(e) => {
      log("compilation failed".into());
    }
  }

}
