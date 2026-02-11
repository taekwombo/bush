const prefix = `deno:${Deno.pid()}`;
let counter: number = 0;

function generateRequestId(): string {
    return `${prefix}:${counter++}`;
}

export class HttpHeaders {
    headerMap: Map<string, string>;

    private constructor() {
        this.headerMap = new Map();
        this.addRequestId();
        this.addUserAgent();
    }

    private add(name: string, value: string): this {
        this.headerMap.set(name, value);
        return this;
    }

    private addUserAgent(): this {
        return this.add('User-Agent', 'deno');
    }

    private addRequestId(): this {
        return this.add('X-Request-Id', generateRequestId());
    }

    public json(): this {
        return this.contentType('application/json');
    }

    public contentType(mrMime: string): this {
        return this.add('Content-Type', mrMime);
    }

    public get value(): Record<string, string> {
        const headers: Record<string, string> = {};

        for (const [name, value] of this.headerMap.entries()) {
            headers[name] = value;
        }

        return headers;
    }
}
