/* tslint:disable */
/* eslint-disable */
/**
* @param {string} data 
* @returns {number} 
*/
export function parse_json(data: string): number;
/**
* @param {string} data 
* @returns {number} 
*/
export function parse_json_typed(data: string): number;
/**
* @param {number} size 
* @returns {number} 
*/
export function allocate_buffer(size: number): number;
/**
* @returns {number} 
*/
export function parse_json_move(): number;
/**
* @returns {number} 
*/
export function parse_json_move_typed(): number;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        