import { addControl, Circle2, Canvas, p2, Segment2, Color, rad } from '../../lib/mod.js';
import { Vector, Mat4, Triangle } from '../../lib/3d/mod.js';
import type { Img } from '../../lib/mod.js';

/**
 * Canvas
 *
 * X 0 ←─────────────→ 500
 * Y 0 ←─────────────→ 500
 *
 *    TOP
 *    Y 0
 *    ↑
 *    │
 *    │    LEFT                 RIGHT
 *    │    X 0 ←──────────────→ X 500
 *    │
 *    │
 *    ↓
 *    Y 500
 *    BOTTOM
 *
 * World
 *
 * X -250 ←─────────────→ 250
 * Y -250 ←─────────────→ 250
 * Z -250 ←─────────────→ 250
 *
 *    UP       FAR (visible)
 *    Y 250    Z 250
 *    ↑        ↑
 *    │        │
 *    │        │    LEFT                    RIGHT
 *    │        │    X -250 ←──────────────→ X 250
 *    │        │
 *    │        │
 *    ↓        ↓
 *    Y -250   Z -250
 *    DOWN     BEHIND (hidden)
 *
 * Camera
 *
 * X -250 ←─────────────→ 250
 * Y -250 ←─────────────→ 250
 * Z -250 ←─────────────→ 250
 *
 *    UP      FAR (visible)
 *    Y 250   Z -250
 *    ↑       ↑
 *    │       │
 *    │       │    LEFT                    RIGHT
 *    │       │    X -250 ←──────────────→ X 250
 *    │       │
 *    │       │
 *    ↓       ↓
 *    Y -250  Z 250
 *    DOWN    BEHIND (hidden)
 */

const width = 500;
const height = 500;
const canvas = Canvas.create2(width, height);
const canvasR = Canvas.create2(width, height, 'right');
const points: Vector[] = [];
const camera = Vector.new(250, 250, 250);

let camR = 0;   // Camera rotation along Y axis.
let far = 100;  // Far plane distance
let near = 10;  // Near plane distance
let fov = 90;   // Field of view in degrees

function drawCamera(img: Img): void {
    // Camera position
    const c = p2(camera.x, camera.z);
    
    // Draw camera Z axis
    Segment2.drawSegment(img, c, p2(c.x, 0), Color.Blue);
    Segment2.drawSegment(img, c, p2(c.x, 500), Color.Blue);
    // Draw camera X axis
    Segment2.drawSegment(img, p2(0, c.y), c, Color.Red);
    Segment2.drawSegment(img, p2(500, c.y), c, Color.Red);

    // Draw circle at the camera position.
    new Circle2(c, 2, Color.Lime).draw(img);
    img.floodFill(c.x, c.y, Color.Lime);

    // Calculate half of length of near and far planes
    const nearHL = Math.round(near * Math.tan(rad(fov / 2)));
    const farHL = Math.round(far * Math.tan(rad(fov / 2)));

    const na = p2(c.x - nearHL, c.y - near).rotate(camR, c);
    const nb = p2(c.x + nearHL, c.y - near).rotate(camR, c);
    const fa = p2(c.x - farHL, c.y - far).rotate(camR, c);
    const fb = p2(c.x + farHL, c.y - far).rotate(camR, c);

    // Calculate field of view line segment positions
    // Length of the field of view line
    const fovL = Math.sqrt(far ** 2 + farHL ** 2);
    // Field of view start point
    const fov1 = p2(c.x, c.y);
    // Field of view end point - left
    const fov2 = p2(c.x, c.y - fovL);
    // Field of view end point - right
    const fov3 = fov2.clone();

    // Rotate end points along camera rotation and then rotate by half of field of view.
    fov2.rotate(camR, c).rotate(fov / 2, c);
    fov3.rotate(camR, c).rotate(fov / -2, c);

    // Draw field of view
    Segment2.pipeDraw(img, [[fov1, fov2], [fov1, fov3]], Color.Lime);
    // Draw near and far planes
    if (na.x !== nb.x) {
        Segment2.drawSegment(img, na, nb, Color.Fuchsia);
    } else {
        img.drawPoint(~~na.x, ~~na.y, Color.Fuchsia);
    }
    Segment2.drawSegment(img, fa, fb, Color.Fuchsia);
}

