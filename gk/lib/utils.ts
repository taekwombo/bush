export function nn<T>(v: T): Exclude<T, null> {
    if (v === null) {
        throw new Error('Expected value to be non null');
    }

    return v as Exclude<T, null>;
}

/**
 * Ensure that all arguments are not NaN nor Infinity.
 */
export function num(...vals: number[]): void {
    for (const v of vals) {
        if (Math.abs(v) === Infinity || Number.isNaN(v)) {
            throw new Error(`arguments can not be NaN nor Infinity`);
        }
    }
}

/**
 * Ensure at least one of the arguments is non-zero.
 */
export function nz(...vals: number[]): void {
    for (const v of vals) {
        if (v !== 0) {
            return;
        }
    }

    throw new Error('at least one of the arguments must be != 0');
}

export function rad(value: number): number {
    return value / 180 * Math.PI;
}

function getUIDiv(id: string = 'id'): HTMLDivElement {
    const ex = document.querySelector<HTMLDivElement>(`#${id}`);

    if (ex) {
        return ex;
    }

    const ui = document.createElement('div');

    ui.id = id;

    ui.style.position = 'fixed';
    ui.style.backgroundColor = 'transparent';
    ui.style.top = '0';

    ui.onclick = (e) => (e.stopPropagation(), e.preventDefault());

    document.body.appendChild(ui);

    return ui;
}

export function addControl(label: string, opt: Partial<HTMLInputElement>, divId?: string) {
    const ui = getUIDiv(divId);
    
    const { value, onchange, ...rest } = opt;

    const l = document.createElement('label');
    const i = document.createElement('input');

    l.innerText = label;
    l.style.width = '270px';
    l.style.display = 'flex';
    l.style.justifyContent = 'space-between';
    l.style.padding = '2px';
    i.type = 'range';

    Object.assign(i, rest);
    i.value = value || '';

    const s = document.createElement('span');
    s.innerText = value || '-';

    i.onchange = (e) => {
        s.innerText = (e as any).target.value;
        if (onchange) {
            onchange.call(i, e);
        }
    };

    l.appendChild(s);
    l.appendChild(i);
    ui.appendChild(l);
}
