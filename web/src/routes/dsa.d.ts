/* tslint:disable */
/* eslint-disable */

export class CompileResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    readonly error_log: string;
    readonly result: boolean;
}

export function compile_code(s: string): CompileResult;

export function compile_code_with_sources(s: string, sources_json: string): CompileResult;

export function get_code_schema(): string;

export function get_import_urls(yaml: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_compileresult_free: (a: number, b: number) => void;
    readonly compile_code: (a: number, b: number) => number;
    readonly compile_code_with_sources: (a: number, b: number, c: number, d: number) => number;
    readonly compileresult_error_log: (a: number) => [number, number];
    readonly compileresult_result: (a: number) => number;
    readonly get_code_schema: () => [number, number];
    readonly get_import_urls: (a: number, b: number) => [number, number];
    readonly main: (a: number, b: number) => number;
    readonly wgpu_render_bundle_set_pipeline: (a: number, b: bigint) => void;
    readonly wgpu_render_bundle_draw_indirect: (a: number, b: bigint, c: bigint) => void;
    readonly wgpu_render_bundle_set_bind_group: (a: number, b: number, c: bigint, d: number, e: number) => void;
    readonly wgpu_render_bundle_set_vertex_buffer: (a: number, b: number, c: bigint, d: bigint, e: bigint) => void;
    readonly wgpu_render_bundle_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wgpu_render_bundle_draw_indexed_indirect: (a: number, b: bigint, c: bigint) => void;
    readonly wgpu_render_bundle_draw: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly wgpu_render_bundle_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly wgpu_render_bundle_insert_debug_marker: (a: number, b: number) => void;
    readonly wgpu_render_bundle_pop_debug_group: (a: number) => void;
    readonly wgpu_render_bundle_set_index_buffer: (a: number, b: bigint, c: number, d: bigint, e: bigint) => void;
    readonly wgpu_render_bundle_push_debug_group: (a: number, b: number) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___js_sys_ea71831a44c3662a___Array__web_sys_b6b4da6eec6572d0___features__gen_ResizeObserver__ResizeObserver______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue__core_ed718c3d60ebd546___result__Result_____wasm_bindgen_5cfc456e5c09313a___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true__10: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true__11: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true__12: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true__13: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true__14: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true__15: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___wasm_bindgen_5cfc456e5c09313a___JsValue______true__16: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke___bool__true_: (a: number, b: number) => number;
    readonly wasm_bindgen_5cfc456e5c09313a___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
