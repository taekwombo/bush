// Use telemetrygen for less rubbish.

if (!import.meta.main) {
    throw new Error("Not main module https://docs.deno.com/runtime/manual/examples/module_metadata#concepts");
}

import { propagation, context, trace, Span, SpanStatusCode, Attributes } from '@opentelemetry/api';
import { BasicTracerProvider, ConsoleSpanExporter, SimpleSpanProcessor, BatchSpanProcessor } from '@opentelemetry/sdk-trace-base';
import { Resource } from '@opentelemetry/resources';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';

import { randomSeeded, randomIntegerBetween } from '@std/random';
import { faker } from 'faker';
import { RootTree, DefaultSpanNode, SpanNode, SpanInfo } from './tree.ts';
import { Gen } from './gen.ts';
import { Cli } from '../../tss/cli/mod.ts';

const args = new Cli()
    .num('parallelism', { shortName: 'p', defaultValue: 64 })
    .num('traces', { shortName: 't', optional: true })
    .str('otlp-exporter-host', { shortName: 'h', defaultValue: '0.0.0.0' })
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

let counter = args.traces || faker.number.int({ min: 10, max: 1_000 });

const work = new Array(args.parallelism).fill(0).map(async () => {
    await new Promise((r) => setTimeout(r, faker.number.int({ min: 25, max: 1_000 })));

    while (--counter >= 0) {
        const tree = gen.tree(tracer);

        tree.display(0, 1);
        await tree.execute();
    }
});

await Promise.all(work);
await provider.shutdown();
