use libc::c_char;
use std::{
    ffi::CString,
    ptr::{null, null_mut},
    slice,
};

pub use fdo_data_formats::ownershipvoucher::OwnershipVoucher;
use fdo_data_formats::Serializable;

#[no_mangle]
/// Creates a new OwnershipVoucher from raw data
///
/// Return value:
/// NULL on error
/// Pointer to an FdoOwnershipVoucher on success
pub extern "C" fn fdo_ownershipvoucher_from_data(
    data: *const std::ffi::c_void,
    len: libc::size_t,
) -> *mut OwnershipVoucher {
    if data.is_null() {
        return null_mut();
    }
    let data = unsafe { slice::from_raw_parts(data as *const u8, len) };
    match OwnershipVoucher::deserialize_data(data) {
        Ok(voucher) => Box::into_raw(Box::new(voucher)),
        Err(_) => null_mut(),
    }
}

#[no_mangle]
/// Frees an OwnershipVoucher
pub extern "C" fn fdo_ownershipvoucher_free(v: *mut OwnershipVoucher) {
    if v.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(v));
    }
}

#[no_mangle]
/// Returns the protocol version in the ownership voucher
///
/// Return value:
/// -1 on error
/// protocol version on success
pub extern "C" fn fdo_ownershipvoucher_header_get_protocol_version(
    v: *const OwnershipVoucher,
) -> i32 {
    if v.is_null() {
        return -1;
    }
    let voucher = unsafe { &*v };
    voucher.header().protocol_version() as i32
}

#[no_mangle]
/// Returns the GUID of the ownership voucher
///
/// Return value:
/// NULL on error
/// Pointer to a string containing the GUID on success
///
/// Note: The returned string ownership is transferred to the caller, and should
/// be freed with `fdo_free_string`
pub extern "C" fn fdo_ownershipvoucher_header_get_guid(
    v: *const OwnershipVoucher,
) -> *const c_char {
    if v.is_null() {
        return null();
    }
    let voucher = unsafe { &*v };
    let guid = voucher.header().guid().to_string();
    match CString::new(guid) {
        Err(_) => null(),
        Ok(cstr) => cstr.into_raw(),
    }
}

#[no_mangle]
/// Returns the device info of the ownership voucher if it is a string
///
/// Return value:
/// NULL on error or if Device Info is not a string
/// Pointer to a string containing the Device Info on success
///
/// Note: The returned string ownership is transferred to the caller, and should
/// be freed with `fdo_free_string`
pub extern "C" fn fdo_ownershipvoucher_header_get_device_info_string(
    v: *const OwnershipVoucher,
) -> *const c_char {
    if v.is_null() {
        return null();
    }
    let voucher = unsafe { &*v };
    match CString::new(voucher.header().device_info()) {
        Err(_) => null(),
        Ok(cstr) => cstr.into_raw(),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_common as TC;
    use crate::test_common::OutputExt;

    #[test]
    fn test_ownershipvoucher_parsing() {
        let ov_path = TC::test_asset_path("testdevice1.ov");

        let result = TC::run_external("ownershipvoucher", &[ov_path.to_str().unwrap()]);

        assert!(result.status.success());
        result.stdout_equals(
            "Protocol version: 100
Device GUID: 214d64be-3227-92da-0333-b1e1fe832f24
Device Info: testdevice1",
        );
        result.stderr_equals("");
    }
}
