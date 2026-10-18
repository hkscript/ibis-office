/* tslint:disable */
/* eslint-disable */
/**
 * The `ReadableStreamType` enum.
 *
 * *This API requires the following crate features to be activated: `ReadableStreamType`*
 */

export type ReadableStreamType = "bytes";

export class IntoUnderlyingByteSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableByteStreamController): Promise<any>;
    start(controller: ReadableByteStreamController): void;
    readonly autoAllocateChunkSize: number;
    readonly type: ReadableStreamType;
}

export class IntoUnderlyingSink {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    abort(reason: any): Promise<any>;
    close(): Promise<any>;
    write(chunk: any): Promise<any>;
}

export class IntoUnderlyingSource {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    cancel(): void;
    pull(controller: ReadableStreamDefaultController): Promise<any>;
}

/**
 * Start the editor in the page. Icons are fetched from
 * `<asset_base>/assets/icons/*.svg`. Pass an absolute URL such as
 * `location.origin`: the icon loader fetches with reqwest, which rejects
 * relative URLs, so `None` (an empty base) loads no icons.
 *
 * `file` is a .pptx, .xlsx or .docx to open, named `file_name`; without it (or if it
 * cannot be read) the editor shows the start view.
 */
export function run(asset_base?: string | null, file_name?: string | null, file?: Uint8Array | null): void;

/**
 * Entry point for web workers
 */
export function wasm_thread_entry_point(ptr: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_intounderlyingbytesource_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingsink_free: (a: number, b: number) => void;
    readonly __wbg_intounderlyingsource_free: (a: number, b: number) => void;
    readonly intounderlyingbytesource_autoAllocateChunkSize: (a: number) => number;
    readonly intounderlyingbytesource_cancel: (a: number) => void;
    readonly intounderlyingbytesource_pull: (a: number, b: any) => any;
    readonly intounderlyingbytesource_start: (a: number, b: any) => void;
    readonly intounderlyingbytesource_type: (a: number) => number;
    readonly intounderlyingsink_abort: (a: number, b: any) => any;
    readonly intounderlyingsink_close: (a: number) => any;
    readonly intounderlyingsink_write: (a: number, b: any) => any;
    readonly intounderlyingsource_cancel: (a: number) => void;
    readonly intounderlyingsource_pull: (a: number, b: any) => any;
    readonly run: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly wasm_thread_entry_point: (a: number) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke___js_sys_79763b9f34dcdf95___Function_fn_wasm_bindgen_549fb158025ef4f3___JsValue_____wasm_bindgen_549fb158025ef4f3___sys__Undefined___js_sys_79763b9f34dcdf95___Function_fn_wasm_bindgen_549fb158025ef4f3___JsValue_____wasm_bindgen_549fb158025ef4f3___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke___wasm_bindgen_549fb158025ef4f3___JsValue__core_73622ca1b2019a5d___result__Result_____wasm_bindgen_549fb158025ef4f3___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures________invoke___web_sys_a50b2e48ee331def___features__gen_MessageEvent__MessageEvent______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke___js_sys_79763b9f34dcdf95___Array______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke___js_sys_79763b9f34dcdf95___Array______true__17: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke___wasm_bindgen_549fb158025ef4f3___JsValue______true_: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke___wasm_bindgen_549fb158025ef4f3___JsValue______true__16: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly wasm_bindgen_549fb158025ef4f3___convert__closures_____invoke_______true__1_: (a: number, b: number) => void;
    readonly __wbindgen_malloc_command_export: (a: number, b: number) => number;
    readonly __wbindgen_realloc_command_export: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc_command_export: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store_command_export: (a: number) => void;
    readonly __wbindgen_free_command_export: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure_command_export: (a: number, b: number) => void;
    readonly __externref_table_dealloc_command_export: (a: number) => void;
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
