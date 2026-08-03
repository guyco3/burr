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
/**
 * # Variants
 * 
 * ## `"ipv4"`
 * 
 * ## `"ipv6"`
 */
export type IpAddressFamily = 'ipv4' | 'ipv6';
export type Duration = import('./wasi-clocks-types.js').Duration;
export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };

export class TcpSocket {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  static create(addressFamily: IpAddressFamily): TcpSocket;
  bind(localAddress: IpSocketAddress): void;
  connect(remoteAddress: IpSocketAddress): Promise<void>;
  listen(): ReadableStream<TcpSocket>;
  send(data: ReadableStream<number>): Promise<Result<void, ErrorCode>>;
  receive(): [ReadableStream<number>, Promise<Result<void, ErrorCode>>];
  getLocalAddress(): IpSocketAddress;
  getRemoteAddress(): IpSocketAddress;
  getIsListening(): boolean;
  getAddressFamily(): IpAddressFamily;
  setListenBacklogSize(value: bigint): void;
  getKeepAliveEnabled(): boolean;
  setKeepAliveEnabled(value: boolean): void;
  getKeepAliveIdleTime(): Duration;
  setKeepAliveIdleTime(value: Duration): void;
  getKeepAliveInterval(): Duration;
  setKeepAliveInterval(value: Duration): void;
  getKeepAliveCount(): number;
  setKeepAliveCount(value: number): void;
  getHopLimit(): number;
  setHopLimit(value: number): void;
  getReceiveBufferSize(): bigint;
  setReceiveBufferSize(value: bigint): void;
  getSendBufferSize(): bigint;
  setSendBufferSize(value: bigint): void;
}

export class UdpSocket {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  static create(addressFamily: IpAddressFamily): UdpSocket;
  bind(localAddress: IpSocketAddress): void;
  connect(remoteAddress: IpSocketAddress): void;
  disconnect(): void;
  send(data: Uint8Array, remoteAddress: IpSocketAddress | undefined): Promise<void>;
  receive(): Promise<[Uint8Array, IpSocketAddress]>;
  getLocalAddress(): IpSocketAddress;
  getRemoteAddress(): IpSocketAddress;
  getAddressFamily(): IpAddressFamily;
  getUnicastHopLimit(): number;
  setUnicastHopLimit(value: number): void;
  getReceiveBufferSize(): bigint;
  setReceiveBufferSize(value: bigint): void;
  getSendBufferSize(): bigint;
  setSendBufferSize(value: bigint): void;
}
