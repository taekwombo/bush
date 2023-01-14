//! https://www.youtube.com/watch?v=xcygqF5LVmM
//! https://doc.rust-lang.org/reference/type-layout.html
//! Waker vtable example: https://cfsamson.github.io/books-futures-explained/3_waker_context.html

#[allow(dead_code)]
fn monomorph_or_not() {
    fn strlen_impl_trait(v: impl AsRef<str>) -> usize {
        v.as_ref().len()
    }
    fn strlen_generic<T>(v: T) -> usize
    where
        T: AsRef<str>,
    {
        v.as_ref().len()
    }
    fn strlen_dyn(v: &dyn AsRef<str>) -> usize {
        v.as_ref().len()
    }

    // Caller cannot specify parameter type.
    strlen_impl_trait("string");
    strlen_impl_trait(String::from("string"));
    // Caller can specify parameter type.
    strlen_generic::<&'static str>("string");
    strlen_generic(String::from("string"));

    strlen_dyn(Box::new("").as_ref());
    strlen_dyn(&"");
}

pub trait FastFood {
    fn sauce(&self) -> &'static str;

    // Adding method without &self paremeter renders the trait unusable as a trait object.
    // VTable cannot be propertly generated.
    // Adding "where Self: Sized" bound to the method means it cannot be applied to trait objects.
    // Therefore it allows for partial object safety. Type can be converted into trait object but
    // only some methods can be invoked.
    // https://doc.rust-lang.org/reference/items/traits.html#object-safety
    // fn test() -> () {}
    // fn test() where Self: Sized -> () {}
}

impl FastFood for &str {
    fn sauce(&self) -> &'static str {
        "&str sauce"
    }
}

impl FastFood for String {
    fn sauce(&self) -> &'static str {
        "String sauce"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Accept references to anything that implements FastFood trait.
    fn get_fat_size(_: &dyn FastFood) -> usize {
        // Returns 16.
        // Two pointers: (data_pointer: usize, vtable_pointer: usize).
        std::mem::size_of::<&dyn FastFood>()
    }

    #[test]
    fn size_of_dyn_ref() {
        assert_eq!(16, get_fat_size(&"s"));
    }

    #[test]
    fn constructing_trait_object() {
        /// Constructs a fat pointer to u8 as FastFood.
        /// See std::ptr::DynMetadata.
        /// https://rust-lang.github.io/rfcs/2580-ptr-meta.html
        use std::mem::{align_of, size_of, size_of_val, transmute};

        fn sauce(_value: &u8) -> &'static str {
            "u8 sauce"
        }

        let data = 10_u8;
        let vtable = vec![
            0, // Destructor pointer. In this case we never drop the value
            // because it is behind a reference. Box would require the pointer.
            size_of::<u8>(),  // Size of data type.
            align_of::<u8>(), // Alignment of data type.
            sauce as usize,   // Following methods of the trait.
                              // Can be also provided method from another type impl for the trait.
                              // Be careful about parameter sizes.
                              // <&str as FastFood>::sauce as usize,
        ];

        // Can be (*const u8, *const usize) or (usize, usize) when "&data as *const u8" is
        // casted to usize.
        let trait_obj = (&data as *const u8, vtable.as_ptr());
        // Transmute two usize pointers next to each other into a &dyn FastFood.
        let fat_pointer =
            unsafe { transmute::<(*const u8, *const usize), &dyn FastFood>(trait_obj) };

        assert_eq!("u8 sauce", fat_pointer.sauce());
        assert_eq!(16, size_of::<&dyn FastFood>()); // Size of dyn pointer.
        assert_eq!(16, size_of_val(&trait_obj)); // Size of two neighbouring pointers.
        assert_eq!(1, size_of_val(fat_pointer)); // Size of u8.
    }
}
