// https://en.wikipedia.org/wiki/ANSI_escape_code#Fe_Escape_sequences
const CSI = String.fromCodePoint(27) + '[';
const RED       = (v: string) => CSI + '1m'  + v + CSI + '0m';
const PURPLE    = (v: string) => CSI + '35m' + v + CSI + '0m';
const YELLOW    = (v: string) => CSI + '33m' + v + CSI + '0m';
const BLUE      = (v: string) => CSI + '34m' + v + CSI + '0m';

const ENCODER = new TextEncoder();

function writeMessage(message: string, args: any[]): void {
    const text = args.reduce((acc, v) => acc + ' ' + Deno.inspect(v), message);
    const encoded = ENCODER.encode(text + '\n');

    Deno.stderr.writeSync(encoded);
}

export function debug(message: string, ...args: any[]): void {
    writeMessage(PURPLE('DEBUG') + ' ' + message, args);
}

export function info(message: string, ...args: any[]): void {
    writeMessage(BLUE('INFO') + ' ' + message, args);
}

export function warn(message: string, ...args: any[]): void {
    writeMessage(YELLOW('WARNING') + ' ' + message, args);
}

export function error(message: string, ...args: any[]): void {
    writeMessage(RED('ERROR') + ' ' + message, args);
}