// Transforms point from Canvas coordinates to World coordinates.
const canvasToWorld = Mat4
    .translate(Vector.new(-250, -250, -250))
    .mul(Mat4.scale(Vector.new(1, 1, -1)));

let cameraPosition: Vector,
    cameraToWorld: Mat4,    // Transformation of the camera position in world space.
    worldToCamera: Mat4,    // Transformation to the camera space (camera at (0,0,0)).
    perspective: Mat4;      // Perspective transformation matrix.

function createPerspectiveMatrix(): Mat4 {
    const f = far;
    const n = near;

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/building-basic-perspective-projection-matrix
    // Maps z inside view frustum to <0; 1>
    // const m22 = -f / (f - n);
    // const m23 = -(f * n) / (f - n);
    // Scales the x and y coordinates depending on the field of view.
    // const m01 = 1 / (Math.tan(rad(fov / 2))); // This works assuming the near = 1
    // const m12 = 1 / (Math.tan(rad(fov / 2))); // This works assuming the near = 1

    const t = Math.tan(rad(fov / 2)) * n;
    const b = -t;
    const r = t * (width / height);
    const l = -r;

    // Maps z inside view frustum to <-1; 1>
    const zz = -((f + n) / (f - n));
    const wz = -((2 * f * n) / (f - n));
    // Scales x and y coordinates
    const zx = (r + l) / (r - l);
    const zy = (t + b) / (t - b);
    const x = (2 * n) / (r - l);
    const y = (2 * n) / (t - b);

    return Mat4.new([
        x,    0,    0,    0,
        0,    y,    0,    0,
        zx,   zy,   zz,  -1,
        0,    0,    wz,   0,
    ]);
}

function leftFrame(img: Img): void {
    drawCamera(img);

    // console.log('[WORLD]: camera position', cameraPosition);

    for (const point of points) {
        // pw in the world coordinates
        const pw = point.clone().t(canvasToWorld);
        const pc = pw.clone().t(worldToCamera);
        const pp = pc.clone().t(perspective);

        // console.log('[CANVAS]: point', point.toString());
        // console.log('[WORLD}: point', pw.toString());
        // console.log('[CAMERA]: point', pc.toString());
        // console.log('[NDC]: point', pp.toString());

        const color = visible(pp)
            ? pcol(pp)
            : Color.Gray;

        const p2d = p2(point.x, point.z, color);
        const c = new Circle2(p2d, 2, color);

        c.draw(img);
        img.floodFill(p2d.x, p2d.y, color);
    }
}

// Default triangle rendered on the right frame.
const triangle = Triangle
    .new(-1, -1, 0, -1, 1, 0, 1, 1, 0) // Looks like: ◸
    .transform(Mat4.rotate([rad(20), 0, rad(25)]));

// Stores triangles displayed on right canvas.
const triangles: Triangle[] = [];
// Triangle rotation transformation - triangles are rotated on each draw call.
const triangleRotate = Mat4.rotate([0, rad(4), 0]);

// Utility variables for transforming NDC point to point on Canvas.
const _wh = width * 0.5;
const _hh = height * 0.5;
// Note: Since the canvas Y coordinate is flipped in relation to the Camera Y coordinate - it needs to be reversed here.
const p2c = (v: Vector) => p2((v.x + 1.0) * _wh, height - ((v.y + 1.0) * _hh))

