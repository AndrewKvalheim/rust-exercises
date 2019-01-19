mod inspectable;

use inspectable::Inspectable;
use std::io::{Read, Result, Write};

pub type ReadStats<T> = Stats<T>;
pub type WriteStats<T> = Stats<T>;

//
// Common
//

pub struct Stats<T> {
    bytes: usize,
    io: T,
    transfers: usize,
}

impl<T> Stats<T> {
    pub fn new(io: T) -> Self {
        Self {
            bytes: 0,
            io,
            transfers: 0,
        }
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn get_ref(&self) -> &T {
        &self.io
    }

    pub fn transfers(&self) -> usize {
        self.transfers
    }
}

//
// Read
//

impl<T: Read> Stats<T> {
    pub fn reads(&self) -> usize {
        self.transfers()
    }
}

impl<T: Read> Read for Stats<T> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.io.read(buffer).inspect(|&bytes_through| {
            self.bytes += bytes_through;
            self.transfers += 1;
        })
    }
}

//
// Write
//

impl<T: Write> Stats<T> {
    pub fn writes(&self) -> usize {
        self.transfers()
    }
}

impl<T: Write> Write for Stats<T> {
    fn flush(&mut self) -> Result<()> {
        self.io.flush()
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize> {
        self.io.write(buffer).inspect(|&bytes_through| {
            self.bytes += bytes_through;
            self.transfers += 1;
        })
    }
}
