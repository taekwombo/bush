// Use telemetrygen for less rubbish.

if (!import.meta.main) {
    throw new Error("Not main module https://docs.deno.com/runtime/manual/examples/module_metadata#concepts");
}

import { trace } from '@opentelemetry/api';
import { BasicTracerProvider, SimpleSpanProcessor, BatchSpanProcessor } from '@opentelemetry/sdk-trace-base';
import { Resource } from '@opentelemetry/resources';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';

import { faker } from 'faker';
import { Gen } from './gen.ts';
import { Cli } from '../../tss/cli/mod.ts';

const args = new Cli()
    .num('parallelism', { shortName: 'p', defaultValue: 64 })
    .num('traces', { shortName: 't', optional: true })
    .str('otlp-exporter-host', { shortName: 'url', defaultValue: '0.0.0.0' })
    .int('otlp-exporter-grpc-port', { shortName: 'gp', defaultValue: 4317 })
    .int('otlp-exporter-http-port', { shortName: 'hp', defaultValue: 4318 })
    .bool('use-http', { defaultValue: false })
    .bool('otlp-exporter-span-batched', { shortName: 'b', defaultValue: false })
    .str('seed', { optional: true, defaultValue: 'NON_RANDOM' })
    .parse(Deno.args);

const fakerSeed = (() => {
    // http://www.cse.yorku.ca/~oz/hash.html
    const sdbm = args.seed
        .split('')
        .filter((c) => c.length > 0)
        .reduce(
            (h, c) => c.charCodeAt(0) + (h << 6) + (h << 16) - h,
            0,
        );

    return sdbm;
})();

faker.seed(fakerSeed);

const address = (() => {
    const port = args['use-http'] ? args['otlp-exporter-http-port'] : args['otlp-exporter-grpc-port'];
    const url = `http://${args['otlp-exporter-host']}:${port}/v1/traces`;

    return url;
})();

const provider = new BasicTracerProvider({
    resource: new Resource({
        'service.name': faker.internet.domainName(),
        'service.version': faker.system.semver(),
        'service.ip': faker.internet.ip(),
        'service.port': faker.internet.port(),
        'service.a': { a: 'b', c: 2 } as any,
    }),
    spanProcessors: [
        args['otlp-exporter-span-batched']
            ? new BatchSpanProcessor(new OTLPTraceExporter({ url: address }))
            : new SimpleSpanProcessor(new OTLPTraceExporter({ url: address })),
    ],
});

provider.register();

const tracer = trace.getTracer('spanerator', '0.0.1');

const gen = new Gen();

let counter = args.traces ?? faker.number.int({ min: 10, max: 1_000 });
let doneSpans = 0;

// could be nice if all span counts were summed up after tree was executed

const work = new Array(args.parallelism).fill(0).map(async (i) => {
    await new Promise((r) => setTimeout(r, i * 10));

    while (counter > 0) {
        console.log(counter);
        counter -= 1;

        const tree = gen.tree(tracer);
        const cnt = tree.getCounters();

        await tree.execute();

        doneSpans += cnt.spans;
        console.log('Spans:', doneSpans);
    }
});

await Promise.all(work);
await provider.shutdown();
