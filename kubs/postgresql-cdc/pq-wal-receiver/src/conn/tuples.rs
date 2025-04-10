use crate::bindings;

#[derive(Debug)]
pub struct Tuples<'a> {
    __lifetime: std::marker::PhantomData<&'a ()>,
    result: *mut bindings::PGresult,
    pub num_attrs: std::ffi::c_int,
    pub num_tuples: std::ffi::c_int,
    pub status: bindings::ExecStatusType,
}

impl Tuples<'_> {
    pub unsafe fn new(status: bindings::ExecStatusType, pg_result: *mut bindings::PGresult) -> Self {
        unsafe {
            Self {
                __lifetime: std::marker::PhantomData,
                num_attrs: bindings::PQnfields(pg_result),
                num_tuples: bindings::PQntuples(pg_result),
                result: pg_result,
                status,
            }
        }
    }

    pub fn get_attr_idx(&self, attr_name: &std::ffi::CStr) -> std::ffi::c_int {
        assert!(self.num_attrs > 0);

        for i in 0..self.num_attrs {
            let name = unsafe {
                std::ffi::CStr::from_ptr(bindings::PQfname(self.result, i as i32))
            };

            if attr_name == name {
                return i;
            }
        }

        unreachable!("Get good - invalid name.")
    }

    pub fn get_attr_names(&self) -> Vec<String> {
        assert!(self.num_attrs > 0);
        assert!(self.num_attrs <= std::i32::MAX);

        let mut attrs = Vec::with_capacity(self.num_attrs as usize /* SAFETY: assertions above */);
        
        for i in 0..attrs.capacity() {
            unsafe {
                let attr_name = std::ffi::CStr::from_ptr(
                    bindings::PQfname(self.result, i as i32 /* SAFETY: assertions above */)
                );
                // TODO: Are PG column names utf-8 encoded?
                attrs.push(String::from_utf8(attr_name.to_bytes().to_owned()).unwrap());
            }
        }

        attrs
    }

    pub fn get_attr_name(&self, attr_idx: i32) -> &std::ffi::CStr {
        assert!(attr_idx >= 0);

        unsafe {
            std::ffi::CStr::from_ptr(bindings::PQfname(self.result, attr_idx))
        }
    }

    pub fn get_attr_value(&self, tuple_idx: i32, attr_idx: i32) -> &std::ffi::CStr {
        assert!(attr_idx >= 0);
        assert!(tuple_idx >= 0);

        unsafe {
            std::ffi::CStr::from_ptr(bindings::PQgetvalue(self.result, tuple_idx, attr_idx))
        }
    }

    pub fn iter<'a, 's: 'a>(&'s self) -> TuplesIter<'a, 's> {
        TuplesIter::new(self)
    }
}

impl Drop for Tuples<'_> {
    fn drop(&mut self) {
        unsafe {
            bindings::PQclear(self.result);
        }
    }
}

pub struct Tuple<'a, 't: 'a> {
    tuples: &'a Tuples<'t>,
    idx: std::ffi::c_int,
}

impl<'a, 't: 'a> Tuple<'a, 't> {
    pub fn get_attr_idx(&self, attr_name: &std::ffi::CStr) -> std::ffi::c_int {
        self.tuples.get_attr_idx(attr_name)
    }

    pub fn get_attr_name(&self, attr_idx: i32) -> &std::ffi::CStr {
        self.tuples.get_attr_name(attr_idx)
    }

    pub fn get_attr_names(&self) -> Vec<String> {
        self.tuples.get_attr_names()
    }

    pub fn get_attr_value(&self, attr_idx: i32) -> &std::ffi::CStr {
        self.tuples.get_attr_value(self.idx, attr_idx)
    }
}

impl<'a, 't> std::fmt::Debug for Tuple<'a, 't> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut s = f.debug_struct("Tuplehno");

        for i in 0..self.tuples.num_attrs {
            let attr_name = unsafe {
                std::str::from_utf8_unchecked(self.get_attr_name(i).to_bytes())
            };
            s.field(attr_name, &self.get_attr_value(i));
        }

        s.finish()
    }
}

pub struct TuplesIter<'a, 't> {
    tuples: &'a Tuples<'t>,
    next: std::ffi::c_int,
}

impl<'a, 't> TuplesIter<'a, 't> {
    fn new(tuples: &'a Tuples<'t>) -> Self {
        Self {
            tuples,
            next: 0,
        }
    }
}

impl<'a, 't> Iterator for TuplesIter<'a, 't> {
    type Item = Tuple<'a, 't>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tuples.num_tuples <= self.next {
            return None;
        }
        
        let item = Tuple {
            tuples: self.tuples,
            idx: self.next,
        };

        self.next += 1;

        Some(item)
    }
}
