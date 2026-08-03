/** @module Interface wasi:filesystem/preopens@0.3.0 **/
export function getDirectories(): Array<[Descriptor, string]>;
export type Descriptor = import('./wasi-filesystem-types.js').Descriptor;
