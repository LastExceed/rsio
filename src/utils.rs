use std::ffi::*;
use windows_core::{GUID, Interface};
use crate::windows_bindings::*;

/// Same as [`Interface::cast`], except that the target interface's IID is decoupled from its type.
pub(crate) unsafe fn cast_decoupled<Target: Interface>(interface: &impl Interface, target_iid: *const GUID) -> windows_core::Result<Target> {
    let mut out = None;
    unsafe { interface.query(target_iid, (&raw mut out).cast()) }.ok()?;
    out.ok_or_else(|| E_POINTER.into())
}

#[must_use]
pub(crate) fn convert_cstring(buffer: &[u8]) -> String {
    CStr
    ::from_bytes_until_nul(buffer)
    .expect("buffer overflow")
    .to_string_lossy()
    .into_owned()
}