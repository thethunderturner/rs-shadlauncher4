//! Read ATRAC9 RIFF/WAVE audio one superframe at a time.
use crate::gui::centralpanel::actions::row::atrac9::libatrac9::Atrac9Decoder;
use std::fs::File;
use std::io::{self, BufReader, Read, Seek, SeekFrom};
use std::path::Path;

pub(in crate::gui::centralpanel::actions::row) struct At9 {
    reader: BufReader<File>,
    decoder: Atrac9Decoder,
    encoded: Vec<u8>,
    pcm: Vec<i16>,
    remaining: u64,
    pub channels: u16,
    pub sample_rate: u32,
}

fn invalid(message: impl ToString) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.to_string())
}

impl At9 {
    pub fn open(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let file_size = file.metadata()?.len();
        let mut reader = BufReader::new(file);
        let mut header = [0; 12];
        reader.read_exact(&mut header)?;
        if &header[..4] != b"RIFF" || &header[8..] != b"WAVE" {
            return Err(invalid("Expected an ATRAC9 RIFF/WAVE file"));
        }
        let end = u32::from_le_bytes(header[4..8].try_into().unwrap()) as u64 + 8;
        if end > file_size || end < 12 {
            return Err(invalid("Invalid RIFF size"));
        }
        let mut format = None;
        let mut audio = None;
        let mut position = 12;
        while position + 8 <= end {
            reader.seek(SeekFrom::Start(position))?;
            let mut chunk = [0; 8];
            reader.read_exact(&mut chunk)?;
            let size = u32::from_le_bytes(chunk[4..].try_into().unwrap()) as u64;
            let start = position + 8;
            if start + size > end {
                return Err(invalid("Truncated RIFF chunk"));
            }
            match &chunk[..4] {
                b"fmt " => {
                    if size < 48 {
                        return Err(invalid("ATRAC9 format chunk is too short"));
                    }
                    let mut bytes = [0; 48];
                    reader.read_exact(&mut bytes)?;
                    if bytes[..2] != [0xFE, 0xFF] {
                        return Err(invalid("Expected an extensible ATRAC9 format"));
                    }
                    let config: [u8; 4] = bytes[44..48].try_into().unwrap();
                    let decoder = Atrac9Decoder::new(&config).map_err(invalid)?;
                    let info = decoder.codec_info();
                    if u16::from_le_bytes(bytes[2..4].try_into().unwrap()) as usize != info.channels
                        || u32::from_le_bytes(bytes[4..8].try_into().unwrap())
                            != info.sampling_rate as u32
                        || u16::from_le_bytes(bytes[12..14].try_into().unwrap()) as usize
                            != info.superframe_size
                    {
                        return Err(invalid(
                            "ATRAC9 format does not match the codec configuration",
                        ));
                    }
                    format = Some(decoder);
                }
                b"data" => audio = Some((start, size)),
                _ => {}
            }
            if format.is_some() && audio.is_some() {
                break;
            }
            position = start + size + size % 2;
        }
        let decoder = format.ok_or_else(|| invalid("Missing ATRAC9 format chunk"))?;
        let (start, remaining) = audio.ok_or_else(|| invalid("Missing ATRAC9 data chunk"))?;
        let info = decoder.codec_info();
        if remaining % info.superframe_size as u64 != 0 {
            return Err(invalid("Incomplete ATRAC9 superframe"));
        }
        reader.seek(SeekFrom::Start(start))?;
        Ok(Self {
            reader,
            encoded: vec![0; info.superframe_size],
            pcm: vec![0; info.channels * info.frame_samples * info.frames_in_superframe],
            channels: info.channels as u16,
            sample_rate: info.sampling_rate as u32,
            decoder,
            remaining,
        })
    }

    pub fn next_buffer(&mut self) -> io::Result<Option<Vec<f32>>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.reader.read_exact(&mut self.encoded)?;
        self.decoder
            .decode(&self.encoded, &mut self.pcm)
            .map_err(invalid)?;
        self.remaining -= self.encoded.len() as u64;
        Ok(Some(
            self.pcm
                .iter()
                .map(|&sample| sample as f32 / 32768.0)
                .collect(),
        ))
    }
}
