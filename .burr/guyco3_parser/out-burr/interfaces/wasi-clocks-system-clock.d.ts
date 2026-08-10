/** @module Interface wasi:clocks/system-clock@0.3.0 **/
export interface Instant {
  seconds: bigint,
  nanoseconds: number,
}
