/** @module Interface wasi:random/random@0.3.0 **/
export function getRandomBytes(maxLen: bigint): Uint8Array;
export function getRandomU64(): bigint;
