
#[macro_use]
extern crate log;

extern crate casclib_sys as casclib;

use std::error;
use std::fmt;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io;
use std::mem;
use std::path::{Path};
pub use casclib::{Locale, OpenFileFlags};

#[derive(Debug)]
pub enum CascError {
    Io(io::Error),
    InvalidPath,
    InvalidFileName,
    Code(i32),
}

impl fmt::Display for CascError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CascError::Io(ref err) => err.fmt(f),
            CascError::InvalidPath => write!(f, "Invalid storage path"),
            CascError::InvalidFileName => write!(f, "Invalid file name"),
            CascError::Code(code) => write!(f, "Error code: {}", code),
        }
    }
}

impl error::Error for CascError {
    fn description(&self) -> &str {
        match *self {
            CascError::Io(ref err) => err.description(),
            CascError::InvalidPath => "Invalid storage path",
            CascError::InvalidFileName => "Invalid file name",
            CascError::Code(code) => {
                match code {
                    _ => "Unknown error code",
                }
            }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CascError::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

pub fn open<P: AsRef<Path>>(path: P) -> Result<Storage, CascError> {
    let path_str = path.as_ref().to_str().ok_or(CascError::InvalidPath)?;
    let path_cstr = CString::new(path_str).map_err(|_| CascError::InvalidPath)?;
    let handle = unsafe {
        let mut handle: casclib::Handle = 0;
        let ok = casclib::CascOpenStorage(path_cstr.as_ptr(), 0, &mut handle as *mut casclib::Handle);
        if !ok {
            return Err(CascError::Code(casclib::GetLastError()));
        }
        handle
    };

    Ok(Storage {
        handle: handle,
        file_count: Storage::read_file_count(handle)?,
    })
}

pub struct Storage {
    handle: casclib::Handle,
    file_count: u32,
}

impl Drop for Storage {
    fn drop(&mut self) {
        unsafe {
            casclib::CascCloseStorage(self.handle);
        }
    }
}

impl Storage {
    fn read_file_count(handle: casclib::Handle) -> Result<u32, CascError> {
        unsafe {
            let mut count: u32 = 0;
            let ok =
                casclib::CascGetStorageInfo(handle,
                                            casclib::CASC_STORAGE_INFO_CLASS::CascStorageFileCount,
                                            (&mut count as *mut u32) as *mut u8,
                                            mem::size_of_val(&count),
                                            0 as *mut usize);
            if !ok {
                return Err(CascError::Code(casclib::GetLastError()));
            }
            Ok(count)
        }
    }

    pub fn get_file_count(&self) -> u32 {
        self.file_count
    }

    pub fn files<'a>(&'a self) -> Find<'a> {
        Find {
            storage: self,
        }
    }

    pub fn entry<'a, T>(&'a self, name: T) -> FileEntry<'a> where T : Into<String> {
        FileEntry {
            storage: self,
            name: name.into(),
        }
    }
}

pub struct Find<'a> {
    storage: &'a Storage,
}

impl<'a> IntoIterator for Find<'a> {
    type Item = Result<FileEntry<'a>, CascError>;
    type IntoIter = FindIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FindIterator {
            find: self,
            handle: None,
            data: casclib::CASC_FIND_DATA::default(),
        }
    }
}

pub struct FindIterator<'a> {
    find: Find<'a>,
    handle: Option<casclib::Handle>,
    data: casclib::CASC_FIND_DATA,
}

impl<'a> FindIterator<'a> {
    fn find_first<'b>(&'b mut self) -> Option<Result<FileEntry<'a>, CascError>> {
        let mask = b"*\0".as_ptr() as *const i8;
        unsafe {
            let handle =
                casclib::CascFindFirstFile(self.find.storage.handle,
                                           mask,
                                           &mut self.data as *mut casclib::CASC_FIND_DATA,
                                           0 as *const i8);
            if handle == 0 {
                let code = casclib::GetLastError();
                if code == casclib::ERROR_NO_MORE_FILES {
                    return None;
                } else {
                    return Err(CascError::Code(code)).into();
                }
            }
            self.handle = Some(handle)
        }
        Some(self.get_entry())
    }

    fn get_entry<'b>(&'b self) -> Result<FileEntry<'a>, CascError> {
        unsafe {
            let s = CStr::from_ptr(&self.data.szFileName as *const c_char).to_str().map_err(|_| CascError::InvalidFileName)?;
            Ok(FileEntry::new(self.find.storage, s))
        }
    }
}

