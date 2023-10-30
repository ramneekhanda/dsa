use rustpython_vm::*;

pub enum Event {
  Data,
  Timer(String),
  SetNeighbours(Vec<String>),
}

pub struct Context {
  pub event: Event,
}

#[pymodule]
pub mod send_mod {
  #[pyfunction]
  pub fn send(n_id: String, msg: String) {
    println!("sending msg {:?} to {}", msg, n_id);
  }
}
