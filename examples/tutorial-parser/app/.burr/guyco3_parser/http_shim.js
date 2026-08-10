export { outgoingHandler as client } from '@bytecodealliance/preview2-shim/http';
export const handler = { handle: () => { throw new Error("not implemented"); } };
export * from '@bytecodealliance/preview2-shim/http';
