//! Simple CRC bindings backed by miniz.c

use std::io::IoResult;
use libc;

use ffi;

pub struct Crc {
    crc: libc::c_ulong,
    amt: u32,
}

pub struct CrcReader<R> {
    inner: R,
    crc: Crc,
}

impl Crc {
    pub fn new() -> Crc {
        Crc { crc: 0, amt: 0 }
    }

    pub fn sum(&self) -> libc::c_ulong { self.crc }
    pub fn amt(&self) -> u32 { self.amt }

    pub fn update(&mut self, with: &[u8]) {
        self.amt += with.len() as u32;
        self.crc = unsafe {
            ffi::mz_crc32(self.crc, with.as_ptr(), with.len() as libc::size_t)
        };
    }
}

impl<R: Reader> CrcReader<R> {
    pub fn new(r: R) -> CrcReader<R> {
        CrcReader { inner: r, crc: Crc::new() }
    }
    pub fn crc(&self) -> &Crc { &self.crc }
    pub fn unwrap(self) -> R { self.inner }
    pub fn inner(&mut self) -> &mut R { &mut self.inner }
}

impl<R: Reader> Reader for CrcReader<R> {
    fn read(&mut self, into: &mut [u8]) -> IoResult<uint> {
        let amt = try!(self.inner.read(into));
        self.crc.update(into.slice_to(amt));
        Ok(amt)
    }
}
