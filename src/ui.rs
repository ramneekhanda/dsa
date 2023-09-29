use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use std::collections::HashMap;

type NodeDepsMap = HashMap::<String, Vec<String>>;

#[derive(Resource)]
pub struct GraphDefinition {
  pub graph: NodeDepsMap,
}

#[derive(Resource)]
pub struct CodeStorage {
  pub code: String,
  pub graph_code: String,
  pub console: String,
}

//TODO write this as a actual string parser
fn parse_graph(graph_code: &String) -> Result<NodeDepsMap, String>{
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
              nodes.insert(s.clone(), Vec::<String>::new());
            }
          }
        }
        nodes.insert(key, deps_clean);
      }
    }
  }
  return Ok(nodes);
}

pub fn setup_ui(
  mut contexts: EguiContexts,
  mut code_store: ResMut<CodeStorage>,
  mut graph_def:  ResMut<GraphDefinition>,
) {
  egui::Window::new("Graph Specification").show(contexts.ctx_mut(), |ui| {
    CodeEditor::default()
      .id_source("graphspec")
      .with_rows(12)
      .with_fontsize(14.0)
      .with_theme(ColorTheme::GRUVBOX)
      .with_syntax(Syntax::rust())
      .with_numlines(true)
      .show(ui, &mut code_store.graph_code);
  });

  egui::Window::new("Node Console").show(contexts.ctx_mut(), |ui| {
    if ui.button("compile").clicked() {
      let res = parse_graph(&code_store.graph_code);
      if res.is_err() {
        code_store.console = res.err().unwrap() + "\n" + &code_store.console;
      } else {
        let nodedeps : NodeDepsMap = res.ok().unwrap();
        let len = nodedeps.len();
        code_store.console = format!("Compilation succeeded. {} nodes found!\n {}", len, code_store.console);
        for key in nodedeps.keys() {
          code_store.console = format!("node discovered {}\n {}", key, code_store.console);
        }
        graph_def.graph = nodedeps;
      }
    }
    if ui.button("clear").clicked() {
      code_store.console.clear();
    }

    CodeEditor::default()
      .id_source("Node Console")
      .with_rows(12)
      .with_fontsize(14.0)
      .with_theme(ColorTheme::GRUVBOX)
      .with_syntax(Syntax::rust())
      .with_numlines(true)
      .show(ui, &mut code_store.console);
  });
}

use std::future::Future;
fn execute<F: Future<Output = ()> + 'static>(f: F) {
  wasm_bindgen_futures::spawn_local(f);
}
