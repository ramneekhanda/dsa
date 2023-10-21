use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
use std::collections::{HashMap, HashSet};
use bevy_egui::{egui::{self}, EguiContext};
use egui_dock::{Tree, NodeIndex, DockArea, Style};
use bevy::{prelude::*, window::PrimaryWindow};

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

//global state of the UI
#[derive(Resource)]
pub struct UiState {
  tree: Tree<EguiWindow>,
  viewport_rect: egui::Rect,
}

impl UiState {
  pub fn new() -> Self {
    let mut tree = Tree::new(vec![EguiWindow::ActionPane]);
    let [_, _] =
      tree.split_below(NodeIndex::root(), 0.75, vec![EguiWindow::LogsWindow, EguiWindow::CodeViewer]);
    Self {
      tree,
      viewport_rect: egui::Rect::NOTHING,
    }
  }

  fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
    let mut tab_viewer = TabViewer {
      world,
      viewport_rect: &mut self.viewport_rect,
    };

    DockArea::new(&mut self.tree)
      .style(Style::from_egui(ctx.style().as_ref()))
      .show(ctx, &mut tab_viewer);
  }
}


struct TabViewer<'a> {
  world: &'a mut World,
  viewport_rect: &'a mut egui::Rect,
}

fn draw_codeviewer(w: &mut World, ui: &mut egui::Ui) {
  w.resource_scope::<CodeStorage, _>(|w, mut code_store| {
    egui::ScrollArea::vertical().show(ui, |ui| {
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
            w.resource_scope::<GraphDefinition, _>(|w, mut graph_defn| {
              graph_defn.graph = nodedeps;
            });
          }
        }
      });
      CodeEditor::default()
        .with_rows(100)
        .with_fontsize(14.0)
        .with_theme(ColorTheme::GRUVBOX)
        .with_syntax(Syntax::lua())
        .with_numlines(true)
        .show(ui, &mut code_store.code);
    });
  });
}


fn draw_logviewer(w: &mut World, ui: &mut egui::Ui) {
  w.resource_scope::<CodeStorage, _>(|_, mut code_store| {
    if ui.button("clear").clicked() {
      code_store.console.clear();
    }
    egui::ScrollArea::vertical().show(ui, |ui| {
      ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut code_store.console));
    });

  });
}

impl egui_dock::TabViewer for TabViewer<'_> {
  type Tab = EguiWindow;

  fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {

    match window {
      EguiWindow::CodeViewer => {
        draw_codeviewer(self.world, ui);
      }
      EguiWindow::LogsWindow => {
        draw_logviewer(self.world, ui);
      }
      EguiWindow::ActionPane => {
        *self.viewport_rect = ui.clip_rect();
      }
    }
  }



  fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
    format!("{window:?}").into()
  }

  fn clear_background(&self, window: &Self::Tab) -> bool {
    !matches!(window, EguiWindow::ActionPane)
  }
}

pub fn show_ui_system(world: &mut World) {
  let Ok(egui_context) = world
    .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    .get_single(world)
  else {
    return;
  };
  let mut egui_context = egui_context.clone();
  egui::TopBottomPanel::top("main menu").show(egui_context.get_mut(), |ui| {
    if ui.button("Wat the fuck??").clicked() {

    };
  });


  world.resource_scope::<UiState, _>(|world, mut ui_state| {
    ui_state.ui(world, egui_context.get_mut())
  });
}
