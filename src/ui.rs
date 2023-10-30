use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use std::collections::{HashMap, HashSet};
use bevy_egui::{egui::{self}, EguiContexts};
use bevy::{prelude::*};

use crate::parser::graph::parse_graph;

pub type NodeDepsMap = HashMap::<String, HashSet<String>>;

#[derive(Debug)]
pub enum EguiWindow {
  ActionPane,
  CodeViewer,
  LogsWindow,
}

#[derive(Resource, Default)]
pub struct GraphDefinition {
  pub graph: NodeDepsMap,
}

#[derive(Resource, Default)]
pub struct CodeStorage {
  pub code: String,
  pub console: String,
}

pub fn draw_codeviewer(mut contexts: EguiContexts,
                       mut code_store: ResMut<CodeStorage>,
                       mut graph_defn: ResMut<GraphDefinition>) {

  egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {

      ui.vertical_centered(|ui| {
        if ui.button("compile").clicked() {
          let res = parse_graph(&code_store.code);
          if res.is_err() {
            code_store.console = res.err().unwrap() + "\n" + &code_store.console;
          } else {
            let nodedeps : NodeDepsMap = res.ok().unwrap();
            let len = nodedeps.len();
            code_store.console = format!("Compilation succeeded. {} nodes found!\n {}", len, code_store.console);
            for key in nodedeps.keys() {
              code_store.console = format!("node discovered {}\n {}", key, code_store.console);
            }
            graph_defn.graph = nodedeps;
          }
        }
      });
      CodeEditor::default()
        .with_rows(100)
        .with_fontsize(14.0)
        .with_theme(ColorTheme::GRUVBOX)
        .with_syntax(Syntax::lua())
        .with_numlines(true)
        .vscroll(true)
        .show(ui, &mut code_store.code);
  });
}


fn draw_logviewer(w: &mut World, ui: &mut egui::Ui) {
  w.resource_scope::<CodeStorage, _>(|_, mut code_store| {
    if ui.button("clear").clicked() {
      code_store.console.clear();
    }
    egui::ScrollArea::vertical().show(ui, |ui| {
      ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut code_store.console).frame(false));
    });

  });
}
