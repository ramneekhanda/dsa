use crate::{
    parser::{
        graphv2::{parse_graph2_with_sources, File},
        imports::scan_import_urls,
    },
    resources::{
        common_assets::{LoadingState, LoadingStateOpt},
        graph_def::GraphDefinitionRes,
    },
    ui::CodeStorage,
};

use bevy::prelude::*;

use lazy_static::lazy_static;
use schemars::schema_for;
use serde_json;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref E_CODE: Mutex<String> = Mutex::new(String::new());
    static ref E_SOURCES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[wasm_bindgen]
pub struct CompileResult {
    result: bool,
    error_log: String,
}

#[wasm_bindgen]
impl CompileResult {
    #[wasm_bindgen(getter)]
    pub fn error_log(&self) -> String {
        self.error_log.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn result(&self) -> bool {
        self.result.clone()
    }
}

#[wasm_bindgen]
pub fn get_code_schema() -> String {
    let schema = schema_for!(File);
    return serde_json::to_string_pretty(&schema).unwrap();
}

#[wasm_bindgen]
pub fn get_import_urls(yaml: String) -> String {
    let urls = scan_import_urls(&yaml);
    serde_json::to_string(&urls).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn compile_code_with_sources(s: String, sources_json: String) -> CompileResult {
    let sources: HashMap<String, String> = serde_json::from_str(&sources_json).unwrap_or_default();
    {
        let mut e_sources = E_SOURCES.lock().unwrap();
        for (k, v) in sources.iter() {
            e_sources.insert(k.clone(), v.clone());
        }
    }
    let current_sources = E_SOURCES.lock().unwrap().clone();
    let ret = parse_graph2_with_sources(&s, &current_sources);
    match ret {
        Ok(_file) => {
            let mut code = E_CODE.lock().unwrap();
            *code = s;
            CompileResult {
                result: true,
                error_log: "".to_string(),
            }
        }
        Err(e) => CompileResult {
            result: false,
            error_log: e.to_string(),
        },
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn compile_code(s: String) -> CompileResult {
    let current_sources = E_SOURCES.lock().unwrap().clone();
    let ret = parse_graph2_with_sources(&s, &current_sources);
    match ret {
        Ok(_file) => {
            let mut code = E_CODE.lock().unwrap();
            *code = s;
            CompileResult {
                result: true,
                error_log: "".to_string(),
            }
        }
        Err(e) => CompileResult {
            result: false,
            error_log: e.to_string(),
        },
    }
}

/// Native-only: `compile_code`/`get_code_schema` only ever get called from the
/// SvelteKit frontend's JS, through wasm-bindgen - there's no browser (and no
/// Monaco editor) in a native `cargo run`, so without this a native window
/// would just show an empty background grid forever, with nothing to load a
/// graph. Seeds `E_CODE` at `Startup` with one of the checked-in examples
/// (embedded via `include_str!` so this doesn't depend on the process's
/// working directory) through the exact same `compile_code` path the browser
/// uses, so it's picked up by `ingest_codechange` on the first `Update` frame
/// like any other "code changed" edit.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_native_demo_on_startup() {
    let result = compile_code(crate::systems::native_examples::DEFAULT_EXAMPLE.to_string());
    if !result.result {
        eprintln!("failed to load native demo graph: {}", result.error_log);
    }
}

#[allow(dead_code)]
pub fn ingest_codechange(
    mut code_store: ResMut<CodeStorage>,
    mut graph_defn: ResMut<GraphDefinitionRes>,
    mut ls: ResMut<LoadingState>,
) {
    let code: std::sync::MutexGuard<'_, String> = E_CODE.lock().unwrap();

    if code_store.code != *code {
        // TODO: can improve performance by checking a boolean instead
        code_store.code = (*code).clone();
        let current_sources = E_SOURCES.lock().unwrap().clone();
        let res = parse_graph2_with_sources(&code_store.code, &current_sources);
        match res {
            Ok(file) => {
                ls.state = LoadingStateOpt::Loading;
                graph_defn.graph_defn = file.graph_defn;
            }
            Err(e) => {
                println!("{e}");
                return;
            }
        }
    }
}
