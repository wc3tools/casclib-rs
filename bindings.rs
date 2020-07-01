/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const CASCLIB_VERSION: u32 = 512;
pub const CASCLIB_VERSION_STRING: &'static [u8; 4usize] = b"2.0\0";
pub const CASC_OPEN_BY_NAME: u32 = 0;
pub const CASC_OPEN_BY_CKEY: u32 = 1;
pub const CASC_OPEN_BY_EKEY: u32 = 2;
pub const CASC_OPEN_BY_FILEID: u32 = 3;
pub const CASC_OPEN_TYPE_MASK: u32 = 15;
pub const CASC_OPEN_FLAGS_MASK: u32 = 4294967280;
pub const CASC_STRICT_DATA_CHECK: u32 = 16;
pub const CASC_OVERCOME_ENCRYPTED: u32 = 32;
pub const CASC_LOCALE_ALL: u32 = 4294967295;
pub const CASC_LOCALE_ALL_WOW: u32 = 127990;
pub const CASC_LOCALE_NONE: u32 = 0;
pub const CASC_LOCALE_UNKNOWN1: u32 = 1;
pub const CASC_LOCALE_ENUS: u32 = 2;
pub const CASC_LOCALE_KOKR: u32 = 4;
pub const CASC_LOCALE_RESERVED: u32 = 8;
pub const CASC_LOCALE_FRFR: u32 = 16;
pub const CASC_LOCALE_DEDE: u32 = 32;
pub const CASC_LOCALE_ZHCN: u32 = 64;
pub const CASC_LOCALE_ESES: u32 = 128;
pub const CASC_LOCALE_ZHTW: u32 = 256;
pub const CASC_LOCALE_ENGB: u32 = 512;
pub const CASC_LOCALE_ENCN: u32 = 1024;
pub const CASC_LOCALE_ENTW: u32 = 2048;
pub const CASC_LOCALE_ESMX: u32 = 4096;
pub const CASC_LOCALE_RURU: u32 = 8192;
pub const CASC_LOCALE_PTBR: u32 = 16384;
pub const CASC_LOCALE_ITIT: u32 = 32768;
pub const CASC_LOCALE_PTPT: u32 = 65536;
pub const CASC_CFLAG_LOAD_ON_WINDOWS: u32 = 8;
pub const CASC_CFLAG_LOAD_ON_MAC: u32 = 16;
pub const CASC_CFLAG_LOW_VIOLENCE: u32 = 128;
pub const CASC_CFLAG_DONT_LOAD: u32 = 256;
pub const CASC_CFLAG_NO_NAME_HASH: u32 = 268435456;
pub const CASC_CFLAG_BUNDLE: u32 = 1073741824;
pub const CASC_CFLAG_NO_COMPRESSION: u32 = 2147483648;
pub const CASC_INVALID_INDEX: u32 = 4294967295;
pub const CASC_INVALID_SIZE: u32 = 4294967295;
pub const CASC_INVALID_POS: u32 = 4294967295;
pub const CASC_INVALID_ID: u32 = 4294967295;
pub const CASC_INVALID_OFFS64: i32 = -1;
pub const CASC_INVALID_SIZE64: i32 = -1;
pub const CASC_FEATURE_FILE_NAMES: u32 = 1;
pub const CASC_FEATURE_ROOT_CKEY: u32 = 2;
pub const CASC_FEATURE_TAGS: u32 = 4;
pub const CASC_FEATURE_FNAME_HASHES: u32 = 8;
pub const CASC_FEATURE_FNAME_HASHES_OPTIONAL: u32 = 16;
pub const CASC_FEATURE_FILE_DATA_IDS: u32 = 32;
pub const CASC_FEATURE_LOCALE_FLAGS: u32 = 64;
pub const CASC_FEATURE_CONTENT_FLAGS: u32 = 128;
pub const CASC_FEATURE_ONLINE: u32 = 256;
pub const CASC_KEY_LENGTH: u32 = 16;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutex_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_mutex_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_mutex_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__opaque)
        )
    );
}
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type size_t = __darwin_size_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
pub type BYTE = ::std::os::raw::c_uchar;
pub type LONG = ::std::os::raw::c_int;
pub type DWORD = ::std::os::raw::c_uint;
pub type LONGLONG = ::std::os::raw::c_longlong;
pub type ULONGLONG = ::std::os::raw::c_ulonglong;
pub type PULONGLONG = *mut ::std::os::raw::c_ulonglong;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type TCHAR = ::std::os::raw::c_char;
pub type PDWORD = *mut DWORD;
pub type LPBYTE = *mut BYTE;
pub type LPCSTR = *const ::std::os::raw::c_char;
pub type LPCTSTR = *const TCHAR;
pub type CASC_LOCK = pthread_mutex_t;
pub const _CASC_STORAGE_INFO_CLASS_CascStorageLocalFileCount: _CASC_STORAGE_INFO_CLASS = 0;
pub const _CASC_STORAGE_INFO_CLASS_CascStorageTotalFileCount: _CASC_STORAGE_INFO_CLASS = 1;
pub const _CASC_STORAGE_INFO_CLASS_CascStorageFeatures: _CASC_STORAGE_INFO_CLASS = 2;
pub const _CASC_STORAGE_INFO_CLASS_CascStorageInstalledLocales: _CASC_STORAGE_INFO_CLASS = 3;
pub const _CASC_STORAGE_INFO_CLASS_CascStorageProduct: _CASC_STORAGE_INFO_CLASS = 4;
pub const _CASC_STORAGE_INFO_CLASS_CascStorageTags: _CASC_STORAGE_INFO_CLASS = 5;
pub const _CASC_STORAGE_INFO_CLASS_CascStoragePathProduct: _CASC_STORAGE_INFO_CLASS = 6;
pub const _CASC_STORAGE_INFO_CLASS_CascStorageInfoClassMax: _CASC_STORAGE_INFO_CLASS = 7;
pub type _CASC_STORAGE_INFO_CLASS = u32;
pub use self::_CASC_STORAGE_INFO_CLASS as CASC_STORAGE_INFO_CLASS;
pub const _CASC_FILE_INFO_CLASS_CascFileContentKey: _CASC_FILE_INFO_CLASS = 0;
pub const _CASC_FILE_INFO_CLASS_CascFileEncodedKey: _CASC_FILE_INFO_CLASS = 1;
pub const _CASC_FILE_INFO_CLASS_CascFileFullInfo: _CASC_FILE_INFO_CLASS = 2;
pub const _CASC_FILE_INFO_CLASS_CascFileSpanInfo: _CASC_FILE_INFO_CLASS = 3;
pub const _CASC_FILE_INFO_CLASS_CascFileInfoClassMax: _CASC_FILE_INFO_CLASS = 4;
pub type _CASC_FILE_INFO_CLASS = u32;
pub use self::_CASC_FILE_INFO_CLASS as CASC_FILE_INFO_CLASS;
pub const _CASC_NAME_TYPE_CascNameFull: _CASC_NAME_TYPE = 0;
pub const _CASC_NAME_TYPE_CascNameDataId: _CASC_NAME_TYPE = 1;
pub const _CASC_NAME_TYPE_CascNameCKey: _CASC_NAME_TYPE = 2;
pub const _CASC_NAME_TYPE_CascNameEKey: _CASC_NAME_TYPE = 3;
pub type _CASC_NAME_TYPE = u32;
pub use self::_CASC_NAME_TYPE as CASC_NAME_TYPE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _CASC_FIND_DATA {
    pub szFileName: [::std::os::raw::c_char; 1024usize],
    pub CKey: [BYTE; 16usize],
    pub EKey: [BYTE; 16usize],
    pub TagBitMask: ULONGLONG,
    pub FileSize: ULONGLONG,
    pub szPlainName: *mut ::std::os::raw::c_char,
    pub dwFileDataId: DWORD,
    pub dwLocaleFlags: DWORD,
    pub dwContentFlags: DWORD,
    pub dwSpanCount: DWORD,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub NameType: CASC_NAME_TYPE,
}
#[test]
fn bindgen_test_layout__CASC_FIND_DATA() {
    assert_eq!(
        ::std::mem::size_of::<_CASC_FIND_DATA>(),
        1104usize,
        concat!("Size of: ", stringify!(_CASC_FIND_DATA))
    );
    assert_eq!(
        ::std::mem::align_of::<_CASC_FIND_DATA>(),
        8usize,
        concat!("Alignment of ", stringify!(_CASC_FIND_DATA))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).szFileName as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(szFileName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).CKey as *const _ as usize },
        1024usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(CKey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).EKey as *const _ as usize },
        1040usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(EKey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).TagBitMask as *const _ as usize },
        1056usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(TagBitMask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).FileSize as *const _ as usize },
        1064usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(FileSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).szPlainName as *const _ as usize },
        1072usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(szPlainName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).dwFileDataId as *const _ as usize },
        1080usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(dwFileDataId)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).dwLocaleFlags as *const _ as usize },
        1084usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(dwLocaleFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).dwContentFlags as *const _ as usize },
        1088usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(dwContentFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).dwSpanCount as *const _ as usize },
        1092usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(dwSpanCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FIND_DATA>())).NameType as *const _ as usize },
        1100usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FIND_DATA),
            "::",
            stringify!(NameType)
        )
    );
}
impl _CASC_FIND_DATA {
    #[inline]
    pub fn bFileAvailable(&self) -> DWORD {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_bFileAvailable(&mut self, val: DWORD) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(bFileAvailable: DWORD) -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let bFileAvailable: u32 = unsafe { ::std::mem::transmute(bFileAvailable) };
            bFileAvailable as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type CASC_FIND_DATA = _CASC_FIND_DATA;
pub type PCASC_FIND_DATA = *mut _CASC_FIND_DATA;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CASC_STORAGE_TAG {
    pub szTagName: LPCSTR,
    pub TagNameLength: DWORD,
    pub TagValue: DWORD,
}
#[test]
fn bindgen_test_layout__CASC_STORAGE_TAG() {
    assert_eq!(
        ::std::mem::size_of::<_CASC_STORAGE_TAG>(),
        16usize,
        concat!("Size of: ", stringify!(_CASC_STORAGE_TAG))
    );
    assert_eq!(
        ::std::mem::align_of::<_CASC_STORAGE_TAG>(),
        8usize,
        concat!("Alignment of ", stringify!(_CASC_STORAGE_TAG))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_STORAGE_TAG>())).szTagName as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_TAG),
            "::",
            stringify!(szTagName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_STORAGE_TAG>())).TagNameLength as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_TAG),
            "::",
            stringify!(TagNameLength)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_STORAGE_TAG>())).TagValue as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_TAG),
            "::",
            stringify!(TagValue)
        )
    );
}
pub type CASC_STORAGE_TAG = _CASC_STORAGE_TAG;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CASC_STORAGE_TAGS {
    pub TagCount: size_t,
    pub Reserved: size_t,
    pub Tags: [CASC_STORAGE_TAG; 1usize],
}
#[test]
fn bindgen_test_layout__CASC_STORAGE_TAGS() {
    assert_eq!(
        ::std::mem::size_of::<_CASC_STORAGE_TAGS>(),
        32usize,
        concat!("Size of: ", stringify!(_CASC_STORAGE_TAGS))
    );
    assert_eq!(
        ::std::mem::align_of::<_CASC_STORAGE_TAGS>(),
        8usize,
        concat!("Alignment of ", stringify!(_CASC_STORAGE_TAGS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_STORAGE_TAGS>())).TagCount as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_TAGS),
            "::",
            stringify!(TagCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_STORAGE_TAGS>())).Reserved as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_TAGS),
            "::",
            stringify!(Reserved)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_STORAGE_TAGS>())).Tags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_TAGS),
            "::",
            stringify!(Tags)
        )
    );
}
pub type CASC_STORAGE_TAGS = _CASC_STORAGE_TAGS;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CASC_STORAGE_PRODUCT {
    pub szCodeName: [::std::os::raw::c_char; 28usize],
    pub BuildNumber: DWORD,
}
#[test]
fn bindgen_test_layout__CASC_STORAGE_PRODUCT() {
    assert_eq!(
        ::std::mem::size_of::<_CASC_STORAGE_PRODUCT>(),
        32usize,
        concat!("Size of: ", stringify!(_CASC_STORAGE_PRODUCT))
    );
    assert_eq!(
        ::std::mem::align_of::<_CASC_STORAGE_PRODUCT>(),
        4usize,
        concat!("Alignment of ", stringify!(_CASC_STORAGE_PRODUCT))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_STORAGE_PRODUCT>())).szCodeName as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_PRODUCT),
            "::",
            stringify!(szCodeName)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_STORAGE_PRODUCT>())).BuildNumber as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_STORAGE_PRODUCT),
            "::",
            stringify!(BuildNumber)
        )
    );
}
pub type CASC_STORAGE_PRODUCT = _CASC_STORAGE_PRODUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CASC_FILE_FULL_INFO {
    pub CKey: [BYTE; 16usize],
    pub EKey: [BYTE; 16usize],
    pub DataFileName: [::std::os::raw::c_char; 16usize],
    pub StorageOffset: ULONGLONG,
    pub SegmentOffset: ULONGLONG,
    pub TagBitMask: ULONGLONG,
    pub FileNameHash: ULONGLONG,
    pub ContentSize: ULONGLONG,
    pub EncodedSize: ULONGLONG,
    pub SegmentIndex: DWORD,
    pub SpanCount: DWORD,
    pub FileDataId: DWORD,
    pub LocaleFlags: DWORD,
    pub ContentFlags: DWORD,
}
#[test]
fn bindgen_test_layout__CASC_FILE_FULL_INFO() {
    assert_eq!(
        ::std::mem::size_of::<_CASC_FILE_FULL_INFO>(),
        120usize,
        concat!("Size of: ", stringify!(_CASC_FILE_FULL_INFO))
    );
    assert_eq!(
        ::std::mem::align_of::<_CASC_FILE_FULL_INFO>(),
        8usize,
        concat!("Alignment of ", stringify!(_CASC_FILE_FULL_INFO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).CKey as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(CKey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).EKey as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(EKey)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).DataFileName as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(DataFileName)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).StorageOffset as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(StorageOffset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).SegmentOffset as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(SegmentOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).TagBitMask as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(TagBitMask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).FileNameHash as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(FileNameHash)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).ContentSize as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(ContentSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).EncodedSize as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(EncodedSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).SegmentIndex as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(SegmentIndex)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).SpanCount as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(SpanCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).FileDataId as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(FileDataId)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).LocaleFlags as *const _ as usize
        },
        108usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(LocaleFlags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_FULL_INFO>())).ContentFlags as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_FULL_INFO),
            "::",
            stringify!(ContentFlags)
        )
    );
}
pub type CASC_FILE_FULL_INFO = _CASC_FILE_FULL_INFO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CASC_FILE_SPAN_INFO {
    pub CKey: [BYTE; 16usize],
    pub EKey: [BYTE; 16usize],
    pub StartOffset: ULONGLONG,
    pub EndOffset: ULONGLONG,
    pub ArchiveIndex: DWORD,
    pub ArchiveOffs: DWORD,
    pub HeaderSize: DWORD,
    pub FrameCount: DWORD,
}
#[test]
fn bindgen_test_layout__CASC_FILE_SPAN_INFO() {
    assert_eq!(
        ::std::mem::size_of::<_CASC_FILE_SPAN_INFO>(),
        64usize,
        concat!("Size of: ", stringify!(_CASC_FILE_SPAN_INFO))
    );
    assert_eq!(
        ::std::mem::align_of::<_CASC_FILE_SPAN_INFO>(),
        8usize,
        concat!("Alignment of ", stringify!(_CASC_FILE_SPAN_INFO))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).CKey as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(CKey)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).EKey as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(EKey)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).StartOffset as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(StartOffset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).EndOffset as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(EndOffset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).ArchiveIndex as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(ArchiveIndex)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).ArchiveOffs as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(ArchiveOffs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).HeaderSize as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(HeaderSize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_FILE_SPAN_INFO>())).FrameCount as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_FILE_SPAN_INFO),
            "::",
            stringify!(FrameCount)
        )
    );
}
pub type CASC_FILE_SPAN_INFO = _CASC_FILE_SPAN_INFO;
pub type PFNPROGRESSCALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        PtrUserParam: *mut ::std::os::raw::c_void,
        szWork: LPCSTR,
        szObject: LPCSTR,
        CurrentValue: DWORD,
        TotalValue: DWORD,
    ) -> bool,
