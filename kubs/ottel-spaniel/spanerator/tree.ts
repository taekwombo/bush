import { context, trace } from '@opentelemetry/api';
import type { Tracer, Attributes, Span } from '@opentelemetry/api';

function sleep(ms: number): Promise<void> {
    return new Promise((r) => setTimeout(r, ms));
}

export type Events = Array<[string, number, Attributes]>;

export class SpanInfo {
    minDuration: number;
    name: string;
    attributes: Attributes;
    events: Events;

    constructor(name: string, minDuration: number, attributes: Attributes, events: Events) {
        this.name = name;
        this.minDuration = minDuration;
        this.attributes = attributes;
        this.events = events;
    }
}

export abstract class SpanNode {
    tracer: Tracer;
    info: SpanInfo;
    children: Array<Array<SpanNode>>;

    constructor(tracer: Tracer, info: SpanInfo, children: Array<Array<SpanNode>>) {
        this.tracer = tracer;
        this.info = info;
        this.children = children;
    }

    abstract execute(span?: Span): Promise<void>;

    static async runEvents(span: Span, events: Events): Promise<void> {
        for (const [name, duration, attributes] of events) {
            await sleep(duration);
            span.addEvent(name, attributes);
        }
    }

    static async runChildren(span: Span, children: Array<Array<SpanNode>>): Promise<void> {
        for (const batch of children) {
            await Promise.all(batch.map((child) => child.execute(span)));
        }
    }

    display(depth: number = 0, max: number | null = null): this {
        if (max && max <= depth) {
            return this;
        }

        const pad = '│ '.repeat(depth);
        const displayMs = (value: number) => value.toString().padStart(3, ' ') + 'ms';

        const line = `${displayMs(this.info.minDuration)} ${pad}${this.info.name}`;

        console.log(line);

        this.children.forEach((batch) => {
            batch.forEach((child) => {
                child.display(depth + 1, max);
            });
        });

        return this;
    }
}

export class DefaultSpanNode extends SpanNode {
    constructor(tracer: Tracer, info: SpanInfo, children: Array<Array<SpanNode>>) {
        super(tracer, info, children);
    }

    async execute(s?: Span): Promise<void> {
        await sleep(this.info.minDuration);

        const opts = { attributes: this.info.attributes };
        const span = this.tracer.startSpan(this.info.name, opts, s && trace.setSpan(context.active(), s));

        await Promise.all([
            SpanNode.runChildren(span, this.children),
            SpanNode.runEvents(span, this.info.events),
        ]);

        span.end();
    }
}

export class RootTree extends SpanNode {
    constructor(tracer: Tracer, info: SpanInfo, children: Array<Array<SpanNode>>) {
        super(tracer, info, children);
    }

    async execute(s?: Span): Promise<void> {
        const opts = { root: true, attributes: this.info.attributes };
        const span = this.tracer.startSpan(this.info.name, opts, s && trace.setSpan(context.active(), s));

        await Promise.all([
            SpanNode.runChildren(span, this.children),
            SpanNode.runEvents(span, this.info.events),
        ]);

        span.end();
        console.log(span.spanContext().traceId, this.info.name);
    }
}
