use std::fmt;
use std::io::{Error, ErrorKind, Read, Result};

// Read: https://www.psdevwiki.com/ps4/Param.sfo#Key_table
/*
    From the header we know:
        key_table_offset
        data_table_offset
    Size of key table:
        data_table_offset - key_table_offset
    Keys are stored as null-terminated UTF-8 strings:
        APP_TYPE\0
        APP_VER\0
        ATTRIBUTE\0
        CATEGORY\0
        ...
    The key_offset inside an SfoIndexEntry is relative to the beginning of this table.
*/

pub struct SfoKeyParam {
    pub key: String,
    pub offset: u16,
}

pub struct SfoKeyTable {
    pub params: Vec<SfoKeyParam>,
}

impl SfoKeyTable {
    pub fn get_key(&self, offset: u16) -> Option<&str> {
        self.params
            .iter()
            .find(|param| param.offset == offset)
            .map(|param| param.key.as_str())
    }
}

impl fmt::Debug for SfoKeyParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SfoKeyParam")
            .field("key", &format_args!("{}", self.key))
            .field("offset", &format_args!("{:#X}", self.offset))
            .finish()
    }
}

impl fmt::Debug for SfoKeyTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SfoKeyTable")
            .field("params", &self.params)
            .finish()
    }
}

pub fn read_key_table<R: Read>(
    reader: &mut R,
    key_table_offset: u32,
    data_table_offset: u32,
) -> Result<SfoKeyTable> {
    let table_size = (data_table_offset - key_table_offset) as usize;

    let mut data = vec![0u8; table_size];
    reader.read_exact(&mut data)?;

    let mut params = Vec::new();
    let mut offset = 0usize;

    while offset < data.len() {
        // Padding at the end of the key table
        if data[offset] == 0 {
            break;
        }

        let remaining = &data[offset..];

        let length = remaining
            .iter()
            .position(|&byte| byte == 0)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Key table contains a non-terminated key",
                )
            })?;

        let key = std::str::from_utf8(&remaining[..length])
            .map_err(|_| Error::new(ErrorKind::InvalidData, "Key table contains invalid UTF-8"))?
            .to_string();

        params.push(SfoKeyParam {
            key,
            offset: offset as u16,
        });

        // Move past the string + null terminator
        offset += length + 1;
    }

    Ok(SfoKeyTable { params })
}
