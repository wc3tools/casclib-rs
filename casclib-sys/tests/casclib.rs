extern crate casclib_sys;

extern crate libc;
use libc::{uintptr_t, size_t, c_char};

#[test]
fn test_bindings() {
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::mem;
    let path = CString::new(r#"C:\Program Files (x86)\StarCraft II\SC2Data"#).unwrap();
    let mut handle: uintptr_t = 0;

    // CascOpenStorage
    unsafe {
        let result = casclib_sys::CascOpenStorage(path.as_ptr(), 0, &mut handle as *mut uintptr_t);
        assert_eq!(result, true);
    }

    // CascGetStorageInfo
    unsafe {
        let mut info: u32 = 0;
        let mut needed: size_t = 0;

        let result = casclib_sys::CascGetStorageInfo(
            handle, 
            casclib_sys::CASC_STORAGE_INFO_CLASS::CascStorageFileCount,
            0 as *mut u8,
            1,
            &mut needed as *mut size_t
        );

        assert_eq!(result, false);
        assert_eq!(info, 0);
        assert_eq!(needed, 4);

        let err = casclib_sys::GetLastError();
        assert_eq!(err, casclib_sys::ERROR_INSUFFICIENT_BUFFER);

        info = 0;
        needed = 0;

        let result = casclib_sys::CascGetStorageInfo(
            handle, 
            casclib_sys::CASC_STORAGE_INFO_CLASS::CascStorageFileCount,
            (&mut info as *mut u32) as *mut u8,
            mem::size_of_val(&info),
            &mut needed as *mut size_t
        );

        assert_eq!(result, true);
        assert!(info > 0);
        assert_eq!(needed, 0);
    }

    // CascFindFirstFile
    unsafe {
        let mut find_data = casclib_sys::CASC_FIND_DATA::default();
        let file_handle: uintptr_t =
            casclib_sys::CascFindFirstFile(handle,
                                           ("*\0".as_ptr()) as *const c_char,
                                           &mut find_data as *mut casclib_sys::CASC_FIND_DATA,
                                           0 as *const c_char);
        assert!(file_handle != 0);

        let filename = CStr::from_ptr(&find_data.szFileName as *const c_char).to_str().unwrap();
        println!("file name: {}", filename);

        let result = casclib_sys::CascFindClose(file_handle);
        assert_eq!(result, true);
    }

    // CascCloseStorage
    unsafe {
        let result = casclib_sys::CascCloseStorage(handle);
        assert_eq!(result, true);
    }
}