impl<'a> Iterator for FindIterator<'a> {
    type Item = Result<FileEntry<'a>, CascError>;

    fn next<'b>(&'b mut self) -> Option<Self::Item> {
        match self.handle {
            None => self.find_first(),
            Some(handle) => unsafe {
                let ok =
                    casclib::CascFindNextFile(handle,
                                              &mut self.data as *mut casclib::CASC_FIND_DATA);
                if ok {
                    Some(self.get_entry())
                } else {
                    None
                }
            },
        }
    }
}

pub struct FileEntry<'a> {
    storage: &'a Storage,    
    name: String,
}

impl<'a> FileEntry<'a> {
    fn new<'b>(storage: &'a Storage, name: &'b str) -> FileEntry<'a> {
        FileEntry {
            storage: storage,
            name: name.to_string(),
        }
    }

    pub fn open(self) -> Result<File<'a>, CascError> {
        let mut file_handle: casclib::Handle = 0;
        unsafe {
            let name = CString::new(&self.name[..]).map_err(|_| CascError::InvalidFileName)?;
            let ok = casclib::CascOpenFile(self.storage.handle, name.as_ptr(), 0, 0, &mut file_handle as *mut casclib::Handle);
            if !ok {
                return Err(CascError::Code(casclib::GetLastError()));
            }
        }

        let size: u64 = unsafe {
            let mut size_high: u32 = 0;
            let size_low: u32 = casclib::CascGetFileSize(file_handle, &mut size_high as *mut u32);
            let size: u64 = size_high as u64;
            let size = (size << 32) | (size_low as u64);
            size
        };
        
        Ok(File {
            entry: self,
            handle: file_handle,
            size: size,
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct File<'a> {
    entry: FileEntry<'a>,
    handle: casclib::Handle,
    size: u64
}

impl<'a> Drop for File<'a> {
    fn drop(&mut self) {
        unsafe {
            casclib::CascCloseFile(self.handle);
        }
    }
}

impl<'a> File<'a> {
    pub fn get_name(&self) -> &str {
        &self.entry.name
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn extract<T: io::Write>(&self, mut w: T) -> Result<usize, CascError> {
        unsafe {
            casclib::SetLastError(0);
        }

        let mut buffer: [u8; 0x1000] = [0; 0x1000];
        unsafe {
            let pos = casclib::CascSetFilePointer(self.handle, 0, 0 as *mut i32, casclib::FILE_BEGIN);
            if pos == casclib::CASC_INVALID_POS {
                return Err(CascError::Code(casclib::GetLastError()));
            }
        }

        unsafe {
            let mut bytes_write_total: usize = 0;
            let mut bytes_read: u32 = 1;
            while bytes_read != 0 {
                let ok = casclib::CascReadFile(self.handle, &mut buffer[0] as *mut u8, mem::size_of_val(&buffer) as u32, &mut bytes_read as *mut u32);
                if !ok {
                    break;
                }
                let end_pos = bytes_read as usize;
                if bytes_read != 0 {
                    w.write_all(&buffer[0..end_pos]).map_err(|e| CascError::Io(e))?;
                    bytes_write_total = bytes_write_total + end_pos;
                }
            }
            match casclib::GetLastError() {
                0 => Ok(bytes_write_total),
                code => Err(
                    CascError::Io(
                        io::Error::new(
                            io::ErrorKind::Other, format!("CascReadFile error: code = {}, bytes_write_total = {}", code, bytes_write_total)
                        )
                    )
                ),
            }
        }
    }
}