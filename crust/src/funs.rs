//! https://www.youtube.com/watch?v=dHkzSZnYXmk
//! https://doc.rust-lang.org/reference/types/function-item.html
//! https://doc.rust-lang.org/reference/types/closure.html
//! https://doc.rust-lang.org/reference/types/function-pointer.html

#[allow(dead_code, unused_imports)]
fn function_traits() {
    use std::ops::{Fn, FnMut, FnOnce};

    // Function item.
    fn foo<T>() {}
    // Function pointer.
    let fp: fn() = foo::<u8>;

    fn fn_ptr(_callback: fn()) {}

    fn call_once<F>(fun: F)
    where
        F: FnOnce(),
    {
        fun();
    }

    let value = 1;

    fn_ptr(foo::<u8>);
    fn_ptr(fp);
    // Non capturing closures are coercible to function pointers.
    fn_ptr(|| {});
    // Compile error.
    // note: closures can only be coerced to `fn` types if they do not capture any variables.
    // fn_ptr(|| { let _v = value; });
    // Box<Fn()> is implemented due to unsized_locals
    // https://doc.rust-lang.org/beta/unstable-book/language-features/unsized-locals.html
    // https://poignardazur.github.io/2022/02/23/rust-unsized-vars-analysis/

    call_once(foo::<u8>);
    call_once(fp);
    call_once(|| {});
    call_once(move || {
        let _v = value;
    });
}

#[cfg(test)]
mod tests {
    use std::mem::size_of_val;

    #[test]
    fn function_item_type() {
        fn crazy_fun() {}

        let callback = crazy_fun; // value: fn function() - Function item.
        let fn_pointer: fn() = crazy_fun; // value: fn() - Function pointer.

        assert_eq!(0, size_of_val(&callback));
        assert_eq!(0, size_of_val(&crazy_fun));
        assert_eq!(8, size_of_val(&fn_pointer));
    }
}
