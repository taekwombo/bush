let fovy = 55;
const aspect = 1;
let near = 1;
let far = 1000;
let cb = () => {};

let translate = Mat4.translate([0, 0, -5]);
let rotate = Mat4.rotate([rad(25), rad(5), rad(5)]);
const perspective = Mat4.perspective(aspect, near, far, fovy);

export function proj(): Float32Array {
    return new Float32Array(
        rotate.mul(translate).mul(perspective).transpose().v.flat()
    );
}

export function onUpdate(up: () => void): void {
    cb = up;
}

function rad(n: number): number {
    return Math.PI * (n / 180);
}

const rotateXCC = Mat4.rotate([rad(5), 0, 0]);
const rotateXC = Mat4.rotate([rad(-5), 0, 0]);
const rotateYCC = Mat4.rotate([0, rad(5), 0]);
const rotateYC = Mat4.rotate([0, rad(-5), 0]);
const rotateZCC = Mat4.rotate([0, 0, rad(5)]);
const rotateZC = Mat4.rotate([0, 0, rad(-5)]);
const translateUp = Mat4.translate([0, -2, 0]);
const translateDown = Mat4.translate([0, 2, 0]);
const translateRight = Mat4.translate([2, 0, 0]);
const translateLeft = Mat4.translate([-2, 0, 0]);
const translateCloser = Mat4.translate([0, 0, 2]);
const translateFurther = Mat4.translate([0, 0, -2]);

window.addEventListener('keydown', (event) => {
    switch (event.key) {
        case 'ArrowUp': {
            if (event.shiftKey) {
                translate = translate.mul(translateFurther);
            } else {
                translate = translate.mul(translateUp);
            }
            break;
        }
        case 'ArrowDown': {
            if (event.shiftKey) {
                translate = translate.mul(translateCloser);
            } else {
                translate = translate.mul(translateDown);
            }
            break;
        }
        case 'ArrowLeft': {
            translate = translate.mul(translateLeft);
            break;
        }
        case 'ArrowRight': {
            translate = translate.mul(translateRight);
            break;
        }
        case 'u': {
            rotate = rotate.mul(rotateXCC);
            break;
        }
        case 'j': {
            rotate = rotate.mul(rotateXC);
            break;
        }
        case 'i': {
            rotate = rotate.mul(rotateYCC);
            break;
        }
        case 'k': {
            rotate = rotate.mul(rotateYC);
            break;
        }
        case 'o': {
            rotate = rotate.mul(rotateZCC);
            break;
        }
        case 'l': {
            rotate = rotate.mul(rotateZC);
            break;
        }
        default: {
            return;
        }
    }

    cb();
});

