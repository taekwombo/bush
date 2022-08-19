import { Canvas, Circle2, Color, Point2, addControl, Polygon2, s2, rad } from '../../lib/mod.js';
import { Vector, Mat4, Model } from '../../lib/3d/mod.js';
import type { Img } from '../../lib/mod.js';

/**
 * It would be nice to add clipping but well.
 * Vectors do not store w component. 😅 
 */

const width = 500;
const height = 500;
const view = {
    a: Canvas.create2(width, height, undefined, false),
    b: Canvas.create2(width, height, 'right', false),
};


type Screen = {
    near: number;
    far: number;
    aspect: number;
    fov: number;
    cameraToWorld: Mat4;
    worldToCamera: Mat4;
    perspective: Mat4;
};

/**
 * Left screen renders the cube.
 * Right screen renders cube and the left camera that is looking at the cube.
 */
let left: Screen;

{ // initialize left screen info
    // In relation to the World origin.
    const cameraPosition = Vector.new(0, 0, -5);
    const cameraToWorld = Mat4
        .scale(Vector.new(1, 1, -1))
        .mul(Mat4.translate(cameraPosition));
    const worldToCamera = cameraToWorld.inverse();

    const near = 1;
    const far = 8;
    const aspect = 1;
    const fov = 60;
    const perspective = Mat4.perspective(aspect, near, far, fov);

    left = {
        near,
        far,
        aspect,
        fov,
        perspective,
        cameraToWorld,
        worldToCamera,
    };
}

let right: {
    near: number;
    far: number;
    aspect: number;
    fov: number;
    sceneTransform: Mat4,
    perspective: Mat4,
};
{ // initialize right screen info
    const sceneTransform = Mat4.STR({
        translate: Mat4.translate([3, 0, 9]),
        rotate: Mat4.rotate([rad(10), rad(35), 0]),
    }).inverse();

    const near = 1;
    const far = width * Math.sqrt(2);
    const aspect = 1;
    const fov = 55;
    const perspective = Mat4.perspective(aspect, near, far, fov);

    right = {
        near,
        far,
        aspect,
        fov,
        perspective,
        sceneTransform,
    };
}

const cube = Model.cube().transform(Mat4.rotate([0, rad(40), 0]));
// Assign colors to cube faces.
const colors = [
    Color.Blue,
    Color.Fuchsia,
    Color.Green,
    Color.Gray,
    Color.Yellow,
    Color.Red,
];
cube.triangles.forEach((t, i) => {
    t.color = colors[Math.floor(i / 2)];
});

function draw(img: Img) {
    const triangles = cube
        .clone()
        .transform(left.worldToCamera)
        .getTrianglesToDraw();

    for (const triangle of triangles) {
        // Variables to draw a normal.
        const start = triangle.center();
        const end = start.clone().add(triangle.normal);

        start.t(left.perspective);
        end.t(left.perspective);

        // Projected triangle.
        const triangleNDC = triangle.transform(left.perspective);

        new Polygon2([
            Point2.fromNDC(img, triangleNDC.a),
            Point2.fromNDC(img, triangleNDC.b),
            Point2.fromNDC(img, triangleNDC.c),
        ], triangle.color).draw(img).fill(img);

        try {
            s2(Point2.fromNDC(img, start), Point2.fromNDC(img, end)).draw(img);
        } catch(_) {
        }
    }
}

function drawRight(img: Img): void {
    // Draw circle representing camera position on the left screen.
    const camera = Vector.new(0, 0, 0).t(right.sceneTransform).t(right.perspective);
    new Circle2(Point2.fromNDC(img, camera), 2, Color.Lime).draw(img);

    // Get coordinates fromof the view planes.
    const t = Math.tan(rad(left.fov) / 2) * left.near;
    const b = -t;
    const r = t * left.aspect;
    const l = -r;

    const nearPlane = [
        Vector.new(t, l, -left.near),
        Vector.new(t, r, -left.near),
        Vector.new(b, r, -left.near),
        Vector.new(b, l, -left.near),
    ];
    const nf = left.far / left.near;
    const farPlane = [
        Vector.new(nf * t, nf * l, -left.far),
        Vector.new(nf * t, nf * r, -left.far),
        Vector.new(nf * b, nf * r, -left.far),
        Vector.new(nf * b, nf * l, -left.far),
    ];

    nearPlane.forEach((v) => v.t(right.sceneTransform).t(right.perspective));
    farPlane.forEach((v) => v.t(right.sceneTransform).t(right.perspective));

    for (let i = 1; i <= nearPlane.length; i++) {
        const s = nearPlane[i - 1];
        const e = nearPlane[i % nearPlane.length];

        s2(Point2.fromNDC(img, s), Point2.fromNDC(img, e)).draw(img);
    }

    for (let i = 1; i <= farPlane.length; i++) {
        const s = Point2.fromNDC(img, farPlane[i - 1]);
        const e = Point2.fromNDC(img, farPlane[i % farPlane.length]);

        s2(s, e).draw(img);
        s2(Point2.fromNDC(img, camera), e, Color.Gray).draw(img);
    }

    // Draw cube
    const triangles = cube
        .clone()
        // Right will just slightly affect how the scene
        // is displayed. Z flip is done by left matrix.
        .transform(left.worldToCamera)
        .transform(right.sceneTransform)
        .getTrianglesToDraw();

    for (const triangle of triangles) {
        const ndc = triangle.transform(right.perspective);

        new Polygon2([
            Point2.fromNDC(img, ndc.a),
            Point2.fromNDC(img, ndc.b),
            Point2.fromNDC(img, ndc.c),
        ], triangle.color).draw(img);
    }
}

function animate() {
    cube.transform(Mat4.rotate([rad(2), rad(2), rad(2)]));
    view.a.clear().drawCb(draw);
    view.b.clear().drawCb(drawRight);

    setTimeout(() => {
        queueMicrotask(animate);
    }, 50);
}

animate();

function update() {
    left.perspective = Mat4.perspective(left.aspect, left.near, left.far, left.fov);
}

addControl('LFOV', {
    value: left.fov.toString(),
    min: '20',
    max: '180',
    onchange: (e) => {
        left.fov = parseInt((e as any).target.value);
        update();
    },
}, 'left');
