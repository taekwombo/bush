import { Color, Canvas, Bspline2, p2 } from '../../lib/mod.js';
import type { Point2 } from '../../lib/mod.js';

/*
 * Prettier example: http://nurbscalculator.in/
 */

// utils
type UIPoint = {
    element: HTMLElement;
    input: {
        x: HTMLInputElement;
        y: HTMLInputElement;
    };
    point: Point2
};

function updatePointData(uip: UIPoint, x?: number, y?: number): void {
    if (x !== undefined) {
        uip.point.x = x;
        uip.element.style.left = `${x - 4}px`;
    }
    if (y !== undefined) {
        uip.point.y = y;
        uip.element.style.top = `${y - 4}px`;
    }
}

function updatePoint(uip: UIPoint, x: number, y: number): void {
    uip.point.x = x;
    uip.point.y = y;
    
    uip.input.x.value = x.toString();
    uip.input.y.value = y.toString();

    uip.element.style.top = `${y - 4}px`;
    uip.element.style.left = `${x - 4}px`;
}

const activeHandle = (() => {
    let active: UIPoint | null = null;
    return (value?: UIPoint | null) => {
        if (value === null) {
            active = null;
        } else if (value !== undefined) {
            active = value;
        }

        return active;
    };
})();

function createUIPoint(point: Point2, redraw: () => void, points: Point2[], onPointRemove: () => void): UIPoint {
    const element = document.createElement('div');
    const inputs = [document.createElement('input'), document.createElement('input')];
    const [x, y] = inputs;

    const uiPoint = { element, point, input: { x, y } };

    { // Initialise point handle
        element.classList.add('handle');
        element.setAttribute('draggable', 'true');
        element.style.top = `${point.y - 4}px`;
        element.style.left = `${point.x - 4}px`;
        element.style.backgroundColor = point.color!.toHex();

        element.ondragstart = () => {
            activeHandle(uiPoint);
        };

        element.ondragend = () => {
            activeHandle(null);
        };

        element.onclick = (event) => {
            if (event.shiftKey) {
                event.stopPropagation();

                const index = points.findIndex((p) => p === point);
                points.splice(index, 1);
                onPointRemove();

                element.parentElement!.removeChild(element);
                const label = inputs[0].parentElement!;
                label.parentElement!.removeChild(label);
                redraw();
            }
        };

        document.body.appendChild(element);
    }

    const left = document.getElementById('left')!;
    const count = left.children.length;
    const wrapper = document.createElement('label');

    left.appendChild(wrapper);

    wrapper.innerText = 'Point ' + count;
    wrapper.style.backgroundColor = `rgba(${point.color!.r}, ${point.color!.g}, ${point.color!.b}, 0.4)`;

    { // Initialise point inputs
        for (const input of inputs) {
            input.type = 'number';
            input.min = '0';
            input.max = width.toString();
            wrapper.appendChild(input);
        }

        function parseInputValue(input: EventTarget | null) {
            if (!input) {
                return null;
            }

            const value = parseFloat((input as HTMLInputElement).value);
            
            return Number.isNaN(value) ? null : value;
        }

        x.onchange = (event: Event) => {
            const value = parseInputValue(event.target)

            if (value) {
                updatePointData(uiPoint, value);
                redraw();
            }
        };
        y.onchange = (event: Event) => {
            const value = parseInputValue(event.target)

            if (value) {
                updatePointData(uiPoint, undefined, value);
                redraw();
            }
        };

        x.value = point.x.toString();
        y.value = point.y.toString();
    }

    return uiPoint;
}

function createCanvasOnDragOver(redraw: () => void) {
    return function canvasOnDragOver(event: DragEvent): void {
        const { pageX: x, pageY: y } = event;
        const active = activeHandle();

        if (active) {
            updatePoint(active, x, y);
            redraw();
        }
    };
}

// variables
const width = 500;
const height = 500;
const canvas = Canvas.create2(width, height, 'canvas', false);
const points = [
    p2(100, 100, Color.random(0.4)),
    p2(400, 100, Color.random(0.4)),
    p2(400, 400, Color.random(0.4)),
    p2(100, 300, Color.random(0.4)),
];
const curve = new Bspline2({ points, degree: 3 });

function recreateKnotVector() {
    if (curve.degree >= points.length) {
        curve.degree = points.length - 1;
    }

    curve.knots = curve.createKnotVector();
}

function redraw() {
    canvas.clear().drawCb((image) => curve.draw(image));
}

// UI
const uiPoints = points.map((p, _, arr) => createUIPoint(p, redraw, arr, recreateKnotVector));
const onDragOver = createCanvasOnDragOver(redraw);
const degreeInput = document.createElement('input');

{ // Initialise degree input
    const right = document.getElementById('right');
    const label = document.createElement('label');

    right!.appendChild(label);
    label.innerText = 'Curve degree';
    label.appendChild(degreeInput);

    degreeInput.type = 'number';

    degreeInput.min = '1';
    degreeInput.max = (points.length - 1).toString();
    degreeInput.value = curve.degree.toString();

    degreeInput.onchange = () => {
        const value = parseInt(degreeInput.value);

        if (!Number.isNaN(value)) {
            curve.degree = value;
            recreateKnotVector();
            redraw();
        }
    };
}

canvas.canvas.addEventListener('dragover', onDragOver);

window.addEventListener('click', (e) => {
    if (!e.shiftKey) {
        return;
    }

    const { pageX: x, pageY: y } = e;
    const point = p2(x, y, Color.random(0.4));

    points.push(point);
    uiPoints.push(createUIPoint(point, redraw, points, recreateKnotVector));
    recreateKnotVector();

    degreeInput.max = (points.length - 1).toString();
    
    redraw();
});

redraw();

