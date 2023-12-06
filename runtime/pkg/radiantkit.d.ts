/* tslint:disable */
/* eslint-disable */
/**
*/
export enum RadiantToolType {
  Select = 0,
  Rectangle = 1,
}
/**
*/
export class ColorComponent {
  free(): void;
}
/**
*/
export class RadiantImageNode {
  free(): void;
/**
*/
  id: bigint;
/**
*/
  selection: SelectionComponent;
/**
*/
  tint: ColorComponent;
/**
*/
  transform: TransformComponent;
}
/**
*/
export class RadiantKitAppController {
  free(): void;
/**
* @param {Function} f
* @param {number | undefined} width
* @param {number | undefined} height
*/
  constructor(f: Function, width?: number, height?: number);
/**
* @param {any} message
*/
  handleMessage(message: any): void;
}
/**
*/
export class RadiantLineNode {
  free(): void;
/**
*/
  end: Vec3;
/**
*/
  id: bigint;
/**
*/
  selection: SelectionComponent;
/**
*/
  start: Vec3;
/**
*/
  transform: TransformComponent;
}
/**
*/
export class RadiantPathNode {
  free(): void;
/**
*/
  id: bigint;
/**
*/
  selection: SelectionComponent;
/**
*/
  transform: TransformComponent;
}
/**
*/
export class RadiantRectangleNode {
  free(): void;
/**
* @param {bigint} id
* @param {Vec3} position
* @param {Vec3} scale
* @returns {RadiantRectangleNode}
*/
  static new_wasm(id: bigint, position: Vec3, scale: Vec3): RadiantRectangleNode;
/**
*/
  color: ColorComponent;
/**
*/
  id: bigint;
/**
*/
  selection: SelectionComponent;
/**
*/
  transform: TransformComponent;
}
/**
*/
export class RadiantTextNode {
  free(): void;
/**
*/
  color: ColorComponent;
/**
*/
  id: bigint;
/**
*/
  selection: SelectionComponent;
/**
*/
  transform: TransformComponent;
}
/**
*/
export class SelectionComponent {
  free(): void;
}
/**
*/
export class TransformComponent {
  free(): void;
/**
* @param {Vec3} position
*/
  transform_xy(position: Vec3): void;
/**
* @param {Vec3} scale
*/
  transform_scale(scale: Vec3): void;
/**
* @param {Vec3} position
*/
  set_position(position: Vec3): void;
/**
* @param {Vec3} scale
*/
  set_scale(scale: Vec3): void;
/**
* @param {number} rotation
*/
  set_rotation(rotation: number): void;
/**
* @returns {Vec3}
*/
  position(): Vec3;
/**
* @returns {Vec3}
*/
  scale(): Vec3;
/**
* @returns {number}
*/
  get_rotation(): number;
}
/**
*/
export class Vec3 {
  free(): void;
/**
* @returns {Vec3}
*/
  static zero(): Vec3;
/**
* @param {number} x
* @param {number} y
* @param {number} z
* @returns {Vec3}
*/
  static new(x: number, y: number, z: number): Vec3;
/**
* @param {number} min
* @returns {Vec3}
*/
  static new_with_min(min: number): Vec3;
/**
* @param {Vec3} first
* @param {Vec3} second
* @returns {Vec3}
*/
  static new_with_added(first: Vec3, second: Vec3): Vec3;
/**
* @param {Vec3} other
*/
  add(other: Vec3): void;
/**
* @param {Vec3} other
* @param {number} min
*/
  add_with_min(other: Vec3, min: number): void;
/**
* @param {number} scalar
*/
  add_scalar(scalar: number): void;
/**
* @param {Vec3} other
* @param {number} min
*/
  set_with_min(other: Vec3, min: number): void;
/**
*/
  x: number;
/**
*/
  y: number;
/**
*/
  z: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_radiantkitappcontroller_free: (a: number) => void;
  readonly radiantkitappcontroller_new: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly radiantkitappcontroller_handleMessage: (a: number, b: number) => void;
  readonly __wbg_radiantpathnode_free: (a: number) => void;
  readonly __wbg_get_radiantpathnode_id: (a: number) => number;
  readonly __wbg_set_radiantpathnode_id: (a: number, b: number) => void;
  readonly __wbg_get_radiantpathnode_transform: (a: number) => number;
  readonly __wbg_set_radiantpathnode_transform: (a: number, b: number) => void;
  readonly __wbg_get_radiantpathnode_selection: (a: number) => number;
  readonly __wbg_set_radiantpathnode_selection: (a: number, b: number) => void;
  readonly __wbg_radiantimagenode_free: (a: number) => void;
  readonly __wbg_get_radiantimagenode_id: (a: number) => number;
  readonly __wbg_set_radiantimagenode_id: (a: number, b: number) => void;
  readonly __wbg_get_radiantimagenode_transform: (a: number) => number;
  readonly __wbg_set_radiantimagenode_transform: (a: number, b: number) => void;
  readonly __wbg_get_radiantimagenode_selection: (a: number) => number;
  readonly __wbg_set_radiantimagenode_selection: (a: number, b: number) => void;
  readonly __wbg_get_radiantimagenode_tint: (a: number) => number;
  readonly __wbg_set_radiantimagenode_tint: (a: number, b: number) => void;
  readonly __wbg_radianttextnode_free: (a: number) => void;
  readonly __wbg_get_radianttextnode_id: (a: number) => number;
  readonly __wbg_set_radianttextnode_id: (a: number, b: number) => void;
  readonly __wbg_get_radianttextnode_transform: (a: number) => number;
  readonly __wbg_set_radianttextnode_transform: (a: number, b: number) => void;
  readonly __wbg_get_radianttextnode_selection: (a: number) => number;
  readonly __wbg_set_radianttextnode_selection: (a: number, b: number) => void;
  readonly __wbg_get_radianttextnode_color: (a: number) => number;
  readonly __wbg_set_radianttextnode_color: (a: number, b: number) => void;
  readonly __wbg_radiantrectanglenode_free: (a: number) => void;
  readonly __wbg_get_radiantrectanglenode_id: (a: number) => number;
  readonly __wbg_set_radiantrectanglenode_id: (a: number, b: number) => void;
  readonly __wbg_get_radiantrectanglenode_transform: (a: number) => number;
  readonly __wbg_set_radiantrectanglenode_transform: (a: number, b: number) => void;
  readonly __wbg_get_radiantrectanglenode_selection: (a: number) => number;
  readonly __wbg_set_radiantrectanglenode_selection: (a: number, b: number) => void;
  readonly __wbg_get_radiantrectanglenode_color: (a: number) => number;
  readonly __wbg_set_radiantrectanglenode_color: (a: number, b: number) => void;
  readonly radiantrectanglenode_new_wasm: (a: number, b: number, c: number) => number;
  readonly __wbg_selectioncomponent_free: (a: number) => void;
  readonly __wbg_transformcomponent_free: (a: number) => void;
  readonly transformcomponent_transform_xy: (a: number, b: number) => void;
  readonly transformcomponent_transform_scale: (a: number, b: number) => void;
  readonly transformcomponent_set_position: (a: number, b: number) => void;
  readonly transformcomponent_set_scale: (a: number, b: number) => void;
  readonly transformcomponent_set_rotation: (a: number, b: number) => void;
  readonly transformcomponent_position: (a: number) => number;
  readonly transformcomponent_scale: (a: number) => number;
  readonly transformcomponent_get_rotation: (a: number) => number;
  readonly __wbg_colorcomponent_free: (a: number) => void;
  readonly __wbg_vec3_free: (a: number) => void;
  readonly __wbg_get_vec3_x: (a: number) => number;
  readonly __wbg_set_vec3_x: (a: number, b: number) => void;
  readonly __wbg_get_vec3_y: (a: number) => number;
  readonly __wbg_set_vec3_y: (a: number, b: number) => void;
  readonly __wbg_get_vec3_z: (a: number) => number;
  readonly __wbg_set_vec3_z: (a: number, b: number) => void;
  readonly vec3_zero: () => number;
  readonly vec3_new: (a: number, b: number, c: number) => number;
  readonly vec3_new_with_min: (a: number) => number;
  readonly vec3_new_with_added: (a: number, b: number) => number;
  readonly vec3_add: (a: number, b: number) => void;
  readonly vec3_add_with_min: (a: number, b: number, c: number) => void;
  readonly vec3_add_scalar: (a: number, b: number) => void;
  readonly vec3_set_with_min: (a: number, b: number, c: number) => void;
  readonly __wbg_radiantlinenode_free: (a: number) => void;
  readonly __wbg_get_radiantlinenode_id: (a: number) => number;
  readonly __wbg_set_radiantlinenode_id: (a: number, b: number) => void;
  readonly __wbg_get_radiantlinenode_start: (a: number) => number;
  readonly __wbg_set_radiantlinenode_start: (a: number, b: number) => void;
  readonly __wbg_get_radiantlinenode_end: (a: number) => number;
  readonly __wbg_set_radiantlinenode_end: (a: number, b: number) => void;
  readonly __wbg_get_radiantlinenode_transform: (a: number) => number;
  readonly __wbg_set_radiantlinenode_transform: (a: number, b: number) => void;
  readonly __wbg_get_radiantlinenode_selection: (a: number) => number;
  readonly __wbg_set_radiantlinenode_selection: (a: number, b: number) => void;
  readonly wgpu_compute_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_compute_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_compute_pass_set_push_constant: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_compute_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_bundle_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_bundle_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_set_blend_constant: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_scissor_rect: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_viewport: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly wgpu_render_pass_set_stencil_reference: (a: number, b: number) => void;
  readonly wgpu_render_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_render_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_render_pass_execute_bundles: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_pop_debug_group: (a: number) => void;
  readonly wgpu_render_bundle_insert_debug_marker: (a: number, b: number) => void;
  readonly wgpu_render_bundle_push_debug_group: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcdb301ba96475d9e: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0aed65c4bd670abd: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1a68760c989ce7c3: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h206ecff7fb3d562a: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h7feb6432b025df08: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
