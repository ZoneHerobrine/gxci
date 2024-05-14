#![allow(dead_code)]


use std::ffi::{c_void};

// Define GX_DEV_HANDLE as a pointer to a c_void to represent it as an opaque type.
pub type GX_DEV_HANDLE = *mut c_void;
pub type GX_EVENT_CALLBACK_HANDLE = *mut c_void;
pub type GX_FEATURE_CALLBACK_HANDLE = *mut c_void;