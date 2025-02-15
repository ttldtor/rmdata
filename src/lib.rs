// Copyright (c) 2025 ttldtor.
// SPDX-License-Identifier: BSL-1.0

mod ffi;

use crate::ffi::graal_create_isolate;
use crate::ffi::graal_create_isolate_params_t;
use crate::ffi::graal_isolate_t;
use crate::ffi::graal_isolatethread_t;
use std::{ffi::c_int, ptr};

#[derive(Debug)]
pub struct Isolate {
    #[allow(dead_code)]
    ptr: *mut graal_isolate_t,
}

#[derive(Debug)]
pub struct IsolateThread {
    #[allow(dead_code)]
    ptr: *mut graal_isolatethread_t,
}

/// Safe wrapper function for `graal_create_isolate`.
pub fn safe_graal_create_isolate(
    params: *mut graal_create_isolate_params_t,
) -> Result<(Isolate, IsolateThread), c_int> {
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
        Ok((
            Isolate { ptr: isolate_ptr },
            IsolateThread { ptr: thread_ptr },
        ))
    } else {
        Err(result)
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
    fn test_safe_graal_create_isolate() {
        // Call the safe wrapper.
        let result = safe_graal_create_isolate(ptr::null_mut());

        // Check the result, expect an error.
        assert!(result.is_ok());
    }
}
