// -----------------------------------------------------------------------------
// Defines

pub const CASCLIB_VERSION: u32 = 0x0100;  // Current version of CascLib (1.0)
pub const CASCLIB_VERSION_STRING: &'static str = "1.00";  // String version of CascLib version

// Values for CascOpenStorage
pub const CASC_STOR_XXXXX: u32 = 0x00000001;  // Not used

// Values for CascOpenFile
pub const CASC_OPEN_BY_ENCODING_KEY: u32 = 0x00000001;  // The name is just the encoding key; skip ROOT file processing

pub const CASC_LOCALE_ALL: u32 = 0xFFFFFFFF;
pub const CASC_LOCALE_NONE: u32 = 0x00000000;
pub const CASC_LOCALE_UNKNOWN1: u32 = 0x00000001;
pub const CASC_LOCALE_ENUS: u32 = 0x00000002;
pub const CASC_LOCALE_KOKR: u32 = 0x00000004;
pub const CASC_LOCALE_RESERVED: u32 = 0x00000008;
pub const CASC_LOCALE_FRFR: u32 = 0x00000010;
pub const CASC_LOCALE_DEDE: u32 = 0x00000020;
pub const CASC_LOCALE_ZHCN: u32 = 0x00000040;
pub const CASC_LOCALE_ESES: u32 = 0x00000080;
pub const CASC_LOCALE_ZHTW: u32 = 0x00000100;
pub const CASC_LOCALE_ENGB: u32 = 0x00000200;
pub const CASC_LOCALE_ENCN: u32 = 0x00000400;
pub const CASC_LOCALE_ENTW: u32 = 0x00000800;
pub const CASC_LOCALE_ESMX: u32 = 0x00001000;
pub const CASC_LOCALE_RURU: u32 = 0x00002000;
pub const CASC_LOCALE_PTBR: u32 = 0x00004000;
pub const CASC_LOCALE_ITIT: u32 = 0x00008000;
pub const CASC_LOCALE_PTPT: u32 = 0x00010000;

pub const CASC_LOCALE_BIT_ENUS: u8 = 0x01;
pub const CASC_LOCALE_BIT_KOKR: u8 = 0x02;
pub const CASC_LOCALE_BIT_RESERVED: u8 = 0x03;
pub const CASC_LOCALE_BIT_FRFR: u8 = 0x04;
pub const CASC_LOCALE_BIT_DEDE: u8 = 0x05;
pub const CASC_LOCALE_BIT_ZHCN: u8 = 0x06;
pub const CASC_LOCALE_BIT_ESES: u8 = 0x07;
pub const CASC_LOCALE_BIT_ZHTW: u8 = 0x08;
pub const CASC_LOCALE_BIT_ENGB: u8 = 0x09;
pub const CASC_LOCALE_BIT_ENCN: u8 = 0x0A;
pub const CASC_LOCALE_BIT_ENTW: u8 = 0x0B;
pub const CASC_LOCALE_BIT_ESMX: u8 = 0x0C;
pub const CASC_LOCALE_BIT_RURU: u8 = 0x0D;
pub const CASC_LOCALE_BIT_PTBR: u8 = 0x0E;
pub const CASC_LOCALE_BIT_ITIT: u8 = 0x0F;
pub const CASC_LOCALE_BIT_PTPT: u8 = 0x10;


pub const MAX_CASC_KEY_LENGTH: usize = 0x10;  // Maximum length of the key (equal to MD5 hash)

pub const MD5_HASH_SIZE: usize = 0x10;
pub const MD5_STRING_SIZE: usize = 0x20;

pub const SHA1_DIGEST_SIZE: usize = 0x14; // 160 bits

pub const LANG_NEUTRAL: u32 = 0x00;  // Neutral locale

// Return value for CascGetFileSize and CascSetFilePointer
pub const CASC_INVALID_SIZE: usize = 0xFFFFFFFF;
pub const CASC_INVALID_POS: usize = 0xFFFFFFFF;

// Flags for CascGetStorageInfo
pub const CASC_FEATURE_LISTFILE: u32 = 0x00000001;  // The storage supports listfile