function rightFrame(img: Img): void {
    if (!triangles.length) {
        return;
    }

    triangles.forEach((t) => t.transform(triangleRotate));

    for (let i = 0; i < points.length; i++) {
        const point = points[i];

        const pointWorld = point.clone().t(canvasToWorld);
        const triangleToWorld = Mat4.translate(pointWorld);
        const pointNDC = pointWorld.t(worldToCamera).t(perspective);

        if (!visible(pointNDC)) {
            continue;
        }

        // console.log('[MODEL]: triangle', triangles[i].toString());
        const t = triangles[i].clone().transform(triangleToWorld);
        // console.log('[WORLD]: triangle', t.toString());
        t.transform(worldToCamera);
        // console.log('[CAMERA]: triangle', t.toString());
        t.transform(perspective);
        // console.log('[NDC]: triangle', t.toString());

        const a = p2c(t.a);
        const b = p2c(t.b);
        const c = p2c(t.c);

        Segment2.pipeDraw(img, [a, b, c, a]);
    }
}

function update() {
    cameraPosition = camera.clone().t(canvasToWorld);

    // camera coordinates transformation in world coordinates
    cameraToWorld =  
        // Flip z axis 
        Mat4.scale(Vector.new(1, 1, -1))
        // Rotate camera in the world
        .mul(camR ? Mat4.rotate([0, rad(camR), 0]) : Mat4.identity())
        // Move camera to its position in the world
        .mul(Mat4.translate(cameraPosition));

    // Transforms world coordinates to camera coordinates
    worldToCamera = cameraToWorld.inverse();

    perspective = createPerspectiveMatrix();

    // Redraw left canvas on update
    canvas.clear().drawCb(leftFrame);
}

/** Verify whether point is in clip space */
function visible(v: Vector): boolean {
    return v.x >= -1 && v.x <= 1
        && v.y >= -1 && v.y <= 1
        && v.z >= -1 && v.z <= 1;
}

/** Create point color in the left canvas */
function pcol(v: Vector): Color {
    const zColor = Color.Yellow;
    const z = (v.z + 1.0) * 0.2;

    let r = 255 - (255 - zColor.r) * z;
    let g = 255 - (255 - zColor.g) * z;
    let b = 255 - (255 - zColor.b) * z;

    return new Color(r, g, b, 255);
}

/** Continously animate right panel */
function draw() {
    try {
        canvasR.clear().drawCb(rightFrame);
    } catch (error) {
        console.error(error);
    }

    setTimeout(() => {
        queueMicrotask(draw);
    }, 50);
}

update();
draw();

window.addEventListener('keypress', (event) => {
    if (event.key === 'r') {
        points.length = 0;
        triangles.length = 0;
    }

    update();
});

window.addEventListener('click', (event) => {
    const { width, height } = canvas;
    const { clientWidth: xMax, clientHeight: yMax } = canvas.canvas;
    const { offsetX: x, offsetY: y } = event;

    const px = width * (x / xMax);
    const py = height * (y / yMax);

    if (px >= 500 || px <= 0 || py <= 0 || py >= 500) {
        return;
    }

    const pointInCanvasSpace = Vector.new(px, 250, py);

    points.push(pointInCanvasSpace);
    triangles.push(triangle.clone());

    update();
});

addControl('FOV', { value: fov.toString(), min: '5', max: '179', onchange: (e) => {
    fov = parseInt((e as any).target.value);
    update();
}});

addControl('FAR', { value: far.toString(), min: '5', max: '500', onchange: (e) => {
    far = parseInt((e as any).target.value);
    update();
}});

addControl('NEAR', { value: near.toString(), min: '0.5', max: '200', onchange: (e) => {
    near = parseInt((e as any).target.value);
    update();
}});

addControl('C-ROT', { value: camR.toString(), min: '0', max: '360', onchange: (e) => {
    camR = parseInt((e as any).target.value);
    update();
}});

addControl('C-Z', { value: camera.z.toString(), min: '0', max: '500', onchange: (e) => {
    camera.z = parseInt((e as any).target.value);
    update();
}});

addControl('C-X', { value: camera.x.toString(), min: '0', max: '500', onchange: (e) => {
    camera.x = parseInt((e as any).target.value);
    update();
}});
