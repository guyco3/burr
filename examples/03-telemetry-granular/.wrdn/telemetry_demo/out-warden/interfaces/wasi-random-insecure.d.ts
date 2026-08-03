/** @module Interface wasi:random/insecure@0.3.0 **/
export function getInsecureRandomBytes(maxLen: bigint): Uint8Array;
export function getInsecureRandomU64(): bigint;
