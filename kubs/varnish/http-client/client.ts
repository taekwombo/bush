#!/usr/bin/env -S deno run --check --ext=ts --allow-net --allow-env

import { propagation, context, trace, Span, SpanStatusCode } from 'npm:@opentelemetry/api';
import { BasicTracerProvider, ConsoleSpanExporter, SimpleSpanProcessor } from 'npm:@opentelemetry/sdk-trace-base';
import { Resource } from 'npm:@opentelemetry/resources';
import { OTLPTraceExporter } from 'npm:@opentelemetry/exporter-trace-otlp-http';

const provider = new BasicTracerProvider({
    resource: new Resource({
        'service.name': 'http-client',
    }),
    spanProcessors: [
        new SimpleSpanProcessor(new OTLPTraceExporter({
            url: 'http://localhost:4318/v1/traces',
        })),
        // new SimpleSpanProcessor(new ConsoleSpanExporter()),
    ],
});
provider.register();

const tracer = trace.getTracer('http-client', '0.0.1');

function exit(message: string, ...args: any[]): never {
    console.error('%cEXIT', 'color:red', message, ...args);
    Deno.exit(1);
}

function range(min: number, max: number): number[] {
    const arr = [];
    for (let i = min; i <= max; i++) {
        arr.push(i);
    }
    return arr;
}

function randIdx(size: number): number {
    return Math.floor(Math.random() * size);
}

function logTimes(key: string, times: number[]) {
    const pad = (v: string | number) => (typeof v === 'string' ? v : v.toString()).padStart(8, ' ');
    const total = times.length;

    if (total === 0) {
        console.log(`${pad(key)} Request (${pad(0)})`);
        return;
    }

    const meanReq = times.reduce((acc, r) => acc + r, 0) / total;
    const minReq = Math.min(...times);
    const maxReq = Math.max(...times);

    console.log(`${pad(key)} Request (${pad(total)}): avg: ${meanReq.toFixed(2)} - ${minReq}...${maxReq}`);
}

const HOST = Deno.env.get('HOST') ?? exit('HOST environment missing');

type Key = 'ability' | 'type' | 'item' | 'move' | 'pokemon';
type Data = Record<Key, {
    id: number[];
    times: number[];
}>;

async function request(s: Span, url: string): Promise<[null | number, number]> {
    const parent_ctx = trace.setSpan(context.active(), s);
    const span = tracer.startSpan('http.client.request', undefined, parent_ctx);
    const ctx = trace.setSpan(context.active(), span);
    const headers: Record<string, string> = {};
    propagation.inject(ctx, headers);
    const start = Date.now();

    try {
        const response = await fetch(url, { headers });
        if (response.status !== 200) {
            span.setStatus({ code: SpanStatusCode.ERROR });
            if (response.status !== 404) {
                return [null, response.status] as const;
            }
            return [Date.now() - start, response.status] as const;
        } else {
            span.setStatus({ code: SpanStatusCode.OK });
            const body = await response.json();
            return [Date.now() - start, response.status] as const;
        }
    } catch (_) {
        span.setStatus({ code: SpanStatusCode.ERROR });
        return [null, 0];
    } finally {
        span.end();
    }
}

const RESOURCES: Data = {
    ability: {
        id: range(1, 307).concat(range(10001, 10060)),
        times: [],
    },
    type: {
        id: range(1, 19).concat(range(10001, 10002)),
        times: [],
    },
    item: {
        id: range(1, 2229).concat(range(10001, 10002)),
        times: [],
    },
    move: {
        id: range(1, 919).concat(range(10001, 10018)),
        times: [],
    },
    pokemon: {
        id: range(1, 1025).concat(range(10001, 10277)),
        times: [],
    },
};

async function requestRandom(s: Span, count: number, key?: string) {
    const keys = key && key in RESOURCES ? [key] as Key[] : Object.keys(RESOURCES) as Key[];
    function getRandomResource(): [Key, number] {
        const key = keys[randIdx(keys.length)];
        const id = RESOURCES[key].id[randIdx(RESOURCES[key].id.length)];
        return [key, id];
    }

    let requested = 0;
    const promises = new Array(20).fill(0).map(async () => {
        while (requested++ < count) {
            try {
                const [key, id] = getRandomResource();
                const url = `http://${HOST}/pokedex/en/${key}/${id}`;
                const [time, status] = await request(s, url);
                if (time != null) {
                    RESOURCES[key].times.push(time);
                }
            } catch (e) {
                console.error(e);
                requested -= 1;
            }
        }
    });

    await Promise.all(promises);
    for (const key of keys) {
        logTimes(key, RESOURCES[key].times);
    }
}

async function requestAll(s: Span, key?: string) {
    const indexes: Record<Key, number> = {
        ability: 0, type: 0, move: 0, item: 0, pokemon: 0,
    };
    const keys = key && key in RESOURCES ? [key] as Key[] : Object.keys(indexes) as Key[];

    function getResource(): null | [Key, number] {
        for (const key of keys) {
            const idx = indexes[key];

            if (RESOURCES[key].id.length <= idx) {
                continue;
            }

            const id = RESOURCES[key].id[idx];
            indexes[key] = idx + 1;

            return [key, id];
        }

        return null;
    }

    const promises = new Array(15).fill(0).map(async () => {
        while (true) {
            try {
                const resource = getResource();
                if (resource === null) {
                    return;
                }

                const [key, id] = resource;
                const url = `http://${HOST}/pokedex/en/${key}/${id}`;
                const [time, status] = await request(s, url);

                if (time != null) {
                    RESOURCES[key].times.push(time);
                }
            } catch (e) {
                console.error(e);
            }
        }
    });

    await Promise.all(promises);
    for (const key of keys) {
        logTimes(key, RESOURCES[key].times);
    }
}

function int(v: any): number {
    if (typeof v === 'number') {
        if (Number.isNaN(v)) {
            exit('Invalid argument', v);
        }

        return v;
    }
    
    exit('Invalid argument', v);
}

switch (Deno.args[0]) {
    case 'rand':
        await tracer.startActiveSpan('run.rand', async (s) => {
            await requestRandom(s, int(parseInt(Deno.args[1])), Deno.args[2]);
            s.end();
        });
        break;
    case 'all':
        await tracer.startActiveSpan('run.all', async (s) => {
            await requestAll(s, Deno.args[1]);
            s.end();
        });
        break;
}

await provider.shutdown();
