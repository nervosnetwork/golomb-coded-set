// The code is copy from: https://github.com/rust-bitcoin/rust-bitcoin/blob/a148e0673665a099d2771bf9c2dcf3402b75e7de/src/util/bip158.rs

// Rust Bitcoin Library
// Written in 2019 by
//   The rust-bitcoin developers
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

// This module was largely copied from https://github.com/rust-bitcoin/murmel/blob/master/src/blockfilter.rs
// on 11. June 2019 which is licensed under Apache, that file specifically
// was written entirely by Tamas Blummer, who is re-licensing its contents here as CC0.

//!
//! # BIP158 GCS Data structure
//!

use std::cmp::Ordering;
use std::collections::HashSet;
use std::{cmp, io};

use bitcoin_hashes::siphash24;

/// Golomb encoding parameter as in BIP-158, see also https://gist.github.com/sipa/576d5f09c3b86c3b1b75598d799fc845
pub const P: u8 = 19;
pub const M: u64 = 784_931;

/// Golomb-Rice encoded filter reader
pub struct GCSFilterReader<H> {
    filter: GCSFilter<H>,
    m: u64,
}

impl<H: ToU64Hasher> GCSFilterReader<H> {
    /// Create a new filter reader with specific seed to siphash
    pub fn new(hasher: H, m: u64, p: u8) -> GCSFilterReader<H> {
        GCSFilterReader {
            filter: GCSFilter::new(hasher, p),
            m,
        }
    }

    /// match any query pattern
    pub fn match_any(
        &self,
        reader: &mut dyn io::Read,
        query: &mut dyn Iterator<Item = &[u8]>,
    ) -> Result<bool, io::Error> {
        let mut decoder = reader;

        // NOTE: bitcoin use VarInt here
        let mut length_data = [0u8; 8];
        let n_elements = decoder
            .read_exact(&mut length_data)
            .map(|()| u64::from_le_bytes(length_data))
            .unwrap_or(0);

        let reader = &mut decoder;
        // map hashes to [0, n_elements << grp]
        let nm = n_elements * self.m;
        let mut mapped = query
            .map(|e| map_to_range(self.filter.hash(e), nm))
            .collect::<Vec<_>>();
        // sort
        mapped.sort_unstable();
        if mapped.is_empty() {
            return Ok(true);
        }
        if n_elements == 0 {
            return Ok(false);
        }

        // find first match in two sorted arrays in one read pass
        let mut reader = BitStreamReader::new(reader);
        let mut data = self.filter.golomb_rice_decode(&mut reader)?;
        let mut remaining = n_elements - 1;
        for p in mapped {
            loop {
                match data.cmp(&p) {
                    Ordering::Equal => {
                        return Ok(true);
                    }
                    Ordering::Less => {
                        if remaining > 0 {
                            data += self.filter.golomb_rice_decode(&mut reader)?;
                            remaining -= 1;
                        } else {
                            return Ok(false);
                        }
                    }
                    Ordering::Greater => {
                        break;
                    }
                }
            }
        }
        Ok(false)
    }

