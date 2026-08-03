/** @module Interface local:telemetry-demo/processor@0.1.0 **/
export function analyzeBatch(readings: ReadableStream<SensorReading>): Promise<string>;
export interface SensorReading {
  sensorId: string,
  value: number,
  timestamp: bigint,
}
export type ProcessingError = ProcessingErrorCorruptedData | ProcessingErrorSensorOffline | ProcessingErrorLimitExceeded;
export interface ProcessingErrorCorruptedData {
  tag: 'corrupted-data',
  val: string,
}
export interface ProcessingErrorSensorOffline {
  tag: 'sensor-offline',
}
export interface ProcessingErrorLimitExceeded {
  tag: 'limit-exceeded',
}
