/* tslint:disable */
/* eslint-disable */
export function run_rust_snippet_async(code: string): Promise<any>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run_rust_snippet_async: (a: number, b: number) => any;
  readonly wasm_bindgen__convert__closures_____invoke__h2b08158c8e18ba35: (a: number, b: number) => void;
  readonly wasm_bindgen__closure__destroy__hd891b09c0e6cd361: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__hbfa4c9eab7f6dd93: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen__closure__destroy__h0231de21c3a1b30f: (a: number, b: number) => void;
  readonly wasm_bindgen__convert__closures_____invoke__h8a284c9985e3e563: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
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
