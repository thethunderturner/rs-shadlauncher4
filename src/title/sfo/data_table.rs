use crate::title::sfo::index_table::SfoIndexTable;
use crate::title::sfo::key_table::SfoKeyTable;
use std::fmt;
use std::io::{Error, ErrorKind, Read};

// Read: https://www.psdevwiki.com/ps4/Param.sfo#Data_table
/*
   From the index table we know:
       key_offset: Where the key is
       data_offset: Where the value of the key is
   Example:
       SfoIndexEntry {
           key_offset: 0x0,
           param_fmt: 0x0404,
           param_len: 4,
           param_max_len: 4,
           data_offset: 0x0,
       },
       SfoKeyTable {
           params: [
               SfoKeyParam {
                   key: APP_TYPE,
                   offset: 0x0,
               },
               ...
           ]
        }
        key_offset 0x0: APP_TYPE
        data_offset 0x0: to be found
*/

pub enum SfoValue {
    Utf8(String),
    Integer(u32),
    Raw(Vec<u8>),
}

pub struct SfoDataParam {
    pub key: String,
    pub data: SfoValue,
    pub key_offset: u16,
    pub data_offset: u32,
    pub param_fmt: u16,
}

pub struct SfoDataTable {
    pub params: Vec<SfoDataParam>,
}

impl SfoDataTable {
    // Returns the raw typed value of the given SFO parameter.
    pub fn find_param(&self, key: &str) -> Option<&SfoValue> {
        self.params
            .iter()
            .find(|param| param.key == key)
            .map(|param| &param.data)
    }

    // Returns a UTF-8 parameter as a String.
    pub fn find_string(&self, key: &str) -> Option<String> {
        match self.find_param(key)? {
            SfoValue::Utf8(value) => Some(value.clone()),
            _ => None,
        }
    }

    // Returns an integer parameter as a u32.
    pub fn find_integer(&self, key: &str) -> Option<u32> {
        match self.find_param(key)? {
            SfoValue::Integer(value) => Some(*value),
            _ => None,
        }
    }
}

impl fmt::Debug for SfoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SfoValue::Utf8(value) => {
                write!(f, "{:?}", value)
            }
            SfoValue::Integer(value) => {
                write!(f, "{}", value)
            }
            SfoValue::Raw(value) => {
                write!(f, "0x")?;

                for byte in value {
                    write!(f, "{:02X}", byte)?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Debug for SfoDataParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SfoDataParam")
            .field("key", &format_args!("{}", self.key))
            .field("data", &self.data)
            .field("key_offset", &format_args!("{:#X}", self.key_offset))
            .field("data_offset", &format_args!("{:#X}", self.data_offset))
            .field("param_fmt", &format_args!("{:#06X}", self.param_fmt))
            .finish()
    }
}

impl fmt::Debug for SfoDataTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SfoDataTable")
            .field("params", &self.params)
            .finish()
    }
}

pub fn read_data_table<R: Read>(
    reader: &mut R,
    index_table: &SfoIndexTable,
    key_table: &SfoKeyTable,
) -> std::io::Result<SfoDataTable> {
    /*
        Work out the complete size of the data table.
        For every entry:
            data_offset + param_max_len
        This will tell us where that entry's reserved data area ends.
        The largest one therefore gives us the size of the table.
    */
    let table_size = index_table
        .entries
        .iter()
        .map(|entry| entry.data_offset as usize + entry.param_max_len as usize)
        .max()
        .unwrap_or(0);

    let mut data = vec![0u8; table_size];
    reader.read_exact(&mut data)?;

    let mut params = Vec::with_capacity(index_table.entries.len());

    for entry in &index_table.entries {
        // Resolve key_offset -> actual key
        let key = key_table
            .get_key(entry.key_offset)
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid key offset: {:#X}", entry.key_offset),
                )
            })?
            .to_string();

        // data_offset is relative to the beginning of the data table.
        let start = entry.data_offset as usize;
        let end = start + entry.param_len as usize;

        if end > data.len() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Data for '{}' exceeds the data table", key),
            ));
        }

        let raw_data = &data[start..end];
        let data = match entry.param_fmt {
            /*
                UTF-8 special mode.
                    It is still UTF-8 data. The wiki identifies this as a special mode used by system-generated content.
            */
            0x0004 => {
                let bytes = raw_data.strip_suffix(&[0]).unwrap_or(raw_data);

                let string = std::str::from_utf8(bytes)
                    .map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!("Invalid UTF-8 data for '{}'", key),
                        )
                    })?
                    .to_string();

                SfoValue::Utf8(string)
            }

            // Normal UTF-8 string. param_len includes the terminating null byte.
            0x0204 => {
                let bytes = raw_data.strip_suffix(&[0]).unwrap_or(raw_data);

                let string = std::str::from_utf8(bytes)
                    .map_err(|_| {
                        Error::new(
                            ErrorKind::InvalidData,
                            format!("Invalid UTF-8 data for '{}'", key),
                        )
                    })?
                    .to_string();

                SfoValue::Utf8(string)
            }

            // Unsigned 32-bit integer.
            0x0404 => {
                if raw_data.len() != 4 {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Integer '{}' has invalid length {}", key, raw_data.len()),
                    ));
                }

                let bytes: [u8; 4] = raw_data.try_into().unwrap();
                SfoValue::Integer(u32::from_le_bytes(bytes))
            }

            // Unknown format. Keep the bytes instead of throwing them away.
            _ => SfoValue::Raw(raw_data.to_vec()),
        };

        params.push(SfoDataParam {
            key,
            key_offset: entry.key_offset,
            data_offset: entry.data_offset,
            param_fmt: entry.param_fmt,
            data,
        });
    }

    Ok(SfoDataTable { params })
}
