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

//TODO write this as a actual string parser
pub fn parse_graph(graph_code: &String) -> Result<NodeDepsMap, String>{
  let lines = graph_code.lines();
  let mut nodes = NodeDepsMap::new();

  for (i, line) in lines.enumerate() {
    if line.len() > 0 {
      let splits: Vec<_> = line.split(":").collect();
      if splits.len() != 2 {
        return Err("ill format line ".to_string() + &i.to_string());
      } else {
        // cleanup all data
        let deps: Vec<String> = splits.get(1).unwrap().split(' ').map(|x| x.to_string()).collect();
        let mut deps_clean : Vec<String> = deps.iter().map(|x| x.trim().to_string()).filter(|x| x.len() > 0).collect();
        let key = splits.get(0).unwrap().trim().to_string();
        deps_clean.sort();
        deps_clean.dedup();

        // now build hashmap
        for s in deps_clean.iter() {
          if *s != key {
            if nodes.contains_key(s) {
              nodes.get_mut(s).unwrap().push(key.clone());
            } else {
              nodes.insert(s.clone(), vec![key.clone()]);
            }
          }
        }
        nodes.insert(key, deps_clean);
      }
    }
  }
  return Ok(nodes);
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

      // let b = val.as_module().unwrap().body;
      // for s in b.iter() {
      //   if s.is_global_stmt() && s.is_assert_stmt() {
      //      for j in s.as_assign_stmt().iter() {
      //        let k = j.clone();
      //        for i in k.targets {
      //          if (i.is_name_expr();
      //          i.is_constant_expr().to_string();
      //        }
      //      }
      //   }
      // }
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