>;
pub type PFNPRODUCTCALLBACK = ::std::option::Option<
    unsafe extern "C" fn(
        PtrUserParam: *mut ::std::os::raw::c_void,
        ProductList: *mut LPCSTR,
        ProductCount: size_t,
        PtrSelectedProduct: *mut size_t,
    ) -> bool,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _CASC_OPEN_STORAGE_ARGS {
    pub Size: size_t,
    pub szLocalPath: LPCTSTR,
    pub szCodeName: LPCTSTR,
    pub szRegion: LPCTSTR,
    pub PfnProgressCallback: PFNPROGRESSCALLBACK,
    pub PtrProgressParam: *mut ::std::os::raw::c_void,
    pub PfnProductCallback: PFNPRODUCTCALLBACK,
    pub PtrProductParam: *mut ::std::os::raw::c_void,
    pub dwLocaleMask: DWORD,
    pub dwFlags: DWORD,
    pub szBuildKey: LPCTSTR,
}
#[test]
fn bindgen_test_layout__CASC_OPEN_STORAGE_ARGS() {
    assert_eq!(
        ::std::mem::size_of::<_CASC_OPEN_STORAGE_ARGS>(),
        80usize,
        concat!("Size of: ", stringify!(_CASC_OPEN_STORAGE_ARGS))
    );
    assert_eq!(
        ::std::mem::align_of::<_CASC_OPEN_STORAGE_ARGS>(),
        8usize,
        concat!("Alignment of ", stringify!(_CASC_OPEN_STORAGE_ARGS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).Size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).szLocalPath as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(szLocalPath)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).szCodeName as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(szCodeName)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).szRegion as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(szRegion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).PfnProgressCallback as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(PfnProgressCallback)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).PtrProgressParam as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(PtrProgressParam)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).PfnProductCallback as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(PfnProductCallback)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).PtrProductParam as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(PtrProductParam)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).dwLocaleMask as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(dwLocaleMask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).dwFlags as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(dwFlags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_CASC_OPEN_STORAGE_ARGS>())).szBuildKey as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_CASC_OPEN_STORAGE_ARGS),
            "::",
            stringify!(szBuildKey)
        )
    );
}
pub type CASC_OPEN_STORAGE_ARGS = _CASC_OPEN_STORAGE_ARGS;
pub type PCASC_OPEN_STORAGE_ARGS = *mut _CASC_OPEN_STORAGE_ARGS;
extern "C" {
    pub fn CascOpenStorageEx(
        szParams: LPCTSTR,
        pArgs: PCASC_OPEN_STORAGE_ARGS,
        bOnlineStorage: bool,
        phStorage: *mut HANDLE,
    ) -> bool;
}
extern "C" {
    pub fn CascOpenStorage(szParams: LPCTSTR, dwLocaleMask: DWORD, phStorage: *mut HANDLE) -> bool;
}
extern "C" {
    pub fn CascOpenOnlineStorage(
        szParams: LPCTSTR,
        dwLocaleMask: DWORD,
        phStorage: *mut HANDLE,
    ) -> bool;
}
extern "C" {
    pub fn CascGetStorageInfo(
        hStorage: HANDLE,
        InfoClass: CASC_STORAGE_INFO_CLASS,
        pvStorageInfo: *mut ::std::os::raw::c_void,
        cbStorageInfo: size_t,
        pcbLengthNeeded: *mut size_t,
    ) -> bool;
}
extern "C" {
    pub fn CascCloseStorage(hStorage: HANDLE) -> bool;
}
extern "C" {
    pub fn CascOpenFile(
        hStorage: HANDLE,
        pvFileName: *const ::std::os::raw::c_void,
        dwLocaleFlags: DWORD,
        dwOpenFlags: DWORD,
        PtrFileHandle: *mut HANDLE,
    ) -> bool;
}
extern "C" {
    pub fn CascOpenLocalFile(
        szFileName: LPCTSTR,
        dwOpenFlags: DWORD,
        PtrFileHandle: *mut HANDLE,
    ) -> bool;
}
extern "C" {
    pub fn CascGetFileInfo(
        hFile: HANDLE,
        InfoClass: CASC_FILE_INFO_CLASS,
        pvFileInfo: *mut ::std::os::raw::c_void,
        cbFileInfo: size_t,
        pcbLengthNeeded: *mut size_t,
    ) -> bool;
}
extern "C" {
    pub fn CascGetFileSize64(hFile: HANDLE, PtrFileSize: PULONGLONG) -> bool;
}
extern "C" {
    pub fn CascSetFilePointer64(
        hFile: HANDLE,
        DistanceToMove: LONGLONG,
        PtrNewPos: PULONGLONG,
        dwMoveMethod: DWORD,
    ) -> bool;
}
extern "C" {
    pub fn CascReadFile(
        hFile: HANDLE,
        lpBuffer: *mut ::std::os::raw::c_void,
        dwToRead: DWORD,
        pdwRead: PDWORD,
    ) -> bool;
}
extern "C" {
    pub fn CascCloseFile(hFile: HANDLE) -> bool;
}
extern "C" {
    pub fn CascGetFileSize(hFile: HANDLE, pdwFileSizeHigh: PDWORD) -> DWORD;
}
extern "C" {
    pub fn CascSetFilePointer(
        hFile: HANDLE,
        lFilePos: LONG,
        PtrFilePosHigh: *mut LONG,
        dwMoveMethod: DWORD,
    ) -> DWORD;
}
extern "C" {
    pub fn CascFindFirstFile(
        hStorage: HANDLE,
        szMask: LPCSTR,
        pFindData: PCASC_FIND_DATA,
        szListFile: LPCTSTR,
    ) -> HANDLE;
}
extern "C" {
    pub fn CascFindNextFile(hFind: HANDLE, pFindData: PCASC_FIND_DATA) -> bool;
}
extern "C" {
    pub fn CascFindClose(hFind: HANDLE) -> bool;
}
extern "C" {
    pub fn CascAddEncryptionKey(hStorage: HANDLE, KeyName: ULONGLONG, Key: LPBYTE) -> bool;
}
extern "C" {
    pub fn CascAddStringEncryptionKey(hStorage: HANDLE, KeyName: ULONGLONG, szKey: LPCSTR) -> bool;
}
extern "C" {
    pub fn CascFindEncryptionKey(hStorage: HANDLE, KeyName: ULONGLONG) -> LPBYTE;
}
extern "C" {
    pub fn CascGetNotFoundEncryptionKey(hStorage: HANDLE, KeyName: *mut ULONGLONG) -> bool;
}
