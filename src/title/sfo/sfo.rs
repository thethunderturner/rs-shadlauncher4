use crate::title::sfo::data_table::{SfoDataTable, read_data_table};
use crate::title::sfo::header::{SfoHeader, read_header};
use crate::title::sfo::index_table::{SfoIndexTable, read_index};
use crate::title::sfo::key_table::{SfoKeyTable, read_key_table};
use std::io::{Cursor, Error};
use suppaftp::FtpStream;
use suppaftp::types::FileType;

pub struct Sfo {
    pub header: SfoHeader,
    pub index_table: SfoIndexTable,
    pub key_table: SfoKeyTable,
    pub data_table: SfoDataTable,
}
// Read: https://www.psdevwiki.com/ps4/Param.sfo
/*
   Short description
   External Structure: An outer container, that may surround the SFO data in some contexts, like on a disc.
   The wiki itself labels this section as speculation and shows the actual SFO beginning later inside that wrapper, at offset 0x800.

   Internal Structure: The actual contents of param.sfo.
       Header: Header metadata
       Index table: Tells you where everything is
       Key table: Stores the names like TITLE, APP_VER
       Data table Stores the actual values like "Bloodborne", "01.09"
*/
pub fn read_sfo(ftp: &mut FtpStream, title_id: &str) -> std::io::Result<Sfo> {
    let path = format!("/system_data/priv/appmeta/{title_id}/param.sfo");

    // param.sfo is binary data.
    ftp.transfer_type(FileType::Binary).map_err(Error::other)?;

    // Download param.sfo.
    let buffer = ftp.retr_as_buffer(&path).map_err(Error::other)?;

    let mut f = Cursor::new(buffer.into_inner());

    // Read header
    let header = read_header(&mut f)?;

    // Read index table
    let index_table = read_index(&mut f, header.index_table_entries)?;

    // Read key table
    let key_table = read_key_table(&mut f, header.key_table_offset, header.data_table_offset)?;

    // Read data table
    let data_table = read_data_table(&mut f, &index_table, &key_table)?;

    Ok(Sfo {
        header,
        index_table,
        key_table,
        data_table,
    })
}
