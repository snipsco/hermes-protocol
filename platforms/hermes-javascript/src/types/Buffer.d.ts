export {}

declare global {
    export interface Buffer {
        deref: () => any;
        ref: () => Buffer;
        isNull: () => boolean;
    }
}