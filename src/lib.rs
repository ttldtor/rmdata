// Copyright (c) 2025 ttldtor.
// SPDX-License-Identifier: BSL-1.0

mod ffi;

use crate::ffi::graal_create_isolate;
use crate::ffi::graal_create_isolate_params_t;
use crate::ffi::graal_isolate_t;
use crate::ffi::graal_isolatethread_t;
use std::{ffi::c_int, ptr};

#[derive(Debug)]
pub struct IsolateThread {
    #[allow(dead_code)]
    ptr: *mut graal_isolatethread_t,
}

#[derive(Debug)]
pub struct Isolate {
    #[allow(dead_code)]
    ptr: *mut graal_isolate_t,
    #[allow(dead_code)]
    thread: IsolateThread,
}

impl Isolate {
    /// Constructs a new `Isolate` along with its `IsolateThread`.
    pub fn new(params: *mut graal_create_isolate_params_t) -> Result<Self, c_int> {
        let mut isolate_ptr: *mut graal_isolate_t = ptr::null_mut();
        let mut thread_ptr: *mut graal_isolatethread_t = ptr::null_mut();

        let result = unsafe {
            graal_create_isolate(
                params,
                &mut isolate_ptr as *mut *mut graal_isolate_t,
                &mut thread_ptr as *mut *mut graal_isolatethread_t,
            )
        };

        if result == 0 {
            Ok(Isolate {
                ptr: isolate_ptr,
                thread: IsolateThread { ptr: thread_ptr },
            })
        } else {
            Err(result)
        }
    }

    pub fn new_default() -> Result<Self, c_int> {
        Self::new(ptr::null_mut())
    }
}


// Implement Drop for automatic cleanup if needed.
impl Drop for Isolate {
    fn drop(&mut self) {
        // Clean-up logic if required, e.g., tearing down the isolate.
    }
}

impl Drop for IsolateThread {
    fn drop(&mut self) {
        // Clean-up logic if required, e.g., detaching the thread.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolate_new() {
        // Create a new `Isolate` instance.
        let result = Isolate::new(ptr::null_mut());

        // Check the result, expect an error.
        assert!(result.is_ok());
    }

    #[test]
    fn test_isolate_new_default() {
        // Create a new `Isolate` instance.
        let result = Isolate::new_default();

        // Check the result, expect an error.
        assert!(result.is_ok());
    }
}
