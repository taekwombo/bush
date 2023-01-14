//! https://www.youtube.com/watch?v=8O0Nt9qY_vo

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[derive(Clone, Copy)]
enum Borrow {
    Exclusive,
    Shared(usize),
    None,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    flag: Cell<Borrow>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            flag: Cell::new(Borrow::None),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.flag.get() {
            Borrow::Exclusive => None,
            Borrow::None => {
                self.flag.set(Borrow::Shared(1));
                Some(Ref { rc: self })
            }
            Borrow::Shared(_) => Some(Ref { rc: self }),
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.flag.get() {
            Borrow::None => {
                self.flag.set(Borrow::Exclusive);
                Some(RefMut { rc: self })
            }
            _ => None,
        }
    }
}

pub struct Ref<'a, T> {
    rc: &'a RefCell<T>,
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        match self.rc.flag.get() {
            Borrow::Shared(1) => {
                self.rc.flag.set(Borrow::None);
            }
            Borrow::Shared(n) => {
                self.rc.flag.set(Borrow::Shared(n - 1));
            }
            _ => unreachable!(),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rc.value.get() }
    }
}

pub struct RefMut<'a, T> {
    rc: &'a RefCell<T>,
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        match self.rc.flag.get() {
            Borrow::Exclusive => {
                self.rc.flag.set(Borrow::None);
            }
            _ => unreachable!(),
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.rc.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.rc.value.get() }
    }
}

struct RcInner<T> {
    value: T,
    references: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::into_raw(Box::new(RcInner {
            value,
            references: Cell::new(1),
        }));

        Self {
            inner: unsafe { NonNull::new_unchecked(inner) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.references.set(inner.references.get() + 1);

        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &self.inner.as_ref().value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let count = inner.references.get();

        if count == 1 {
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.references.set(count - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell() {
        let c = Cell::new(100_u8);
        assert_eq!(100, c.get());
        c.set(99);
        assert_eq!(99, c.get());
    }

    #[test]
    fn ref_cell() {
        let rc = RefCell::new(100_u8);

        if let Some(mut val) = rc.borrow_mut() {
            *val = 99;
        }

        assert_eq!(Some(99), rc.borrow().map(|v| *v));
    }

    #[test]
    fn ref_cell_none() {
        let rc = RefCell::new(100_u8);
        let _mb = rc.borrow_mut();

        assert!(rc.borrow().is_none());
    }

    #[test]
    fn rc() {
        let rc = Rc::new(9);

        assert_eq!(unsafe { rc.inner.as_ref() }.references.get(), 1);

        let v = vec![rc.clone(), rc.clone()];

        assert_eq!(unsafe { rc.inner.as_ref() }.references.get(), 3);

        for i in v {
            drop(i);
        }

        assert_eq!(unsafe { rc.inner.as_ref() }.references.get(), 1);
    }
}
