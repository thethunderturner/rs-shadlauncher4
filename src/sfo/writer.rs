use crate::sfo::internal::data_table::{SfoDataTable, SfoValue};
use crate::sfo::internal::header::read_header;
use crate::sfo::internal::index_table::read_index;
use std::fs::{self, OpenOptions};
use std::io::{self, Cursor, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

// Keep the original keys, offsets, formats, reserved lengths, and unrelated bytes.
// Editing a value must fit the space reserved for that field in the SFO.
fn update_bytes(mut bytes: Vec<u8>, table: &SfoDataTable) -> io::Result<Vec<u8>> {
    let mut cursor = Cursor::new(&bytes);
    let header = read_header(&mut cursor)?;
    if header.magic != *b"\0PSF" {
        return Err(invalid("This file is not an SFO."));
    }
    let count = header.index_table_entries as usize;
    let index_end = count
        .checked_mul(16)
        .and_then(|size| size.checked_add(20))
        .ok_or_else(|| invalid("SFO index is too large."))?;
    let keys_start = header.key_table_offset as usize;
    let data_start = header.data_table_offset as usize;
    if index_end > keys_start || keys_start > data_start || data_start > bytes.len() {
        return Err(invalid("Invalid SFO table offsets."));
    }
    if count != table.params.len() {
        return Err(invalid(
            "The SFO fields have changed. Re-scan the games before editing.",
        ));
    }
    let index = read_index(&mut cursor, header.index_table_entries)?;
    let mut updates = Vec::with_capacity(count);
    for (number, (entry, param)) in index.entries.iter().zip(&table.params).enumerate() {
        let key_start = keys_start
            .checked_add(entry.key_offset as usize)
            .ok_or_else(|| invalid("Invalid key offset."))?;
        let key_bytes = bytes
            .get(key_start..data_start)
            .ok_or_else(|| invalid("Invalid key offset."))?;
        let key_end = key_bytes
            .iter()
            .position(|&byte| byte == 0)
            .ok_or_else(|| invalid("SFO key is not terminated."))?;
        if key_bytes[..key_end] != *param.key.as_bytes() || entry.param_fmt != param.param_fmt {
            return Err(invalid(
                "The SFO fields have changed. Re-scan the games before editing.",
            ));
        }
        let start = data_start
            .checked_add(entry.data_offset as usize)
            .ok_or_else(|| invalid("Invalid data offset."))?;
        let end = start
            .checked_add(entry.param_max_len as usize)
            .ok_or_else(|| invalid("Invalid data length."))?;
        if end > bytes.len() || entry.param_len > entry.param_max_len {
            return Err(invalid("Invalid or overlapping SFO value ranges."));
        }
        let value = match (&param.data, entry.param_fmt) {
            (SfoValue::Utf8(text), 0x0004 | 0x0204) => {
                if text.contains('\0') {
                    return Err(invalid(format!(
                        "{} cannot contain a null character.",
                        param.key
                    )));
                }
                let mut value = text.as_bytes().to_vec();
                value.push(0);
                value
            }
            (SfoValue::Integer(value), 0x0404) => value.to_le_bytes().to_vec(),
            (SfoValue::Raw(value), format) if !matches!(format, 0x0004 | 0x0204 | 0x0404) => {
                value.clone()
            }
            _ => return Err(invalid(format!("{} has the wrong value type.", param.key))),
        };
        if value.len() > entry.param_max_len as usize {
            return Err(invalid(format!(
                "{} needs {} bytes, but this SFO reserves only {} bytes for it (including any text terminator). Shorten the value.",
                param.key,
                value.len(),
                entry.param_max_len,
            )));
        }
        updates.push((20 + number * 16 + 4, start, end, value));
    }
    let mut ranges: Vec<_> = updates
        .iter()
        .filter(|(_, start, end, _)| start != end)
        .map(|(_, start, end, _)| (*start, *end))
        .collect();
    ranges.sort_unstable();
    if ranges.windows(2).any(|pair| pair[0].1 > pair[1].0) {
        return Err(invalid("Overlapping SFO value ranges."));
    }
    for (length_offset, start, end, value) in updates {
        bytes[length_offset..length_offset + 4]
            .copy_from_slice(&(value.len() as u32).to_le_bytes());
        bytes[start..end].fill(0);
        bytes[start..start + value.len()].copy_from_slice(&value);
    }
    Ok(bytes)
}

pub fn write_sfo(path: &Path, table: &SfoDataTable) -> io::Result<()> {
    let path = fs::canonicalize(path)?;
    let permissions = fs::metadata(&path)?.permissions();
    if permissions.readonly() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "The SFO file is read-only.",
        ));
    }
    let bytes = update_bytes(fs::read(&path)?, table)?;
    let temp = path.with_file_name(format!(
        ".param.sfo.{}.{}.tmp",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    let result = (|| {
        file.write_all(&bytes)?;
        file.set_permissions(permissions)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&temp, &path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}
