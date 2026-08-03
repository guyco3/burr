/** @module Interface wasi:sockets/ip-name-lookup@0.3.0 **/
export function resolveAddresses(name: string): Promise<Array<IpAddress>>;
export type IpAddress = import('./wasi-sockets-types.js').IpAddress;
export type ErrorCode = ErrorCodeAccessDenied | ErrorCodeInvalidArgument | ErrorCodeNameUnresolvable | ErrorCodeTemporaryResolverFailure | ErrorCodePermanentResolverFailure | ErrorCodeOther;
export interface ErrorCodeAccessDenied {
  tag: 'access-denied',
}
export interface ErrorCodeInvalidArgument {
  tag: 'invalid-argument',
}
export interface ErrorCodeNameUnresolvable {
  tag: 'name-unresolvable',
}
export interface ErrorCodeTemporaryResolverFailure {
  tag: 'temporary-resolver-failure',
}
export interface ErrorCodePermanentResolverFailure {
  tag: 'permanent-resolver-failure',
}
export interface ErrorCodeOther {
  tag: 'other',
  val: string | undefined,
}
