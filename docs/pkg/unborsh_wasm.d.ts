/* tslint:disable */
/* eslint-disable */
export function start(): void;
/**
 * Analyze Borsh-serialized data with default options (Comprehensive strategy)
 */
export function analyze_borsh(data: Uint8Array): any;
/**
 * Analyze Borsh-serialized data with a specific strategy
 */
export function analyze_borsh_with_strategy(data: Uint8Array, strategy: number): any;
/**
 * Analyze Borsh-serialized data with custom options
 */
export function analyze_borsh_with_options(data: Uint8Array, strategy: number, max_depth: number, min_confidence: number, include_raw_bytes: boolean, max_matches?: number | null): any;
/**
 * Analyze base64-encoded Borsh data
 */
export function analyze_borsh_base64(base64_str: string): any;
/**
 * Analyze hex-encoded Borsh data
 */
export function analyze_borsh_hex(hex_str: string): any;
/**
 * Extract a structure hypothesis about the data
 */
export function extract_structure_hypothesis(data: Uint8Array): string;
/**
 * Try to interpret the data as a specific type
 */
export function interpret_as(data: Uint8Array, type_name: string): string;
/**
 * Get all available pattern names
 */
export function get_available_patterns(): Array<any>;
/**
 * Get pattern details by name
 */
export function get_pattern_details(pattern_name: string): any;
/**
 * Options for the Borsh analysis
 */
export class JsAnalysisOptions {
  free(): void;
  constructor();
  /**
   * Set the analysis strategy
   * 0: Pattern, 1: Recursive, 2: Probabilistic, 3: Comprehensive
   */
  strategy(strategy: number): JsAnalysisOptions;
  max_depth(depth: number): JsAnalysisOptions;
  min_confidence(confidence: number): JsAnalysisOptions;
  include_raw_bytes(include: boolean): JsAnalysisOptions;
  max_matches(max?: number | null): JsAnalysisOptions;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_jsanalysisoptions_free: (a: number, b: number) => void;
  readonly jsanalysisoptions_new: () => number;
  readonly jsanalysisoptions_strategy: (a: number, b: number) => number;
  readonly jsanalysisoptions_max_depth: (a: number, b: number) => number;
  readonly jsanalysisoptions_min_confidence: (a: number, b: number) => number;
  readonly jsanalysisoptions_include_raw_bytes: (a: number, b: number) => number;
  readonly jsanalysisoptions_max_matches: (a: number, b: number) => number;
  readonly analyze_borsh: (a: any) => [number, number, number];
  readonly analyze_borsh_with_strategy: (a: any, b: number) => [number, number, number];
  readonly analyze_borsh_with_options: (a: any, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
  readonly analyze_borsh_base64: (a: number, b: number) => [number, number, number];
  readonly analyze_borsh_hex: (a: number, b: number) => [number, number, number];
  readonly extract_structure_hypothesis: (a: any) => [number, number, number, number];
  readonly interpret_as: (a: any, b: number, c: number) => [number, number, number, number];
  readonly get_available_patterns: () => [number, number, number];
  readonly get_pattern_details: (a: number, b: number) => [number, number, number];
  readonly start: () => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
