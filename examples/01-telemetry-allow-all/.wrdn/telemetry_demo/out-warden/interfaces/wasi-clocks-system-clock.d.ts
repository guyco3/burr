/** @module Interface wasi:clocks/system-clock@0.3.0 **/
export function now(): Instant;
export function getResolution(): Duration;
export interface Instant {
  seconds: bigint,
  nanoseconds: number,
}
export type Duration = import('./wasi-clocks-types.js').Duration;
