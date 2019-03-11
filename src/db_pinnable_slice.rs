// Copyright 2019 Tyler Neely
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use libc::size_t;

use ffi;
use std::marker::PhantomData;
use std::ops::Deref;
use std::slice;

use crate::DB;

/// Wrapper around RocksDB PinnableSlice struct.
///
/// With a pinnable slice, we can directly leverage in-memory data within
/// RocksDB toa void unnecessary memory copies. The struct here wraps the
/// returned raw pointer and ensures proper finalization work.
pub struct DBPinnableSlice<'a> {
    ptr: *mut ffi::rocksdb_pinnableslice_t,
    db: PhantomData<&'a DB>,
}

impl<'a> AsRef<[u8]> for DBPinnableSlice<'a> {
    fn as_ref(&self) -> &[u8] {
        // Implement this via Deref so as not to repeat ourselves
        &*self
    }
}

impl<'a> Deref for DBPinnableSlice<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe {
            let mut val_len: size_t = 0;
            let val = ffi::rocksdb_pinnableslice_value(self.ptr, &mut val_len) as *mut u8;
            slice::from_raw_parts(val, val_len)
        }
    }
}

impl<'a> Drop for DBPinnableSlice<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::rocksdb_pinnableslice_destroy(self.ptr);
        }
    }
}

impl<'a> DBPinnableSlice<'a> {
    /// Used to wrap a PinnableSlice from rocksdb to avoid unnecessary memcpy
    ///
    /// # Unsafe
    /// Requires that the pointer must be generated by rocksdb_get_pinned
    pub unsafe fn from_c(ptr: *mut ffi::rocksdb_pinnableslice_t) -> DBPinnableSlice<'a> {
        DBPinnableSlice {
            ptr,
            db: PhantomData,
        }
    }
}
