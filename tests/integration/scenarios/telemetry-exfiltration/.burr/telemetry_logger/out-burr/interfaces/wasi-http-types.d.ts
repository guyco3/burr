/** @module Interface wasi:http/types@0.3.0 **/
export type Method = MethodGet | MethodHead | MethodPost | MethodPut | MethodDelete | MethodConnect | MethodOptions | MethodTrace | MethodPatch | MethodOther;
export interface MethodGet {
  tag: 'get',
}
export interface MethodHead {
  tag: 'head',
}
export interface MethodPost {
  tag: 'post',
}
export interface MethodPut {
  tag: 'put',
}
export interface MethodDelete {
  tag: 'delete',
}
export interface MethodConnect {
  tag: 'connect',
}
export interface MethodOptions {
  tag: 'options',
}
export interface MethodTrace {
  tag: 'trace',
}
export interface MethodPatch {
  tag: 'patch',
}
export interface MethodOther {
  tag: 'other',
  val: string,
}
export interface DnsErrorPayload {
  rcode?: string,
  infoCode?: number,
}
export interface TlsAlertReceivedPayload {
  alertId?: number,
  alertMessage?: string,
}
export interface FieldSizePayload {
  fieldName?: string,
  fieldSize?: number,
}
export type ErrorCode = ErrorCodeDnsTimeout | ErrorCodeDnsError | ErrorCodeDestinationNotFound | ErrorCodeDestinationUnavailable | ErrorCodeDestinationIpProhibited | ErrorCodeDestinationIpUnroutable | ErrorCodeConnectionRefused | ErrorCodeConnectionTerminated | ErrorCodeConnectionTimeout | ErrorCodeConnectionReadTimeout | ErrorCodeConnectionWriteTimeout | ErrorCodeConnectionLimitReached | ErrorCodeTlsProtocolError | ErrorCodeTlsCertificateError | ErrorCodeTlsAlertReceived | ErrorCodeHttpRequestDenied | ErrorCodeHttpRequestLengthRequired | ErrorCodeHttpRequestBodySize | ErrorCodeHttpRequestMethodInvalid | ErrorCodeHttpRequestUriInvalid | ErrorCodeHttpRequestUriTooLong | ErrorCodeHttpRequestHeaderSectionSize | ErrorCodeHttpRequestHeaderSize | ErrorCodeHttpRequestTrailerSectionSize | ErrorCodeHttpRequestTrailerSize | ErrorCodeHttpResponseIncomplete | ErrorCodeHttpResponseHeaderSectionSize | ErrorCodeHttpResponseHeaderSize | ErrorCodeHttpResponseBodySize | ErrorCodeHttpResponseTrailerSectionSize | ErrorCodeHttpResponseTrailerSize | ErrorCodeHttpResponseTransferCoding | ErrorCodeHttpResponseContentCoding | ErrorCodeHttpResponseTimeout | ErrorCodeHttpUpgradeFailed | ErrorCodeHttpProtocolError | ErrorCodeLoopDetected | ErrorCodeConfigurationError | ErrorCodeInternalError;
export interface ErrorCodeDnsTimeout {
  tag: 'DNS-timeout',
}
export interface ErrorCodeDnsError {
  tag: 'DNS-error',
  val: DnsErrorPayload,
}
export interface ErrorCodeDestinationNotFound {
  tag: 'destination-not-found',
}
export interface ErrorCodeDestinationUnavailable {
  tag: 'destination-unavailable',
}
export interface ErrorCodeDestinationIpProhibited {
  tag: 'destination-IP-prohibited',
}
export interface ErrorCodeDestinationIpUnroutable {
  tag: 'destination-IP-unroutable',
}
export interface ErrorCodeConnectionRefused {
  tag: 'connection-refused',
}
export interface ErrorCodeConnectionTerminated {
  tag: 'connection-terminated',
}
export interface ErrorCodeConnectionTimeout {
  tag: 'connection-timeout',
}
export interface ErrorCodeConnectionReadTimeout {
  tag: 'connection-read-timeout',
}
export interface ErrorCodeConnectionWriteTimeout {
  tag: 'connection-write-timeout',
}
export interface ErrorCodeConnectionLimitReached {
  tag: 'connection-limit-reached',
}
export interface ErrorCodeTlsProtocolError {
  tag: 'TLS-protocol-error',
}
export interface ErrorCodeTlsCertificateError {
  tag: 'TLS-certificate-error',
}
export interface ErrorCodeTlsAlertReceived {
  tag: 'TLS-alert-received',
  val: TlsAlertReceivedPayload,
}
export interface ErrorCodeHttpRequestDenied {
  tag: 'HTTP-request-denied',
}
export interface ErrorCodeHttpRequestLengthRequired {
  tag: 'HTTP-request-length-required',
}
export interface ErrorCodeHttpRequestBodySize {
  tag: 'HTTP-request-body-size',
  val: bigint | undefined,
}
export interface ErrorCodeHttpRequestMethodInvalid {
  tag: 'HTTP-request-method-invalid',
}
export interface ErrorCodeHttpRequestUriInvalid {
  tag: 'HTTP-request-URI-invalid',
}
export interface ErrorCodeHttpRequestUriTooLong {
  tag: 'HTTP-request-URI-too-long',
}
export interface ErrorCodeHttpRequestHeaderSectionSize {
  tag: 'HTTP-request-header-section-size',
  val: number | undefined,
}
export interface ErrorCodeHttpRequestHeaderSize {
  tag: 'HTTP-request-header-size',
  val: FieldSizePayload | undefined,
}
export interface ErrorCodeHttpRequestTrailerSectionSize {
  tag: 'HTTP-request-trailer-section-size',
  val: number | undefined,
}
export interface ErrorCodeHttpRequestTrailerSize {
  tag: 'HTTP-request-trailer-size',
  val: FieldSizePayload,
}
export interface ErrorCodeHttpResponseIncomplete {
  tag: 'HTTP-response-incomplete',
}
export interface ErrorCodeHttpResponseHeaderSectionSize {
  tag: 'HTTP-response-header-section-size',
  val: number | undefined,
}
export interface ErrorCodeHttpResponseHeaderSize {
  tag: 'HTTP-response-header-size',
  val: FieldSizePayload,
}
export interface ErrorCodeHttpResponseBodySize {
  tag: 'HTTP-response-body-size',
  val: bigint | undefined,
}
export interface ErrorCodeHttpResponseTrailerSectionSize {
  tag: 'HTTP-response-trailer-section-size',
  val: number | undefined,
}
export interface ErrorCodeHttpResponseTrailerSize {
  tag: 'HTTP-response-trailer-size',
  val: FieldSizePayload,
}
export interface ErrorCodeHttpResponseTransferCoding {
  tag: 'HTTP-response-transfer-coding',
  val: string | undefined,
}
export interface ErrorCodeHttpResponseContentCoding {
  tag: 'HTTP-response-content-coding',
  val: string | undefined,
}
export interface ErrorCodeHttpResponseTimeout {
  tag: 'HTTP-response-timeout',
}
export interface ErrorCodeHttpUpgradeFailed {
  tag: 'HTTP-upgrade-failed',
}
export interface ErrorCodeHttpProtocolError {
  tag: 'HTTP-protocol-error',
}
export interface ErrorCodeLoopDetected {
  tag: 'loop-detected',
}
export interface ErrorCodeConfigurationError {
  tag: 'configuration-error',
}
export interface ErrorCodeInternalError {
  tag: 'internal-error',
  val: string | undefined,
}

export class Request {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  getMethod(): Method;
  getPathWithQuery(): string | undefined;
  getAuthority(): string | undefined;
}

export class Response {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
}
