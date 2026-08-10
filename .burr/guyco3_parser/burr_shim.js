import * as burr from './out-burr/virtualizer.js';

const env = burr['wasi:cli/environment@0.3.0'];
export const getEnvironment = env?.getEnvironment;
export const getArguments = env?.getArguments;
export const initialCwd = env?.initialCwd;

const preopens = burr['wasi:filesystem/preopens@0.3.0'];
export const getDirectories = preopens?.getDirectories;

const fsTypes = burr['wasi:filesystem/types@0.3.0'];
export const Descriptor = fsTypes?.Descriptor;

const sockTypes = burr['wasi:sockets/types@0.3.0'];
export const TcpSocket = sockTypes?.TcpSocket;
export const UdpSocket = sockTypes?.UdpSocket;

const ipLookup = burr['wasi:sockets/ip-name-lookup@0.3.0'];
export const resolveAddresses = ipLookup?.resolveAddresses;