    /// match all query pattern
    pub fn match_all(
        &self,
        reader: &mut dyn io::Read,
        query: &mut dyn Iterator<Item = &[u8]>,
    ) -> Result<bool, io::Error> {
        let mut decoder = reader;

        // NOTE: bitcoin use VarInt here
        let mut length_data = [0u8; 8];
        let n_elements = decoder
            .read_exact(&mut length_data)
            .map(|()| u64::from_le_bytes(length_data))
            .unwrap_or(0);

        let reader = &mut decoder;
        // map hashes to [0, n_elements << grp]
        let nm = n_elements * self.m;
        let mut mapped = query
            .map(|e| map_to_range(self.filter.hash(e), nm))
            .collect::<Vec<_>>();
        // sort
        mapped.sort_unstable();
        mapped.dedup();
        if mapped.is_empty() {
            return Ok(true);
        }
        if n_elements == 0 {
            return Ok(false);
        }

        // figure if all mapped are there in one read pass
        let mut reader = BitStreamReader::new(reader);
        let mut data = self.filter.golomb_rice_decode(&mut reader)?;
        let mut remaining = n_elements - 1;
        for p in mapped {
            loop {
                match data.cmp(&p) {
                    Ordering::Equal => {
                        break;
                    }
                    Ordering::Less => {
                        if remaining > 0 {
                            data += self.filter.golomb_rice_decode(&mut reader)?;
                            remaining -= 1;
                        } else {
                            return Ok(false);
                        }
                    }
                    Ordering::Greater => {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }
}

// fast reduction of hash to [0, nm) range
fn map_to_range(hash: u64, nm: u64) -> u64 {
    ((hash as u128 * nm as u128) >> 64) as u64
}

/// Colomb-Rice encoded filter writer
pub struct GCSFilterWriter<'a, H> {
    filter: GCSFilter<H>,
    writer: &'a mut dyn io::Write,
    elements: HashSet<Vec<u8>>,
    m: u64,
}

impl<'a, H: ToU64Hasher> GCSFilterWriter<'a, H> {
    /// Create a new GCS writer wrapping a generic writer, with specific seed to siphash
    pub fn new(writer: &'a mut dyn io::Write, hasher: H, m: u64, p: u8) -> GCSFilterWriter<'a, H> {
        GCSFilterWriter {
            filter: GCSFilter::new(hasher, p),
            writer,
            elements: HashSet::new(),
            m,
        }
    }

    /// Add some data to the filter
    pub fn add_element(&mut self, element: &[u8]) {
        if !element.is_empty() {
            self.elements.insert(element.to_vec());
        }
    }

    /// write the filter to the wrapped writer
    pub fn finish(&mut self) -> Result<usize, io::Error> {
        let nm = self.elements.len() as u64 * self.m;

        // map hashes to [0, n_elements * M)
        let mut mapped: Vec<_> = self
            .elements
            .iter()
            .map(|e| map_to_range(self.filter.hash(e.as_slice()), nm))
            .collect();
        mapped.sort_unstable();

        // NOTE: bitcoin use VarInt here
        // write number of elements as varint
        let mut wrote = self.writer.write(&(mapped.len() as u64).to_le_bytes())?;

        // write out deltas of sorted values into a Golonb-Rice coded bit stream
        let mut writer = BitStreamWriter::new(self.writer);
        let mut last = 0;
        for data in mapped {
            wrote += self.filter.golomb_rice_encode(&mut writer, data - last)?;
            last = data;
        }
        wrote += writer.flush()?;
        Ok(wrote)
    }
}

pub trait ToU64Hasher {
    fn hash_to_u64(&self, element: &[u8]) -> u64;
}

pub struct Sip24Hasher {
    k0: u64, // sip hash key
    k1: u64, // sip hash key
}
impl Sip24Hasher {
    pub fn new(k0: u64, k1: u64) -> Sip24Hasher {
        Sip24Hasher { k0, k1 }
    }
}
impl ToU64Hasher for Sip24Hasher {
    fn hash_to_u64(&self, element: &[u8]) -> u64 {
        siphash24::Hash::hash_to_u64_with_keys(self.k0, self.k1, element)
    }
}

/// Golomb Coded Set Filter
struct GCSFilter<H> {
    hasher: H,
    p: u8,
}

impl<H: ToU64Hasher> GCSFilter<H> {
    /// Create a new filter
    fn new(hasher: H, p: u8) -> GCSFilter<H> {
        GCSFilter { hasher, p }
    }

    /// Golomb-Rice encode a number n to a bit stream (Parameter 2^k)
    fn golomb_rice_encode(&self, writer: &mut BitStreamWriter, n: u64) -> Result<usize, io::Error> {
        let mut wrote = 0;
        let mut q = n >> self.p;
        while q > 0 {
            let nbits = cmp::min(q, 64);
            wrote += writer.write(!0u64, nbits as u8)?;
            q -= nbits;
        }
        wrote += writer.write(0, 1)?;
        wrote += writer.write(n, self.p)?;
        Ok(wrote)
    }

    /// Golomb-Rice decode a number from a bit stream (Parameter 2^k)
    fn golomb_rice_decode(&self, reader: &mut BitStreamReader) -> Result<u64, io::Error> {
        let mut q = 0u64;
        while reader.read(1)? == 1 {
            q += 1;
        }
        let r = reader.read(self.p)?;
        Ok((q << self.p) + r)
    }

    /// Hash an arbitrary slice with siphash using parameters of this filter
    fn hash(&self, element: &[u8]) -> u64 {
        self.hasher.hash_to_u64(element)
    }
}

/// Bitwise stream reader
pub struct BitStreamReader<'a> {
    buffer: [u8; 1],
    offset: u8,
    reader: &'a mut dyn io::Read,
}

impl<'a> BitStreamReader<'a> {
    /// Create a new BitStreamReader that reads bitwise from a given reader
    pub fn new(reader: &'a mut dyn io::Read) -> BitStreamReader {
        BitStreamReader {
            buffer: [0u8],
            reader,
            offset: 8,
        }
    }

    /// Read nbit bits
    pub fn read(&mut self, mut nbits: u8) -> Result<u64, io::Error> {
        if nbits > 64 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "can not read more than 64 bits at once",
            ));
        }
        let mut data = 0u64;
        while nbits > 0 {
            if self.offset == 8 {
                self.reader.read_exact(&mut self.buffer)?;
                self.offset = 0;
            }
            let bits = cmp::min(8 - self.offset, nbits);
            data <<= bits;
            data |= ((self.buffer[0] << self.offset) >> (8 - bits)) as u64;
            self.offset += bits;
            nbits -= bits;
        }
        Ok(data)
    }
}

