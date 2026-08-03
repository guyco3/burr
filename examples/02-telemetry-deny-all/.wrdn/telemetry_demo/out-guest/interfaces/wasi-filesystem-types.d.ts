/** @module Interface wasi:filesystem/types@0.3.0 **/
export interface PathFlags {
  symlinkFollow?: boolean,
}
export interface OpenFlags {
  create?: boolean,
  directory?: boolean,
  exclusive?: boolean,
  truncate?: boolean,
}
export interface DescriptorFlags {
  read?: boolean,
  write?: boolean,
  fileIntegritySync?: boolean,
  dataIntegritySync?: boolean,
  requestedWriteSync?: boolean,
  mutateDirectory?: boolean,
}
export type ErrorCode = ErrorCodeAccess | ErrorCodeAlready | ErrorCodeBadDescriptor | ErrorCodeBusy | ErrorCodeDeadlock | ErrorCodeQuota | ErrorCodeExist | ErrorCodeFileTooLarge | ErrorCodeIllegalByteSequence | ErrorCodeInProgress | ErrorCodeInterrupted | ErrorCodeInvalid | ErrorCodeIo | ErrorCodeIsDirectory | ErrorCodeLoop | ErrorCodeTooManyLinks | ErrorCodeMessageSize | ErrorCodeNameTooLong | ErrorCodeNoDevice | ErrorCodeNoEntry | ErrorCodeNoLock | ErrorCodeInsufficientMemory | ErrorCodeInsufficientSpace | ErrorCodeNotDirectory | ErrorCodeNotEmpty | ErrorCodeNotRecoverable | ErrorCodeUnsupported | ErrorCodeNoTty | ErrorCodeNoSuchDevice | ErrorCodeOverflow | ErrorCodeNotPermitted | ErrorCodePipe | ErrorCodeReadOnly | ErrorCodeInvalidSeek | ErrorCodeTextFileBusy | ErrorCodeCrossDevice | ErrorCodeOther;
export interface ErrorCodeAccess {
  tag: 'access',
}
export interface ErrorCodeAlready {
  tag: 'already',
}
export interface ErrorCodeBadDescriptor {
  tag: 'bad-descriptor',
}
export interface ErrorCodeBusy {
  tag: 'busy',
}
export interface ErrorCodeDeadlock {
  tag: 'deadlock',
}
export interface ErrorCodeQuota {
  tag: 'quota',
}
export interface ErrorCodeExist {
  tag: 'exist',
}
export interface ErrorCodeFileTooLarge {
  tag: 'file-too-large',
}
export interface ErrorCodeIllegalByteSequence {
  tag: 'illegal-byte-sequence',
}
export interface ErrorCodeInProgress {
  tag: 'in-progress',
}
export interface ErrorCodeInterrupted {
  tag: 'interrupted',
}
export interface ErrorCodeInvalid {
  tag: 'invalid',
}
export interface ErrorCodeIo {
  tag: 'io',
}
export interface ErrorCodeIsDirectory {
  tag: 'is-directory',
}
export interface ErrorCodeLoop {
  tag: 'loop',
}
export interface ErrorCodeTooManyLinks {
  tag: 'too-many-links',
}
export interface ErrorCodeMessageSize {
  tag: 'message-size',
}
export interface ErrorCodeNameTooLong {
  tag: 'name-too-long',
}
export interface ErrorCodeNoDevice {
  tag: 'no-device',
}
export interface ErrorCodeNoEntry {
  tag: 'no-entry',
}
export interface ErrorCodeNoLock {
  tag: 'no-lock',
}
export interface ErrorCodeInsufficientMemory {
  tag: 'insufficient-memory',
}
export interface ErrorCodeInsufficientSpace {
  tag: 'insufficient-space',
}
export interface ErrorCodeNotDirectory {
  tag: 'not-directory',
}
export interface ErrorCodeNotEmpty {
  tag: 'not-empty',
}
export interface ErrorCodeNotRecoverable {
  tag: 'not-recoverable',
}
export interface ErrorCodeUnsupported {
  tag: 'unsupported',
}
export interface ErrorCodeNoTty {
  tag: 'no-tty',
}
export interface ErrorCodeNoSuchDevice {
  tag: 'no-such-device',
}
export interface ErrorCodeOverflow {
  tag: 'overflow',
}
export interface ErrorCodeNotPermitted {
  tag: 'not-permitted',
}
export interface ErrorCodePipe {
  tag: 'pipe',
}
export interface ErrorCodeReadOnly {
  tag: 'read-only',
}
export interface ErrorCodeInvalidSeek {
  tag: 'invalid-seek',
}
export interface ErrorCodeTextFileBusy {
  tag: 'text-file-busy',
}
export interface ErrorCodeCrossDevice {
  tag: 'cross-device',
}
export interface ErrorCodeOther {
  tag: 'other',
  val: string | undefined,
}

export class Descriptor {
  /**
   * This type does not have a public constructor.
   */
  private constructor();
  openAt(pathFlags: PathFlags, path: string, openFlags: OpenFlags, flags: DescriptorFlags): Promise<Descriptor>;
}
