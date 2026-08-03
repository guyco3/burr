/** @module Interface wasi:sockets/types@0.3.0 **/
export type Ipv4Address = [number, number, number, number];
export type Ipv6Address = [number, number, number, number, number, number, number, number];
export type IpAddress = IpAddressIpv4 | IpAddressIpv6;
export interface IpAddressIpv4 {
  tag: 'ipv4',
  val: Ipv4Address,
}
export interface IpAddressIpv6 {
  tag: 'ipv6',
  val: Ipv6Address,
}
export interface Ipv4SocketAddress {
  port: number,
  address: Ipv4Address,
}
export interface Ipv6SocketAddress {
  port: number,
  flowInfo: number,
  address: Ipv6Address,
  scopeId: number,
}
export type IpSocketAddress = IpSocketAddressIpv4 | IpSocketAddressIpv6;
export interface IpSocketAddressIpv4 {
  tag: 'ipv4',
  val: Ipv4SocketAddress,
}
export interface IpSocketAddressIpv6 {
  tag: 'ipv6',
  val: Ipv6SocketAddress,
}
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
/**
 * # Variants
 * 
 * ## `"ipv4"`
 * 
 * ## `"ipv6"`
 */
export type IpAddressFamily = 'ipv4' | 'ipv6';

export class TcpSocket {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  static create(addressFamily: IpAddressFamily): TcpSocket;
  connect(remoteAddress: IpSocketAddress): Promise<void>;
}

export class UdpSocket {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  static create(addressFamily: IpAddressFamily): UdpSocket;
  connect(remoteAddress: IpSocketAddress): void;
}
