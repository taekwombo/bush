import { error } from './log.ts';

export function assert<T>(condition: T | null | undefined | false, message: string, ...args: any[]): T {
    if (!condition) {
        const msg = `Assertion fialed: ${message}`;
        error(msg, ...args);
        throw new Error(msg);
    }
    return condition;
}
