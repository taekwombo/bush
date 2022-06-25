export function debounce(f, t) {
    let x = null;
    
    return (...a) => {
        if (x) clearTimeout(x);
        x = setTimeout(() => {
            f(...a);
            x = null;
        }, t);
    };
}

