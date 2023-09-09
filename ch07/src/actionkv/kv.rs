use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::Path,
};

use byteorder::ReadBytesExt;
use byteorder::{LittleEndian, WriteBytesExt};
use crc::{Crc, CRC_32_ISO_HDLC};
use serde::{Deserialize, Serialize};

type ByteString = Vec<u8>;
type ByteStr = [u8];
const CRC_32_ISO_HDLC_CALC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

#[derive(Debug)]
pub struct ActionKv {
    /// The file backing up the in-memory store
    f: File,

    /// Maintains a mapping between keys and file locations
    pub index: HashMap<ByteString, u64>,
}

impl ActionKv {
    /// Open a Key-Value store given its file path
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;

        let index = HashMap::new();

        Ok(ActionKv { f, index })
    }

    /// Populates the index mapping
    pub fn load(&mut self) -> io::Result<()> {
        let mut bufreader = BufReader::new(&mut self.f);

        loop {
            // position is the number of bytes from the start of the file to the current location.
            // We will store {key, position} in the index mapping.
            let position = bufreader.stream_position()?;
            let kv = match ActionKv::process_record(&mut bufreader) {
                Ok(kv) => {
                    println!(
                        "Loading {}, {}",
                        String::from_utf8_lossy(&kv.key),
                        String::from_utf8_lossy(&kv.value)
                    );
                    kv
                }
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => {
                        return Err(err);
                    }
                },
            };

            self.index.insert(kv.key, position);
        }

        println!("Index map:");
        for (k, i) in self.index.iter() {
            println!("\t{} -> {}", String::from_utf8_lossy(k), i);
        }

        Ok(())
    }

    /// Insert a (key/value) pair
    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    pub fn get(&mut self, key: &ByteStr) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => return Ok(None),
            Some(position) => *position,
        };
        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }

    /// Update a key with a value
    /// Because we're using an append-only design, to update a key, we
    /// simply write the new value at the end of the file.
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    /// Delete a key from the mapping
    /// Because we're using an append-only design, to delete a key,
    /// we simply write an empty value.
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

    /// Read a record from the current location of a reader
    fn process_record<R: Read>(reader: &mut R) -> io::Result<KeyValuePair> {
        let saved_checksum = reader.read_u32::<LittleEndian>()?;
        let key_len = reader.read_u32::<LittleEndian>()?;
        let val_len = reader.read_u32::<LittleEndian>()?;
        let data_len = key_len + val_len;

        let mut data = ByteString::with_capacity(data_len as usize);

        {
            reader
                .by_ref()
                .take(data_len as u64)
                .read_to_end(&mut data)?;
        }

        let checksum = CRC_32_ISO_HDLC_CALC.checksum(&data);
        if checksum != saved_checksum {
            panic!(
                "Data corruption encounted ({:08x} != {:08x})",
                checksum, saved_checksum
            );
        }

        let (key, value) = data.split_at(key_len as usize);

        Ok(KeyValuePair {
            key: key.to_vec(),
            value: value.to_vec(),
        })
    }

    /// Inserts (k, v) to the backing file
    /// Because we're using an append-only design,
    /// we will append the new value at the end of the file.
    /// In the file, there can be multiple (k, v) pairs, in which the last one
    /// is the updated version. Earlier pairs are stale values.
    /// Return the current position in the file, before writing (k, v)
    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut writer = BufWriter::new(&mut self.f);
        let key_len = key.len();
        let value_len = value.len();
        let mut tmp = ByteString::with_capacity(key_len + value_len);

        for &byte in key {
            tmp.push(byte);
        }

        for &byte in value {
            tmp.push(byte);
        }

        let checksum = CRC_32_ISO_HDLC_CALC.checksum(&tmp);

        // Write the new record (k, v) at the end of the file
        let record_position = writer.seek(SeekFrom::End(0))?;
        writer.write_u32::<LittleEndian>(checksum)?;
        writer.write_u32::<LittleEndian>(key_len as u32)?;
        writer.write_u32::<LittleEndian>(value_len as u32)?;
        writer.write_all(&tmp)?;

        Ok(record_position)
    }

    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        self.f.seek(SeekFrom::End(0))
    }

    /// Read the (key, value) pair at a given position in the file
    fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut reader = BufReader::new(&mut self.f);
        reader.seek(SeekFrom::Start(position))?;
        let kv = ActionKv::process_record(&mut reader)?;
        Ok(kv)
    }
}
