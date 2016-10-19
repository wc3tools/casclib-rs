#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;
use libc::{c_char, c_long, size_t, uintptr_t};

pub mod defines;

const MAX_PATH: usize = 260;

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
    pub EncodingKey: [u8; defines::MD5_HASH_SIZE],
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
            EncodingKey: [0; defines::MD5_HASH_SIZE],
            dwLocaleFlags: 0,
            dwFileSize: 0,
        }
    }
}

pub const ERROR_INVALID_HANDLE: u32 = 6;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

extern "stdcall" {
    pub fn CascOpenStorage(szDataPath: *const c_char,
                           dwLocaleMask: u32,
                           phStorage: *mut uintptr_t)
                           -> bool;
    pub fn CascGetStorageInfo(hStorage: uintptr_t,
                              InfoClass: CASC_STORAGE_INFO_CLASS,
                              pvStorageInfo: *mut u8,
                              cbStorageInfo: size_t,
                              pcbLengthNeeded: *mut size_t)
                              -> bool;
    pub fn CascCloseStorage(hStorage: uintptr_t) -> bool;


    pub fn CascOpenFileByIndexKey(hStorage: uintptr_t,
                                  pIndexKey: *const QUERY_KEY,
                                  dwFlags: u32,
                                  phFile: *mut uintptr_t)
                                  -> bool;
    pub fn CascOpenFileByEncodingKey(hStorage: uintptr_t,
                                     pEncodingKey: *const QUERY_KEY,
                                     dwFlags: u32,
                                     phFile: *mut uintptr_t)
                                     -> bool;
    pub fn CascOpenFile(hStorage: uintptr_t,
                        szFileName: *const c_char,
                        dwLocale: u32,
                        dwFlags: u32,
                        phFile: *mut uintptr_t)
                        -> bool;
    pub fn CascGetFileSize(hFile: uintptr_t, pdwFileSizeHigh: *mut u32) -> u32;
    pub fn CascSetFilePointer(hFile: uintptr_t,
                              lFilePos: c_long,
                              plFilePosHigh: *mut c_long,
                              dwMoveMethod: u32)
                              -> u32;
    pub fn CascReadFile(hFile: uintptr_t,
                        lpBuffer: *mut u8,
                        dwToRead: u32,
                        pdwRead: *mut u32)
                        -> bool;
    pub fn CascCloseFile(hFile: uintptr_t) -> bool;


    pub fn CascFindFirstFile(hStorage: uintptr_t,
                             szMask: *const c_char,
                             pFindData: *mut CASC_FIND_DATA,
                             szListFile: *const c_char)
                             -> uintptr_t;
    pub fn CascFindNextFile(hFind: uintptr_t, pFindData: *mut CASC_FIND_DATA) -> bool;
    pub fn CascFindClose(hFind: uintptr_t) -> bool;
    pub fn GetLastError() -> u32;
}