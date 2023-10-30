use rustpython_vm as py_vm;
use rustpython_vm::{PyRef, builtins::PyCode};
use py_vm::Interpreter;
use rustpython_vm::compiler::CompileError;
use crate::python::context::*;

use crate::shimmer::*;
use std::mem::size_of_val;

pub fn compile(code: &String) -> Result<(Interpreter,  PyRef<PyCode>), String>{

  let interpreter = py_vm::Interpreter::with_init(Default::default(), |vm| {
    vm.add_native_module("send_mod", Box::new(send_mod::make_module));
  });

  let res : Result<PyRef<PyCode>, String> = interpreter.enter(|vm| {
    let code_obj_res = vm.compile(code, py_vm::compiler::Mode::Exec, "<embedded>".to_owned());

    let res = match code_obj_res {
      Ok(code_obj) => {
        let mcode_obj = code_obj.clone();
        let scope = vm.new_scope_with_builtins();
        let myscope = scope.clone();
        let a = vm.run_code_obj(code_obj, scope);
        match a {
          Ok(v) => {
            let func = myscope.globals.get_item("handle_event", vm);
            if func.is_ok() {
              Ok(mcode_obj)
            } else {
              Err("function handle_event - not found!".to_string())
            }
          },
          Err(e) => {
            Err("Code execution failed".to_string())
          }
        }
      },
      Err(e) => {
        Err("compilation failed".to_string())
      }
    };
    res
  });
  if res.is_ok() {
    Ok((interpreter, res.unwrap()))
  } else {
    Err(res.err().unwrap())
  }
}

#[test]
fn test_can_compile_simple() {
  let code = r#"
def handle_event():
  import send_mod;
  import time;
  for i in range(1,10):
    send_mod.send("a", time.strftime("%H", time.localtime()))
  return 5
  "#;
  let res: Result<(Interpreter, PyRef<PyCode>), String> = compile(&code.to_string());
  assert_eq!(res.is_ok(), true);

  let (interp, py_code) = res.unwrap();
  for i in 1..5  {
    let py_c = py_code.clone();
    interp.enter(|vm| {
      let scope = vm.new_scope_with_builtins();
      let myscope = scope.clone();
      let a = vm.run_code_obj(py_c, scope);
      let func = myscope.globals.get_item("handle_event", vm).unwrap();
      let v = func.call((), vm);
      match v {
        Ok(val) => {
          let y = val.to_number().str(vm).unwrap().to_string();
          println!("value returned was {}", y);
        },
        Err(b) => {
          panic!("errorrrr2");
        }
      }
    });
  }
}
