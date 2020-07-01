extern crate casclib_sys;
extern crate libc;

#[test]
fn test_wc3() {
    use std::ffi::CString;
    use std::mem;
    use std::ptr;
    let path = CString::new(r#"C:\Program Files (x86)\Warcraft III\Data"#).unwrap();
    let mut handle: casclib_sys::HANDLE = ptr::null_mut();

    // CascOpenStorage
    unsafe {
        let result =
            casclib_sys::CascOpenStorage(path.as_ptr(), 0, &mut handle as *mut casclib_sys::HANDLE);
        assert_eq!(result, true);
    }

    let mut info: u32 = 0;

    let result = unsafe {
        casclib_sys::CascGetStorageInfo(
            handle,
            casclib_sys::_CASC_STORAGE_INFO_CLASS_CascStorageTotalFileCount,
            mem::transmute(&mut info as *mut u32),
            4,
            ptr::null_mut(),
        )
    };

    assert!(result);
    assert!(info > 0);

    let result = unsafe {
        casclib_sys::CascGetStorageInfo(
            handle,
            casclib_sys::_CASC_STORAGE_INFO_CLASS_CascStorageFeatures,
            mem::transmute(&mut info as *mut u32),
            4,
            ptr::null_mut(),
        )
    };

    assert!(result);
}
