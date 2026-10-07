use std::fmt;
use std::io::Read;

// Read: https://www.psdevwiki.com/ps4/Param.sfo#Index_table
/*
   Example: Printing the header gives you something like:
   In the SfoHeader, we have the "index_table_entries" field.
   Each index entry is 0x10 bytes = 16 bytes, and it repeats index_table_entries times
   So the index table occupies:
       start = 0x14
       size  = 22 × 0x10 = 0x160
       end   = 0x14 + 0x160 = 0x174
*/
pub struct SfoIndexEntry {
    pub key_offset: u16, // actual key position. So 0x174 (key_table_offset) + 0x14D = 0x2C1. You find null-terminated name of parameter.
    pub param_fmt: u16, // Format of parameter (UTF-8 special mode, UTF-8 string, 32-bit unsigned integer)
    pub param_len: u32, // The actual value currently occupies 6 bytes, including the terminating "\0" for a UTF-8 entry
    pub param_max_len: u32, // The # of bytes that are reserved for this value in the data table
    pub data_offset: u32, // relative to the beginning of the data table (incrementing as table grows)
}

#[derive(Debug)]
pub struct SfoIndexTable {
    pub entries: Vec<SfoIndexEntry>,
}

impl fmt::Debug for SfoIndexEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SfoIndexEntry")
            .field("key_offset", &format_args!("{:#X}", self.key_offset))
            .field("param_fmt", &format_args!("{:#06X}", self.param_fmt))
            .field("param_len", &self.param_len)
            .field("param_max_len", &self.param_max_len)
            .field("data_offset", &format_args!("{:#X}", self.data_offset))
            .finish()
    }
}

pub fn read_index<R: Read>(reader: &mut R, entry_count: u32) -> std::io::Result<SfoIndexTable> {
    let mut table = SfoIndexTable {
        entries: Vec::new(),
    };

    for _ in 0..entry_count {
        let mut buffer = [0u8; 16];

        reader.read_exact(&mut buffer)?;

        let entry = SfoIndexEntry {
            key_offset: u16::from_le_bytes(buffer[0..2].try_into().unwrap()),
            param_fmt: u16::from_le_bytes(buffer[2..4].try_into().unwrap()),
            param_len: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
            param_max_len: u32::from_le_bytes(buffer[8..12].try_into().unwrap()),
            data_offset: u32::from_le_bytes(buffer[12..16].try_into().unwrap()),
        };

        table.entries.push(entry);
    }

    Ok(table)
}
