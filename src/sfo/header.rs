use std::fmt;
use std::io::Read;

// Read: https://www.psdevwiki.com/ps4/Param.sfo#Header_SFO
/*
   Example:
      SfoHeader {
          magic: "PSF",
          version: 0x00000101,
          key_table_offset: 0x174,
          data_table_offset: 0x2CC,
          index_table_entries: 22,
      }
*/
pub struct SfoHeader {
    pub magic: [u8; 4],           // PSF
    pub version: u32,             // Version of the game
    pub key_table_offset: u32,    // Tells you where the key table starts
    pub data_table_offset: u32,   // Tells you where the data table starts
    pub index_table_entries: u32, // Tells you where the index table starts
}

impl fmt::Debug for SfoHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SfoHeader")
            .field("magic", &String::from_utf8_lossy(&self.magic[1..]))
            .field("version", &format_args!("{:#010X}", self.version))
            .field(
                "key_table_offset",
                &format_args!("{:#X}", self.key_table_offset),
            )
            .field(
                "data_table_offset",
                &format_args!("{:#X}", self.data_table_offset),
            )
            .field("index_table_entries", &self.index_table_entries)
            .finish()
    }
}

pub fn read_header<R: Read>(reader: &mut R) -> std::io::Result<SfoHeader> {
    let mut buffer = [0u8; 20];
    reader.read_exact(&mut buffer)?;

    let header = SfoHeader {
        magic: buffer[0..4].try_into().unwrap(),
        version: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
        key_table_offset: u32::from_le_bytes(buffer[8..12].try_into().unwrap()),
        data_table_offset: u32::from_le_bytes(buffer[12..16].try_into().unwrap()),
        index_table_entries: u32::from_le_bytes(buffer[16..20].try_into().unwrap()),
    };

    Ok(header)
}
