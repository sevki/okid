/* tslint:disable */
/* eslint-disable */
export function init_panic_hook(): void;
export class Blake3 {
  private constructor();
  free(): void;
}
export class ChunkResult {
  private constructor();
  free(): void;
  offset: bigint;
  length: number;
  readonly fingerprint: OkId;
  readonly content_hash: OkId;
}
export class Chunker {
  free(): void;
  constructor(source: string, chunk_size: number, min_size: number, max_size: number);
  intoAsyncIterator(): ChunksIterator;
}
export class ChunksIterator {
  free(): void;
  /**
   * JS constructor: wraps an empty stream by default.
   */
  constructor();
  /**
   * Symbol.asyncIterator method: returns `this` as the iterator.
   */
  async_iterator(): AsyncIterator<any>;
  /**
   * next(): Promise<{ value: JsValue, done: bool }>
   */
  static next(_this: ChunksIterator): Promise<any>;
}
export class MyAsyncIterator {
  free(): void;
  constructor(start_val: number, max_val: number);
  next(): Promise<any>;
  /**
   * Symbol.asyncIterator method: returns `this` (or a new iterator instance)
   */
  asyncIterator(): AsyncIterator<any>;
}
/**
 * OkId is a double clickable representation of arbitrary binary data
 */
export class OkId {
  private constructor();
  free(): void;
  /**
   * Embed an OkId inside a "secret" emoji using variation selectors
   */
  displaySafe(): string;
  /**
   * Helper function to decode a display_safe encoded OkId
   */
  static fromDisplaySafe(s: string): OkId | undefined;
  /**
   * Parse an OkId from a string
   */
  static fromString(s: string): OkId;
  /**
   * Convert the OkId to a string
   */
  toString(): string;
  /**
   * Get the hash type as a string
   */
  hashType(): string;
  /**
   * Convert to path-safe format
   */
  toPathSafe(): string;
  /**
   * Create an OkId from a SHA256 hash
   */
  static fromSha256(data: Uint8Array): OkId;
  /**
   * Create an OkId from a Blake3 hash
   */
  static fromBlake3(data: Uint8Array): OkId;
  /**
   * Create a new UUID-based OkId
   */
  static newUuid(): OkId;
  /**
   * Create a new ULID-based OkId
   */
  static newUlid(): OkId;
  /**
   * Create a fingerprint OkId from data
   */
  static fingerprint(data: Uint8Array): OkId;
}
export class Sha256 {
  free(): void;
  /**
   * Create a new Sha256 instance from a byte array.
   */
  constructor(bytes: Uint8Array);
  intoOkId(): OkId;
}
export class Uuid {
  free(): void;
  /**
   * Create a new UUID from a string representation.
   */
  constructor();
  /**
   * Create a new UUID from a string representation.
   */
  intoOkId(): OkId;
  /**
   * Create a new UUID from a string representation.
   */
  static fromString(s: string): Uuid;
  /**
   * Create a new UUID from a string representation.
   */
  inner(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_panic_hook: () => void;
  readonly __wbg_chunkresult_free: (a: number, b: number) => void;
  readonly __wbg_get_chunkresult_offset: (a: number) => bigint;
  readonly __wbg_set_chunkresult_offset: (a: number, b: bigint) => void;
  readonly __wbg_get_chunkresult_length: (a: number) => number;
  readonly __wbg_set_chunkresult_length: (a: number, b: number) => void;
  readonly chunkresult_fingerprint: (a: number) => number;
  readonly chunkresult_content_hash: (a: number) => number;
  readonly __wbg_chunker_free: (a: number, b: number) => void;
  readonly chunker_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly chunker_intoAsyncIterator: (a: number) => number;
  readonly __wbg_chunksiterator_free: (a: number, b: number) => void;
  readonly chunksiterator_js_new: () => number;
  readonly chunksiterator_async_iterator: (a: number) => any;
  readonly chunksiterator_next: (a: number) => any;
  readonly __wbg_myasynciterator_free: (a: number, b: number) => void;
  readonly myasynciterator_new: (a: number, b: number) => number;
  readonly myasynciterator_next: (a: number) => any;
  readonly myasynciterator_asyncIterator: (a: number) => any;
  readonly __wbg_sha256_free: (a: number, b: number) => void;
  readonly sha256_new: (a: number, b: number) => number;
  readonly sha256_intoOkId: (a: number) => number;
  readonly okid_displaySafe: (a: number) => [number, number];
  readonly okid_fromDisplaySafe: (a: number, b: number) => number;
  readonly __wbg_uuid_free: (a: number, b: number) => void;
  readonly uuid_new: () => number;
  readonly uuid_intoOkId: (a: number) => number;
  readonly uuid_fromString: (a: number, b: number) => number;
  readonly uuid_inner: (a: number) => [number, number];
  readonly __wbg_okid_free: (a: number, b: number) => void;
  readonly okid_fromString: (a: number, b: number) => [number, number, number];
  readonly okid_toString: (a: number) => [number, number];
  readonly okid_hashType: (a: number) => [number, number];
  readonly okid_toPathSafe: (a: number) => [number, number];
  readonly okid_fromSha256: (a: number, b: number) => number;
  readonly okid_fromBlake3: (a: number, b: number) => number;
  readonly okid_newUuid: () => number;
  readonly okid_newUlid: () => number;
  readonly okid_fingerprint: (a: number, b: number) => number;
  readonly __wbg_blake3_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_6: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure40_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure87_externref_shim: (a: number, b: number, c: any, d: any) => void;
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
