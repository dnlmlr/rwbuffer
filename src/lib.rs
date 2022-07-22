#[cfg(test)]
mod test;

use std::io::{Read, Write};

pub trait RwBufferExt {
    fn put_u8(&mut self, val: u8) {
        self.put_slice(&[val]);
    }

    fn put_u16(&mut self, val: u16) {
        self.put_slice(&val.to_be_bytes());
    }

    fn put_u32(&mut self, val: u32) {
        self.put_slice(&val.to_be_bytes());
    }

    fn put_u64(&mut self, val: u64) {
        self.put_slice(&val.to_be_bytes());
    }

    fn put_u128(&mut self, val: u128) {
        self.put_slice(&val.to_be_bytes());
    }

    fn put_i8(&mut self, val: i8) {
        self.put_u8(val as u8);
    }

    fn put_i16(&mut self, val: i16) {
        self.put_u16(val as u16);
    }

    fn put_i32(&mut self, val: i32) {
        self.put_u32(val as u32);
    }

    fn put_i64(&mut self, val: i64) {
        self.put_u64(val as u64);
    }

    fn put_i128(&mut self, val: i128) {
        self.put_u128(val as u128);
    }

    fn put_f32(&mut self, val: f32) {
        self.put_slice(&val.to_be_bytes());
    }

    fn put_f64(&mut self, val: f64) {
        self.put_slice(&val.to_be_bytes());
    }

    fn put_slice(&mut self, val: &[u8]);

    fn get_u8(&mut self) -> Option<u8> {
        let mut buf = [0; 1];
        self.copy_to_slice(&mut buf)?;
        Some(u8::from_be_bytes(buf))
    }

    fn get_u16(&mut self) -> Option<u16> {
        let mut buf = [0; 2];
        self.copy_to_slice(&mut buf)?;
        Some(u16::from_be_bytes(buf))
    }

    fn get_u32(&mut self) -> Option<u32> {
        let mut buf = [0; 4];
        self.copy_to_slice(&mut buf)?;
        Some(u32::from_be_bytes(buf))
    }

    fn get_u64(&mut self) -> Option<u64> {
        let mut buf = [0; 8];
        self.copy_to_slice(&mut buf)?;
        Some(u64::from_be_bytes(buf))
    }

    fn get_u128(&mut self) -> Option<u128> {
        let mut buf = [0; 16];
        self.copy_to_slice(&mut buf)?;
        Some(u128::from_be_bytes(buf))
    }

    fn get_i8(&mut self) -> Option<i8> {
        self.get_u8().map(|it| it as i8)
    }

    fn get_i16(&mut self) -> Option<i16> {
        self.get_u16().map(|it| it as i16)
    }

    fn get_i32(&mut self) -> Option<i32> {
        self.get_u32().map(|it| it as i32)
    }

    fn get_i64(&mut self) -> Option<i64> {
        self.get_u64().map(|it| it as i64)
    }

    fn get_i128(&mut self) -> Option<i128> {
        self.get_u128().map(|it| it as i128)
    }

    fn get_f32(&mut self) -> Option<f32> {
        let mut buf = [0; 4];
        self.copy_to_slice(&mut buf)?;
        Some(f32::from_be_bytes(buf))
    }

    fn get_f64(&mut self) -> Option<f64> {
        let mut buf = [0; 8];
        self.copy_to_slice(&mut buf)?;
        Some(f64::from_be_bytes(buf))
    }

    fn get_str_zero_terminated(&mut self) -> Option<String> {
        let mut buf = Vec::new();
        loop {
            let byte = self.get_u8()?;
            if byte == 0 {
                break;
            }
            buf.push(byte);
        }
        Some(String::from_utf8(buf).unwrap())
    }

    fn copy_to_slice(&mut self, buf: &mut [u8]) -> Option<()>;
}

/// The RwBuffer contains multiple zones in the underlying Vec which are IsRead, Readable, Writable
/// and Expected.
/// - IsRead is the zone from `index=0` to `index=read_index` and designates data that was
///   previously read using the `get_*` functions.
/// - Readable is the zone from `index=read_index` to `index=write_index` and designates data that
///   is available to read with the `get_*` functions.
/// - Writable is the zone from `index=write_index` to `index=capacity` and designates data that
///   will be written by the `put_*` functions.
/// - Expected overlaps with Writable and is the zone from `index=write_index` to
///   `index=expected_len` and designates data that will be filled by using the `read_from`
///   function. This zone can also become *negative* if `expected<write_index`, in which case the
///   `read_from` function will do nothing but all `put_*` functions still work.
///
///
#[derive(Debug, Default)]
pub struct RwBuffer {
    /// The actual data buffer. Only a portion of this data is actually valid.
    buf: Vec<u8>,
    /// When reading from the buffer (`get_*`), this is the index of the next byte to read.
    read_index: usize,
    /// When writing to the buffer (`put_*`), this is the index of the next byte to write. This is
    /// also the effective used size of the buffer.
    write_index: usize,
    /// The expected length of data that will be read.
    expected_len: usize,
}

