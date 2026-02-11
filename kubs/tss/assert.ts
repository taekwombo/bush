import { fatal } from './log.ts';

export function assert<T>(condition: T | null | undefined | false, message: string, ...args: any[]): T {
    if (!condition) fatal(message, ...args);
    return condition;
}