/// Bitwise stream writer
pub struct BitStreamWriter<'a> {
    buffer: [u8; 1],
    offset: u8,
    writer: &'a mut dyn io::Write,
}

impl<'a> BitStreamWriter<'a> {
    /// Create a new BitStreamWriter that writes bitwise to a given writer
    pub fn new(writer: &'a mut dyn io::Write) -> BitStreamWriter {
        BitStreamWriter {
            buffer: [0u8],
            writer,
            offset: 0,
        }
    }

    /// Write nbits bits from data
    pub fn write(&mut self, data: u64, mut nbits: u8) -> Result<usize, io::Error> {
        if nbits > 64 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "can not write more than 64 bits at once",
            ));
        }
        let mut wrote = 0;
        while nbits > 0 {
            let bits = cmp::min(8 - self.offset, nbits);
            self.buffer[0] |= ((data << (64 - nbits)) >> (64 - 8 + self.offset)) as u8;
            self.offset += bits;
            nbits -= bits;
            if self.offset == 8 {
                wrote += self.flush()?;
            }
        }
        Ok(wrote)
    }

    /// flush bits not yet written
    pub fn flush(&mut self) -> Result<usize, io::Error> {
        if self.offset > 0 {
            self.writer.write_all(&self.buffer)?;
            self.buffer[0] = 0u8;
            self.offset = 0;
            Ok(1)
        } else {
            Ok(0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashSet;
    use std::io::Cursor;

    #[test]
    fn test_filter() {
        let mut patterns = HashSet::new();

        patterns.insert(hex::decode("000000").unwrap());
        patterns.insert(hex::decode("111111").unwrap());
        patterns.insert(hex::decode("222222").unwrap());
        patterns.insert(hex::decode("333333").unwrap());
        patterns.insert(hex::decode("444444").unwrap());
        patterns.insert(hex::decode("555555").unwrap());
        patterns.insert(hex::decode("666666").unwrap());
        patterns.insert(hex::decode("777777").unwrap());
        patterns.insert(hex::decode("888888").unwrap());
        patterns.insert(hex::decode("999999").unwrap());
        patterns.insert(hex::decode("aaaaaa").unwrap());
        patterns.insert(hex::decode("bbbbbb").unwrap());
        patterns.insert(hex::decode("cccccc").unwrap());
        patterns.insert(hex::decode("dddddd").unwrap());
        patterns.insert(hex::decode("eeeeee").unwrap());
        patterns.insert(hex::decode("ffffff").unwrap());

        let mut out = Cursor::new(Vec::new());
        {
            let mut writer = GCSFilterWriter::new(&mut out, Sip24Hasher::new(0, 0), M, P);
            for p in &patterns {
                writer.add_element(p.as_slice());
            }
            writer.finish().unwrap();
        }

        let bytes = out.into_inner();

        {
            let query = vec![
                hex::decode("abcdef").unwrap(),
                hex::decode("eeeeee").unwrap(),
            ];
            let reader = GCSFilterReader::new(Sip24Hasher::new(0, 0), M, P);
            let mut input = Cursor::new(bytes.clone());
            assert!(reader
                .match_any(&mut input, &mut query.iter().map(|v| v.as_slice()))
                .unwrap());
        }
        {
            let query = vec![
                hex::decode("abcdef").unwrap(),
                hex::decode("123456").unwrap(),
            ];
            let reader = GCSFilterReader::new(Sip24Hasher::new(0, 0), M, P);
            let mut input = Cursor::new(bytes.clone());
            assert!(!reader
                .match_any(&mut input, &mut query.iter().map(|v| v.as_slice()))
                .unwrap());
        }
        {
            let reader = GCSFilterReader::new(Sip24Hasher::new(0, 0), M, P);
            let mut query = Vec::new();
            for p in &patterns {
                query.push(p.clone());
            }
            let mut input = Cursor::new(bytes.clone());
            assert!(reader
                .match_all(&mut input, &mut query.iter().map(|v| v.as_slice()))
                .unwrap());
        }
        {
            let reader = GCSFilterReader::new(Sip24Hasher::new(0, 0), M, P);
            let mut query = Vec::new();
            for p in &patterns {
                query.push(p.clone());
            }
            query.push(hex::decode("abcdef").unwrap());
            let mut input = Cursor::new(bytes);
            assert!(!reader
                .match_all(&mut input, &mut query.iter().map(|v| v.as_slice()))
                .unwrap());
        }
    }

    #[test]
    fn test_bit_stream() {
        let mut out = Cursor::new(Vec::new());
        {
            let mut writer = BitStreamWriter::new(&mut out);
            writer.write(0, 1).unwrap(); // 0
            writer.write(2, 2).unwrap(); // 10
            writer.write(6, 3).unwrap(); // 110
            writer.write(11, 4).unwrap(); // 1011
            writer.write(1, 5).unwrap(); // 00001
            writer.write(32, 6).unwrap(); // 100000
            writer.write(7, 7).unwrap(); // 0000111
            writer.flush().unwrap();
        }
        let bytes = out.into_inner();
        assert_eq!(
            "01011010110000110000000001110000",
            format!(
                "{:08b}{:08b}{:08b}{:08b}",
                bytes[0], bytes[1], bytes[2], bytes[3]
            )
        );
        {
            let mut input = Cursor::new(bytes);
            let mut reader = BitStreamReader::new(&mut input);
            assert_eq!(reader.read(1).unwrap(), 0);
            assert_eq!(reader.read(2).unwrap(), 2);
            assert_eq!(reader.read(3).unwrap(), 6);
            assert_eq!(reader.read(4).unwrap(), 11);
            assert_eq!(reader.read(5).unwrap(), 1);
            assert_eq!(reader.read(6).unwrap(), 32);
            assert_eq!(reader.read(7).unwrap(), 7);
            // 4 bits remained
            assert!(reader.read(5).is_err());
        }
    }
}
