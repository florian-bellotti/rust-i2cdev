// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::unix::prelude::*;
use nix;

use ::linux::ffi;
use ::{I2CSMBus, I2CMaster};

impl I2CMaster for AsRawFd {
    /// Select the slave with the given address
    ///
    /// Typically the address is expected to be 7-bits but 10-bit addresses
    /// may be supported by the kernel driver in some cases.  Little validation
    /// is done in Rust as the kernel is good at making sure things are valid.
    fn set_slave_address(&self, slave_address: u16) -> Result<(), nix::Error> {
        ffi::i2c_set_slave_address(self.as_raw_fd(), slave_address)
    }
}

impl I2CSMBus for AsRawFd {

    /// This sends a single bit to the device, at the place of the Rd/Wr bit
    fn smbus_write_quick(&self, bit: bool) -> Result<(), nix::Error> {
        ffi::i2c_smbus_write_quick(self.as_raw_fd(), bit)
    }

    /// Read a single byte from a device, without specifying a device register
    ///
    /// Some devices are so simple that this interface is enough; for
    /// others, it is a shorthand if you want to read the same register as in
    /// the previous SMBus command.
    fn smbus_read_byte(&self) -> Result<u8, nix::Error> {
        ffi::i2c_smbus_read_byte(self.as_raw_fd())
    }

    /// Write a single byte to a sdevice, without specifying a device register
    ///
    /// This is the opposite operation as smbus_read_byte.  As with read_byte,
    /// no register is specified.
    fn smbus_write_byte(&self, value: u8) -> Result<(), nix::Error> {
        ffi::i2c_smbus_write_byte(self.as_raw_fd(), value)
    }

    /// Read a single byte from a device, from a designated register
    ///
    /// The register is specified through the Comm byte.
    fn smbus_read_byte_data(&self, register: u8) -> Result<u8, nix::Error> {
        ffi::i2c_smbus_read_byte_data(self.as_raw_fd(), register)
    }

    /// Write a single byte to a specific register on a device
    ///
    /// The register is specified through the Comm byte.
    fn smbus_write_byte_data(&self, register: u8, value: u8) -> Result<(), nix::Error> {
        ffi::i2c_smbus_write_byte_data(self.as_raw_fd(), register, value)
    }

    /// Read 2 bytes form a given register on a device
    fn smbus_read_word_data(&self, register: u8) -> Result<u16, nix::Error> {
        ffi::i2c_smbus_read_word_data(self.as_raw_fd(), register)
    }

    /// Write 2 bytes to a given register on a device
    fn smbus_write_word_data(&self, register: u8, value: u16) -> Result<(), nix::Error> {
        ffi::i2c_smbus_write_word_data(self.as_raw_fd(), register, value)
    }

    /// Select a register, send 16 bits of data to it, and read 16 bits of data
    fn smbus_process_word(&self, register: u8, value: u16) -> Result<u16, nix::Error> {
        ffi::i2c_smbus_process_call(self.as_raw_fd(), register, value)
    }

    /// Read a block of up to 32 bytes from a device
    ///
    /// The actual number of bytes available to read is returned in the count
    /// byte.  This code returns a correctly sized vector containing the
    /// count bytes read from the device.
    fn smbus_read_block_data(&self, register: u8) -> Result<Vec<u8>, nix::Error> {
        ffi::i2c_smbus_read_block_data(self.as_raw_fd(), register)
    }

    /// Write a block of up to 32 bytes to a device
    ///
    /// The opposite of the Block Read command, this writes up to 32 bytes to
    /// a device, to a designated register that is specified through the
    /// Comm byte. The amount of data is specified in the Count byte.
    fn smbus_write_block_data(&self, register: u8, values: &[u8]) -> Result<(), nix::Error> {
        ffi::i2c_smbus_write_block_data(self.as_raw_fd(), register, values)
    }

    /// Select a register, send 1 to 31 bytes of data to it, and reads
    /// 1 to 31 bytes of data from it.
    fn smbus_process_block(&self, register: u8, values: &[u8]) -> Result<(), nix::Error> {
        ffi::i2c_smbus_write_i2c_block_data(self.as_raw_fd(), register, values)
    }

}