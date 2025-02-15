// Copyright (c) 2025 ttldtor.
// SPDX-License-Identifier: BSL-1.0

use std::os::raw::{c_char, c_int, c_ulonglong};

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct __graal_isolate_t;
#[allow(non_camel_case_types)]
pub type graal_isolate_t = __graal_isolate_t;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct __graal_isolatethread_t;
#[allow(non_camel_case_types)]
pub type graal_isolatethread_t = __graal_isolatethread_t;

// Constants
pub const __GRAAL_CREATE_ISOLATE_PARAMS_VERSION: i32 = 4;

// Struct definition
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct __graal_create_isolate_params_t {
    pub version: c_int, // Version of this struct

    // Fields from version 1
    pub reserved_address_space_size: c_ulonglong, // Size of address space to reserve

    // Fields from version 2
    pub auxiliary_image_path: *const c_char, // Path to an auxiliary image to load
    pub auxiliary_image_reserved_space_size: c_ulonglong, // Reserved bytes for loading an auxiliary image

    // Fields from version 3
    pub _reserved_1: c_int,            // Internal usage, do not use
    pub _reserved_2: *mut *mut c_char, // Internal usage, do not use
    pub pkey: c_int,                   // Isolate protection key or domain

    // Fields from version 4
    pub _reserved_3: c_char, // Internal usage, do not use
    pub _reserved_4: c_char, // Internal usage, do not use
}
#[allow(non_camel_case_types)]
pub type graal_create_isolate_params_t = __graal_create_isolate_params_t;

// Function declarations
#[allow(dead_code)]
extern "C" {
    /// Creates an isolate.
    pub fn graal_create_isolate(
        params: *mut graal_create_isolate_params_t,
        isolate: *mut *mut graal_isolate_t,
        thread: *mut *mut graal_isolatethread_t,
    ) -> c_int;

    /// Attaches the current thread to the passed isolate.
    /// Returns 0 on success or a non-zero value on failure.
    pub fn graal_attach_thread(
        isolate: *mut graal_isolate_t,
        thread: *mut *mut graal_isolatethread_t,
    ) -> c_int;

    /// Gets the currently attached thread for the given isolate.
    /// Returns a pointer to the current `graal_isolatethread_t` or null on failure.
    pub fn graal_get_current_thread(isolate: *mut graal_isolate_t) -> *mut graal_isolatethread_t;

    /// Gets the associated isolate for the given isolate thread.
    /// Returns a pointer to the `graal_isolate_t` or null on failure.
    pub fn graal_get_isolate(thread: *mut graal_isolatethread_t) -> *mut graal_isolate_t;

    /// Detaches the passed isolate thread from its isolate.
    /// Returns 0 on success, or a non-zero value on failure.
    pub fn graal_detach_thread(thread: *mut graal_isolatethread_t) -> c_int;

    /// Tears down the isolate of the passed (and still attached) isolate thread.
    /// Returns 0 on success, or a non-zero value on failure.
    pub fn graal_tear_down_isolate(isolate_thread: *mut graal_isolatethread_t) -> c_int;

    /// Detaches all externally started threads and tears down the isolate.
    /// Returns 0 on success, or a non-zero value on (non-fatal) failure.
    pub fn graal_detach_all_threads_and_tear_down_isolate(
        isolate_thread: *mut graal_isolatethread_t,
    ) -> c_int;
}
