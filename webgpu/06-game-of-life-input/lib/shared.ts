export function assert<T>(value: T): Exclude<T, undefined> {
    if (value === undefined) {
        throw new Error('Value must be assigned');
    }

    return value as Exclude<T, undefined>
}

export function cmp(usage: number, flag: number): boolean {
    return (usage & flag) === flag;
}

export function assert_cmp(usage: number, flag: number): void | never {
    if((usage & flag) !== flag) {
        const u = usage.toString(16);
        const f = flag.toString(16);
        throw new Error(`Required flag (${f}) is not present in usage (${u})`);
    };
}

export function viewportData(canvas: HTMLCanvasElement): Float32Array {
    return new Float32Array([canvas.width, canvas.height]);
}

export function cellSize(): number {
    return 10;
}

export function gridSize(canvas: HTMLCanvasElement): [width: number, height: number] {
    const size = cellSize();
    const { width, height } = canvas;

    return [Math.ceil(width / size), Math.ceil(height / size)];
}

export function gridData(canvas: HTMLCanvasElement): Float32Array {
    const [x, y] = gridSize(canvas);

    const data = new Array(1 + x * y).fill(0);
    data[0] = x * y;

    return new Float32Array(data);
}

export function updateCanvas(canvas: HTMLCanvasElement): void {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
}

export function debounce<A extends any[]>(fn: (...args: A) => void, t: number): (...args: A) => void {
    let x: number | null = null;
    
    return (...a: A): void => {
        if (x) clearTimeout(x);
        x = setTimeout(() => {
            fn(...a);
            x = null;
        }, t);
    };
}

