//! https://www.youtube.com/watch?v=TJOFSMpJdzg

use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Pudlo<T> {
    value: NonNull<T>,
    // Tells the compiler it does drop the T.
    // Type holds only a pointer to T - since it's not owning a T in the eyes in type system it
    // should not drop it.
    _drops_t: PhantomData<T>,
}

impl<T> Pudlo<T> {
    pub fn new(value: T) -> Self {
        let val = Box::new(value);

        Self {
            value: unsafe { NonNull::new_unchecked(Box::into_raw(val)) },
            _drops_t: PhantomData,
        }
    }

    pub fn get_raw(&self) -> *const T {
        unsafe { self.value.as_ref() }
    }
}

impl<T> std::ops::Deref for Pudlo<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: pointer created from Box, wasn't freed, aligned.
        unsafe { self.value.as_ref() }
    }
}

impl<T> std::ops::DerefMut for Pudlo<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: pointer created from Box, wasn't freed, aligned.
        unsafe { &mut *self.value.as_mut() }
    }
}

// `#[may_dangle]` tells the compiler that this drop implementation does not access the T
unsafe impl<#[may_dangle] T> Drop for Pudlo<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.value.as_mut());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let val = Vec::<u8>::new();
        let mut p = Pudlo::new(val);

        {
            let ptr = p.get_raw();
            assert!(unsafe { (*ptr).len() == 0 })
        }

        assert!(p.len() == 0);
        p.push(10);
        assert!(p.len() == 1);
    }

    #[test]
    #[allow(unused_variables, unused_mut)]
    fn too_restrictive() {
        //! Does not compile if `#[may_dangle]` is not present.
        //! Pudlo<T> has its own Drop implementation, therefore compiler assumes that when Pudlo<T> is dropped
        //! then T is accessed.
        //! Therefore, immutable (shared) borrow in `assert(value === 100)` conflicts with mutable
        //! borrow at the end of this function when `drop(pud)` is called.
        let mut value = 100;
        let mut pud = Pudlo::new(&mut value);

        assert!(value == 100);
    }

    #[cfg(feature = "failures")]
    #[test]
    #[allow(unused_variables, unused_mut)]
    fn not_restrictive_enough() {
        //! This example compiles without PhantomData in Pudlo.
        //! The problem here is that drop implementation for Hia<T> accessed the T via exclusive
        //! reference.
        //! While variable `b` holds and exclusive reference to `&mut u32` until the end of the
        //! scope of this function there is shared borrow of `value` variable in `println!`
        //! statement.
        //! Mutable and immutable borrow overlaps because Hia<T> has its own Drop implementation
        //! so compiler assumes that T is accessed (&mut u32) in this case.
        //! So, exclusive and shared borrows overlap.
        use std::fmt::Debug;

        struct Hia<T: Debug>(T);

        impl<T> Drop for Hia<T>
        where
            T: Debug,
        {
            fn drop(&mut self) {
                println!("{:?}", self.0);
            }
        }

        let mut value = 99;
        let b = Pudlo::new(Hia(&mut 99));

        println!("{:?}", value);
    }
}
