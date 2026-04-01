import type { Attributes, AttributeValue, Tracer } from '@opentelemetry/api';
import type { Events, SpanNode } from './tree.ts';

import { assert } from '@std/assert';
import { SpanInfo, RootTree, DefaultSpanNode } from './tree.ts';
import { faker } from 'faker';

export class Gen {
    static attrName(): string {
        return faker.system
            .fileName({ extensionCount: faker.number.int({ min: 1, max: 3 }) })
            .toLowerCase()
            .replaceAll(/[_-]/g, '.');
    }

    static attrNames(size: number): string[] {
        return new Array(size).fill(0).map(Gen.attrName);
    }

    static attrValue(): AttributeValue {
        switch (faker.number.int({ min: 1, max: 4 })) {
            case 1:
                return faker.location.latitude();
            case 2:
                return faker.system.cron();
            case 3:
                return faker.number.int();
            default:
                return faker.system.fileExt();
        }
    }

    static attributes(keys: string[], attempts: number) {
        let values: Map<string, AttributeValue> = new Map();

        for (let i = 0; i < attempts; i++) {
            values.set(faker.helpers.arrayElement(keys), Gen.attrValue());
        }

        return Object.fromEntries(values.entries());
    }

    static spanNames(length: number): string[] {
        function httpName(): string {
            const method = faker.helpers.arrayElement(['GET', 'POST', 'PUT', 'DELETE', 'PATCH']);
            const path = faker.system.fileName({ extensionCount: faker.number.int({ min: 1, max: 3 }) })
                .toLowerCase()
                .replaceAll(/[\._-]/g, '/');

            return `${method} ${path}`;
        }

        function randomName(): string {
            return faker.internet.domainName().replaceAll(/[-_]/g, '.');
        }

        return new Array(length).fill(0).map((_, idx) => {
            if (faker.number.int({ min: 0, max: idx }) % 3 == 0) {
                return httpName();
            }

            return randomName();
        });
    }

    static duration(): number {
        return faker.number.int({ min: 5, max: 45 });
    }

    static events(eventAttrNames: string[], length: number): Events {
        return new Array(length)
            .fill(0)
            .map(() => {
                const name = faker.word.words({ count: { min: 1, max: 4 } });
                const duration = Gen.duration();
                const attributes = Gen.attributes(eventAttrNames, faker.number.int({ min: 0, max: 4 }));

                return [name, duration, attributes];
            });
    }

    names: Names;
    cnt: Counter;

    constructor() {
        const rootSpans = Gen.spanNames(faker.number.int({ min: 4, max: 24 }));
        const childSpans = new Array(faker.number.int({ min: 8, max: 24 }))
            .fill(0)
            .map(() => Gen.spanNames(faker.number.int({ min: 8, max: 48 })));


        this.cnt = {
            created: 0,
            maximum: 0,
            names: childSpans.map((v) => v.length),
        };

        this.names = {
            rootSpans,
            childSpans,
            attributes: {
                span: Gen.attrNames(faker.number.int({ min: 2, max: 16 })),
                event: Gen.attrNames(faker.number.int({ min: 2, max: 6 })),
                resource: Gen.attrNames(faker.number.int({ min: 8, max: 18 })),
            },
        };
    }

    spanInfo(depth: number | null = null) {
        if (depth !== null) {
            assert(depth < this.names.childSpans.length);
        }

        const names = depth == null ? this.names.rootSpans : this.names.childSpans[depth];

        return new SpanInfo(
            faker.helpers.arrayElement(names),
            Gen.duration(),
            Gen.attributes(this.names.attributes.span, faker.number.int({ min: 0, max: this.names.attributes.span.length })),
            Gen.events(this.names.attributes.event, faker.number.int({ min: 0, max: 4 })),
        );
    }

    child(tracer: Tracer, depth: number): SpanNode {
        this.cnt.created += 1;

        const info = this.spanInfo(depth);
        const children = this.children(tracer, depth + 1);

        return new DefaultSpanNode(tracer, info, children);
    }

    children(tracer: Tracer, depth: number): Array<Array<SpanNode>> {
        const result: Array<Array<SpanNode>> = [];
        const maxDepth = this.names.childSpans.length - 1;

        if (depth > maxDepth) {
            return result;
        }

        const namesLeft = this.cnt.names.slice(depth).reduce((a, b) => a + b);
        const spansLeft = () => this.cnt.maximum - this.cnt.created;

        let current_depth = depth;

        while (faker.number.float() >= (this.cnt.created / this.cnt.maximum)) {
            if (faker.number.float() >= 0.5) {
                current_depth++;
            }

            if (current_depth > maxDepth) {
                break;
            }

            if (spansLeft() === 0) {
                break;
            }

            const batch = [];

            do {
                batch.push(this.child(tracer, current_depth));

                if (spansLeft() === 0) {
                    break;
                }
            } while (faker.number.float() >= 0.65);

            result.push(batch);
        }

        return result;
    }

    tree(tracer: Tracer, maxSpans: number = faker.number.int({ min: 4, max: 64 })): SpanNode {
        this.cnt.created = 1;
        this.cnt.maximum = maxSpans;

        const info = this.spanInfo(null);
        const children = this.children(tracer, 0);

        return new RootTree(tracer, info, children);
    }
}

type Names = {
    rootSpans: string[];
    /* 
     * [
     *  [...], // 1st level span names
     *  [...], // 2nd level span names
     * ]
     */
    childSpans: Array<string[]>;

    attributes: {
        span: string[];
        event: string[];
        resource: string[];
    };
}

type Counter = {
    created: number;
    maximum: number;
    names: number[];
};
