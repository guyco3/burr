/** @module Interface wasi:sockets/types@0.3.0 **/
/**
 * # Variants
 * 
 * ## `"ipv4"`
 * 
 * ## `"ipv6"`
 */
export type IpAddressFamily = 'ipv4' | 'ipv6';
export type ErrorCode = ErrorCodeAccessDenied | ErrorCodeNotSupported | ErrorCodeInvalidArgument | ErrorCodeOutOfMemory | ErrorCodeTimeout | ErrorCodeInvalidState | ErrorCodeAddressNotBindable | ErrorCodeAddressInUse | ErrorCodeRemoteUnreachable | ErrorCodeConnectionRefused | ErrorCodeConnectionBroken | ErrorCodeConnectionReset | ErrorCodeConnectionAborted | ErrorCodeDatagramTooLarge | ErrorCodeOther;
export interface ErrorCodeAccessDenied {
  tag: 'access-denied',
}
export interface ErrorCodeNotSupported {
  tag: 'not-supported',
}
export interface ErrorCodeInvalidArgument {
  tag: 'invalid-argument',
}
export interface ErrorCodeOutOfMemory {
  tag: 'out-of-memory',
}
export interface ErrorCodeTimeout {
  tag: 'timeout',
}
export interface ErrorCodeInvalidState {
  tag: 'invalid-state',
}
export interface ErrorCodeAddressNotBindable {
  tag: 'address-not-bindable',
}
export interface ErrorCodeAddressInUse {
  tag: 'address-in-use',
}
export interface ErrorCodeRemoteUnreachable {
  tag: 'remote-unreachable',
}
export interface ErrorCodeConnectionRefused {
  tag: 'connection-refused',
}
export interface ErrorCodeConnectionBroken {
  tag: 'connection-broken',
}
export interface ErrorCodeConnectionReset {
  tag: 'connection-reset',
}
export interface ErrorCodeConnectionAborted {
  tag: 'connection-aborted',
}
export interface ErrorCodeDatagramTooLarge {
  tag: 'datagram-too-large',
}
export interface ErrorCodeOther {
  tag: 'other',
  val: string | undefined,
}

export class TcpSocket {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  static create(addressFamily: IpAddressFamily): TcpSocket;
}
