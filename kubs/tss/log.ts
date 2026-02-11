/// <reference types="./global.d.ts" />

if (Deno.env.get('DEBUG')) {
    globalThis.debugFlag = true;
}

export function debug(message: string, ...args: any[]): void {
    if (!globalThis.debugFlag) {
        return;
    }

    console.log('%cDEBUG', 'color:purple', message, ...args);
}

export function info(message: string, ...args: any[]): void {
    console.info('%cINFO', 'color:blue', message, ...args);
}

export function warn(message: string, ...args: any[]): void {
    console.info('%cWARNING', 'color:yellow', message, ...args);
}

export function error(message: string, ...args: any[]): void {
    console.error('%cERROR', 'color:red', message, ...args);
}

export function fatal(message: string, ...args: any[]): never {
    console.error('%cFATAL', 'color:red', message, ...args);
    Deno.exit(1);
}
