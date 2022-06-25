export const cellSize = 20.0;

export const GridRule = {
    Random() {
        return Math.floor(Math.random() + 0.3);
    },
    Blank() {
        return 0.0;
    },
    Pattern(x, y, maxX, maxY) {
        if (y % 2 === 0 && x % 2 === 0) {
            return 1.0;
        } else if (y % 2 !== 0 && x % 2 !== 0) {
            return 1.0;
        } else {
            return 0.0;
        }
    },
    LastCol(x, y, maxX, maxY) {
        if (y === Math.floor(maxY / 2) || x === Math.floor(maxX / 2)) {
            return 1.0;
        }

        return 0.0;
    }
};

export function getGridSize() {
    const x = Math.ceil(window.innerWidth / cellSize);
    const y = Math.ceil(window.innerHeight / cellSize);

    return { x, y };
}

export function createGrid(rule = GridRule.Random) {
    const { x, y } = getGridSize();
    const grid = new Array(1 + (x * y));

    grid[0] = x * y;

    for (let i = 0; i < y; i++) {
        for (let j = 0; j < x; j++) {
            const index = 1 + j + (i * x);

            grid[index] = rule(j, i, x, y);
        }
    }

    return grid;
}
