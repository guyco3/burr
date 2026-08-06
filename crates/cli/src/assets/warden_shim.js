import * as warden from './out-warden/virtualizer.js';

const env = warden['wasi:cli/environment@0.3.0'];
export const getEnvironment = env?.getEnvironment;
export const getArguments = env?.getArguments;
export const initialCwd = env?.initialCwd;

const preopens = warden['wasi:filesystem/preopens@0.3.0'];
export const getDirectories = preopens?.getDirectories;

const fsTypes = warden['wasi:filesystem/types@0.3.0'];
export const Descriptor = fsTypes?.Descriptor;

const sockTypes = warden['wasi:sockets/types@0.3.0'];
export const TcpSocket = sockTypes?.TcpSocket;
export const UdpSocket = sockTypes?.UdpSocket;

const ipLookup = warden['wasi:sockets/ip-name-lookup@0.3.0'];
export const resolveAddresses = ipLookup?.resolveAddresses;
