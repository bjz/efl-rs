// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate ffi;

pub use self::ffi::*;

use libc;
use std::ptr;

pub static EINA_FALSE: Eina_Bool = 0;
pub static EINA_TRUE: Eina_Bool = 1;

/// Convert from a Rust boolean to an `Eina_Bool`.
pub fn to_eina_bool(x: bool) -> Eina_Bool {
    if x { EINA_TRUE } else { EINA_FALSE }
}

/// Convert an `Eina_Bool` to a Rust boolean.
pub fn from_eina_bool(x: Eina_Bool) -> bool {
    if x == EINA_FALSE { false } else { true }
}

pub fn eina_list_iter(list: *const Eina_List) -> EinaListItems {
    EinaListItems {
        iter: unsafe { eina_list_iterator_new(list) },
    }
}

pub struct EinaListItems {
    iter: *mut Eina_Iterator,
}

impl Iterator<*mut libc::c_void> for EinaListItems {
    fn next(&mut self) -> Option<*mut libc::c_void> {
        let mut data = ptr::mut_null();
        match unsafe { eina_iterator_next(self.iter, &mut data) } {
            EINA_TRUE => Some(data),
            _ => None,
        }
    }
}

impl Drop for EinaListItems {
    fn drop(&mut self) {
        unsafe {
            eina_iterator_free(self.iter);
        }
    }
}