impl RwBufferExt for RwBuffer {
    fn put_slice(&mut self, val: &[u8]) {
        let unused = self.unused();
        // Grow the underlying vec buffer if necessary.
        if val.len() > unused {
            self.buf.reserve(val.len() - unused);
            self.buf.resize(self.buf.capacity(), 0);
        }
        // Copy the data into the buffer .
        self.buf[self.write_index..self.write_index + val.len()].copy_from_slice(val);
        self.write_index += val.len();
    }

    fn copy_to_slice(&mut self, buf: &mut [u8]) -> Option<()> {
        if buf.len() > self.remaining() {
            return None;
        }

        buf.copy_from_slice(&self.buf[self.read_index..self.read_index + buf.len()]);
        self.read_index += buf.len();

        Some(())
    }
}

impl RwBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: vec![0; capacity],
            read_index: 0,
            write_index: 0,
            expected_len: 0,
        }
    }

    pub fn with_expected(expected_len: usize) -> Self {
        Self {
            buf: Vec::with_capacity(expected_len),
            read_index: 0,
            write_index: 0,
            expected_len,
        }
    }

    /// Create the RwBuffer with the given capacity and expected length. If the capacity is less
    /// than the expected length, the buffer will be grown to the expected length and the given
    /// capacity will be ignored.
    pub fn with_capacity_and_expected(capacity: usize, expected_len: usize) -> Self {
        Self {
            buf: vec![0; capacity.max(expected_len)],
            read_index: 0,
            write_index: 0,
            expected_len,
        }
    }

    /// Expect a total of `expected` bytes to be written to this buffer in total. This will grow
    /// the internal buffer if necessary.
    pub fn set_expected(&mut self, expected: usize) {
        if self.capacity() < expected {
            self.buf.reserve(expected - self.capacity());
            self.buf.resize(expected, 0);
        }
        self.expected_len = expected;
    }

    pub fn expected_missing(&self) -> usize {
        self.expected_len.saturating_sub(self.write_index)
    }

    /// The number of bytes that can be read from this buffer using the `get_*` functions.
    pub fn remaining(&self) -> usize {
        self.write_index - self.read_index
    }

    /// The total number of bytes in the underlying buffer.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// The number of unused (writable) bytes in the currently allocated buffer.
    pub fn unused(&self) -> usize {
        self.capacity() - self.write_index
    }

    pub fn clear(&mut self) {
        self.read_index = 0;
        self.write_index = 0;
    }

    /// Remove every thing that was already read (index 0 -> read_index) and move the remaining
    /// data to the beginning of the buffer if necessary.
    pub fn clear_read(&mut self) {
        self.buf.copy_within(self.read_index..self.write_index, 0);
        self.write_index -= self.read_index;
        self.read_index = 0;
    }

    /// Reset the read index to the beginning of the buffer without discarding any data. This
    /// causes future reads to start from the beginning of the buffer again.
    pub fn reset_read_index(&mut self) {
        self.read_index = 0;
    }

    pub fn read_from<T: Read>(&mut self, read: &mut T) -> Result<usize, std::io::Error> {
        let missing = self.expected_missing();
        let bytes_read = read.read(&mut self.buf[self.write_index..missing])?;
        self.write_index += bytes_read;
        Ok(bytes_read)
    }

    pub fn write_to<T: Write>(&mut self, write: &mut T) -> Result<usize, std::io::Error> {
        let bytes_written = write.write(self.as_slice())?;
        self.read_index += bytes_written;
        Ok(bytes_written)
    }

    /// Get the used buffer as a slice, starting at the current read index. This does not advance
    /// the read index.
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[self.read_index..self.write_index]
    }

    /// Get the used buffer as a mutable slice, starting at the current read index. This does not
    /// advance the read index.
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.buf[self.read_index..self.write_index]
    }
}

impl Write for RwBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        self.put_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Read for RwBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let remaining = self.remaining();
        if buf.len() > remaining {
            self.copy_to_slice(&mut buf[..remaining]).unwrap();
            Ok(remaining)
        } else {
            self.copy_to_slice(&mut buf[..]).unwrap();
            Ok(buf.len())
        }
    }
}
