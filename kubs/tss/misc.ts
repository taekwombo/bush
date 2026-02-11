import { debug } from './log.ts';

export function sleep(ms: number): Promise<void> {
    debug('Sleeping', DurationFormat.millis(ms));

    return new Promise((r) => setTimeout(r, ms));
}

export class DurationFormat {
    public static millis(millis: number): string {
        const rem = millis % 1000;

        const chunks: Array<[string, number, number]> = [
            ['s', 1000, 60],
            ['m', 60, 60],
            ['h', 60, 24],
            ['d', 24, Number.POSITIVE_INFINITY],
        ];

        const parts: string[] = [`${rem}ms`];

        let current = millis;

        for (const [suffix, divBy, modBy] of chunks) {
            current = current / divBy;

            if (current < 1) {
                break;
            }

            let value = Math.floor(current);
            if (Number.isFinite(modBy)) {
                value = value % modBy;
            }

            parts.push(value.toString() + suffix);
        }

        return parts.reverse().join(' ');
    }
}

export class DateFormat {
    public static ymd(v: Date): string {
        const year = v.getFullYear().toString();
        const month = (v.getMonth() + 1).toString().padStart(2, '0')
        const day = v.getDate().toString().padStart(2, '0');
        return `${year}-${month}-${day}`;
    }
}

export class StringFormat {
    public static stripMargin(str: string): string {
        const lines = str.split('\n');
        const firstIndex = lines.findIndex((l) => l.trim().length > 0);
        const lastIndex = lines.findLastIndex((l) => l.trim().length > 0);

        const pad = /^(?<pad>\s*)/.exec(lines[firstIndex])?.groups?.pad || '';

        return lines
            .slice(firstIndex, lastIndex + 1)
            .map((l) => l.replace(pad, ''))
            .join('\n');
    }
}

export class Dict<K extends string | number, V> {
    public static new<V>(v: Record<string, V>): Dict<string, V> {
        return new Dict(Object.entries(v));
    }

    private inner: Array<[K, V]>

    private constructor(inner: Array<[K, V]>) {
        this.inner = inner;
    }

    public filter(cond: (k: K, v: V) => boolean): this {
        this.inner = this.inner.filter(([k, v]) => cond(k, v));

        return this;
    }

    public filterKeys(cond: (k: K) => boolean): this {
        this.inner = this.inner.filter(([k, _]) => cond(k));

        return this;
    }

    public filterValues(cond: (v: V) => boolean): this {
        this.inner = this.inner.filter(([_, v]) => cond(v));

        return this;
    }

    public map(map: (k: K, v: V) => [K, V]): this {
        this.inner = this.inner.map(([k, v]) => map(k, v));

        return this;
    }

    public mapKeys(map: (k: K) => K): this {
        this.inner = this.inner.map(([k, v]) => [map(k), v]);

        return this;
    }

    public mapValues(map: (v: V) => V): this {
        this.inner = this.inner.map(([k, v]) => [k, map(v)]);

        return this;
    }

    public get value(): Record<K, V> {
        return Object.fromEntries(this.inner) as Record<K, V>;
    }
}
