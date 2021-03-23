use core::convert::TryFrom;

use crate::error::Error;

use super::{ffi, gc::Gc, map::Map, obj::Obj};

pub type Dict = ffi::mp_obj_dict_t;

impl Dict {
    pub fn alloc_with_capacity(capacity: usize) -> Gc<Self> {
        unsafe {
            let ptr = ffi::mp_obj_new_dict(capacity);
            Gc::from_raw(ptr.as_ptr().cast())
        }
    }

    pub fn with_map(map: Map) -> Self {
        unsafe {
            Self {
                base: ffi::mp_obj_base_t {
                    type_: &ffi::mp_type_dict,
                },
                map,
            }
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn map_mut(&mut self) -> &mut Map {
        &mut self.map
    }
}

impl From<Gc<Dict>> for Obj {
    fn from(value: Gc<Dict>) -> Self {
        // SAFETY:
        //  - `value` is an object struct with a base and a type.
        //  - `value` is GC-allocated.
        unsafe { Obj::from_ptr(Gc::into_raw(value).cast()) }
    }
}

impl TryFrom<Obj> for Gc<Dict> {
    type Error = Error;

    fn try_from(value: Obj) -> Result<Self, Self::Error> {
        if unsafe { ffi::mp_type_dict.is_type_of(value) } {
            // SAFETY: We assume that if `value` is an object pointer with the correct type,
            // it is always GC-allocated.
            let this = unsafe { Gc::from_raw(value.as_ptr().cast()) };
            Ok(this)
        } else {
            Err(Error::InvalidType)
        }
    }
}

// SAFETY: We are in a single-threaded environment.
unsafe impl Sync for Dict {}
