#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;
#[macro_use]
extern crate bitflags;

use libc::{c_char, c_long, size_t, uintptr_t, SEEK_CUR, SEEK_END, SEEK_SET};

bitflags! {
    pub struct Locale: u32 {
        const ALL = 0xFFFFFFFF;
        const NONE = 0x00000000;
        const UNKNOWN1 = 0x00000001;
        const ENUS = 0x00000002;
        const KOKR = 0x00000004;
        const RESERVED = 0x00000008;
        const FRFR = 0x00000010;
        const DEDE = 0x00000020;
        const ZHCN = 0x00000040;
        const ESES = 0x00000080;
        const ZHTW = 0x00000100;
        const ENGB = 0x00000200;
        const ENCN = 0x00000400;
        const ENTW = 0x00000800;
        const ESMX = 0x00001000;
        const RURU = 0x00002000;
        const PTBR = 0x00004000;
        const ITIT = 0x00008000;
        const PTPT = 0x00010000;
    }
}

bitflags! {
    /// Flags for `File::open`
    pub struct OpenFileFlags: u32 {
        /// The name is just the encoding key; skip ROOT file processing
        const OPEN_BY_ENCODING_KEY = 0x00000001;
    }
}

const MD5_HASH_SIZE: usize = 0x10;
const MAX_PATH: usize = 260;

pub const FILE_BEGIN: i32 = SEEK_SET;
pub const FILE_CURRENT: i32 = SEEK_CUR;
pub const FILE_END: i32 = SEEK_END;

// Return value for CascGetFileSize and CascSetFilePointer
pub const CASC_INVALID_SIZE: u32 = 0xFFFFFFFF;
pub const CASC_INVALID_POS: u32 = 0xFFFFFFFF;

#[repr(C)]
pub enum CASC_STORAGE_INFO_CLASS {
    CascStorageFileCount,
    CascStorageFeatures,
    CascStorageGameInfo,
    CascStorageGameBuild,
    CascStorageInfoClassMax4,
}

#[repr(C)]
pub struct QUERY_KEY {
    pbData: *const u8,
    cbData: u32,
}

// Structure for SFileFindFirstFile and SFileFindNextFile
#[repr(C)]
pub struct CASC_FIND_DATA {
    // Full name of the found file
    pub szFileName: [c_char; MAX_PATH],
    // Plain name of the found file
    pub szPlainName: *const c_char,
    // Encoding key
    pub EncodingKey: [u8; MD5_HASH_SIZE],
    // Locale flags (WoW only)
    pub dwLocaleFlags: u32,
    // Size of the file
    pub dwFileSize: u32,
}

impl Default for CASC_FIND_DATA {
    fn default() -> Self {
        CASC_FIND_DATA {
            szFileName: [0; MAX_PATH],
            szPlainName: 0 as *const c_char,
            EncodingKey: [0; MD5_HASH_SIZE],
            dwLocaleFlags: 0,
            dwFileSize: 0,
        }
    }
}

pub const ERROR_INVALID_HANDLE: i32 = 6;
pub const ERROR_NO_MORE_FILES: i32 = 18;
pub const ERROR_INVALID_PARAMETER: i32 = 87;
pub const ERROR_INSUFFICIENT_BUFFER: i32 = 122;

pub type Handle = uintptr_t;

extern "stdcall" {
    pub fn CascOpenStorage(
        szDataPath: *const c_char,
        dwLocaleMask: u32,
        phStorage: *mut uintptr_t,
    ) -> bool;
    pub fn CascGetStorageInfo(
        hStorage: uintptr_t,
        InfoClass: CASC_STORAGE_INFO_CLASS,
        pvStorageInfo: *mut u8,
        cbStorageInfo: size_t,
        pcbLengthNeeded: *mut size_t,
    ) -> bool;
    pub fn CascCloseStorage(hStorage: uintptr_t) -> bool;

    pub fn CascOpenFileByIndexKey(
        hStorage: uintptr_t,
        pIndexKey: *const QUERY_KEY,
        dwFlags: u32,
        phFile: *mut uintptr_t,
    ) -> bool;
    pub fn CascOpenFileByEncodingKey(
        hStorage: uintptr_t,
        pEncodingKey: *const QUERY_KEY,
        dwFlags: u32,
        phFile: *mut uintptr_t,
    ) -> bool;
    pub fn CascOpenFile(
        hStorage: uintptr_t,
        szFileName: *const c_char,
        dwLocale: u32,
        dwFlags: u32,
        phFile: *mut uintptr_t,
    ) -> bool;
    pub fn CascGetFileSize(hFile: uintptr_t, pdwFileSizeHigh: *mut u32) -> u32;
    pub fn CascSetFilePointer(
        hFile: uintptr_t,
        lFilePos: c_long,
        plFilePosHigh: *mut c_long,
        dwMoveMethod: i32,
    ) -> u32;
    pub fn CascReadFile(
        hFile: uintptr_t,
        lpBuffer: *mut u8,
        dwToRead: u32,
        pdwRead: *mut u32,
    ) -> bool;
    pub fn CascCloseFile(hFile: uintptr_t) -> bool;

    pub fn CascFindFirstFile(
        hStorage: uintptr_t,
        szMask: *const c_char,
        pFindData: *mut CASC_FIND_DATA,
        szListFile: *const c_char,
    ) -> uintptr_t;
    pub fn CascFindNextFile(hFind: uintptr_t, pFindData: *mut CASC_FIND_DATA) -> bool;
    pub fn CascFindClose(hFind: uintptr_t) -> bool;
    pub fn GetLastError() -> i32;
    pub fn SetLastError(code: i32);
}
