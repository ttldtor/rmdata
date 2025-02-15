// Copyright (c) 2025 ttldtor.
// SPDX-License-Identifier: BSL-1.0

mod ffi;

use std::{ptr, ffi::c_int};
use crate::ffi::graal_create_isolate;
use crate::ffi::graal_create_isolate_params_t;
use crate::ffi::graal_isolate_t;
use crate::ffi::graal_isolatethread_t;

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
    params: *mut graal_create_isolate_params_t, // Changed from `&mut` to raw pointer for flexibility
) -> Result<(Isolate, IsolateThread), c_int> {
    if params.is_null() {
        return Err(1); // Arbitrary error code when null is passed
    }

    // Initialize raw pointers for isolate and thread
    let mut isolate_ptr: *mut graal_isolate_t = ptr::null_mut();
    let mut thread_ptr: *mut graal_isolatethread_t = ptr::null_mut();

    // Call the unsafe external function
    let result = unsafe {
        graal_create_isolate(
            params,
            &mut isolate_ptr as *mut *mut graal_isolate_t,
            &mut thread_ptr as *mut *mut graal_isolatethread_t,
        )
    };

    // Handle success and error cases
    if result == 0 {
        Ok((
            Isolate {
                ptr: isolate_ptr,
            },
            IsolateThread {
                ptr: thread_ptr,
            },
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


pub fn add(left: u64, right: u64) -> u64 {
    left + right
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