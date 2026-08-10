/** @module Interface wasi:http/handler@0.3.0 **/
export function handle(request: Request): Promise<Response>;
export type Response = import('./wasi-http-types.js').Response;
export type ErrorCode = import('./wasi-http-types.js').ErrorCode;
export type Request = import('./wasi-http-types.js').Request;
