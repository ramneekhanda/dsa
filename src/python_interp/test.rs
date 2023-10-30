use rustpython_vm as py_vm;
use py_vm::VirtualMachine;
use py_vm::builtins::{PyIntRef, PyInt};
use py_vm::convert::ToPyObject;
use rustpython_vm::{
    pymodule, pyclass, compiler::parser as parser
};

use crate::shimmer::*;
use std::mem::size_of_val;



fn test_rustpy_parser() {
  let code = r#"
graph = '''
something is funny
''';

def my_fun():
  send_something()

g = 10

def _fun():
  test()
  "#;

  let result = parser::parse(&code, parser::Mode::Module, "<embedded>")
                .map_err(|e| e.to_string());
  match result {
    Ok(val) => {
      log("compilation successful".into());
      let b = &val.as_module().unwrap().body;
      for s in b.iter() {
        if s.is_function_def_stmt() {
          log("function found - ".to_owned() + s.as_function_def_stmt().unwrap().name.as_str());
        } else {
          if s.is_assign_stmt() {
            log("ass statements found!".into());
            let vt = &s.as_assign_stmt().unwrap().targets;
            if vt.len() == 1 {
              if vt[0].is_name_expr() {
                log("name found ".to_owned() + vt[0].as_name_expr().unwrap().id.as_str());
              }
            }
          } else {
            log("found non graph, non function".into())
          }
        }
      }
    },
    Err(e) => {
      log("compilation failed".into());
    }
  }
}

fn test_rustpy_vm() {
  let code = r#"

graph = '''
something is funny
''';

def my_fun():
  return(10)

def _fun():
  return(2)

"#;

  let interpreter = py_vm::Interpreter::with_init(Default::default(), |vm| {
    log("Entered Python VM".into());
  });

  println!("The useful size of `interpreter` is {}", size_of_val(&interpreter));

  interpreter.enter(|vm| {
    let code_obj_res = vm.compile(code, py_vm::compiler::Mode::Exec, "<embedded>".to_owned());

    match code_obj_res {
      Ok(code_obj) => {
        let scope = vm.new_scope_with_builtins();
        println!("The useful size of `scope` is {}", size_of_val(&scope));

        let myscope = scope.clone();
        let a = vm.run_code_obj(code_obj, scope);
        println!("Leng of scope {}", myscope.globals.len());

        match a {
          Ok(v) => {
            let func = myscope.globals.get_item("my_fun", vm).unwrap();
            let v = func.call((), vm);
            match v {
              Ok(val) => {

                let y = val.to_number().str(vm).unwrap().to_string();
                println!("{}", y);
              },
              Err(b) => {
                panic!("errorrrr2");
              }
            }
          },
          Err(e) => {
            panic!("errorrrr");
          }
        }

      },
      Err(e) => {
        log("compilation failed".into());
      }
    }
  });


}
