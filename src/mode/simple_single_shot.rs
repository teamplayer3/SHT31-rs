use std::thread::sleep;
use std::time::Duration;
use embedded_hal::blocking::i2c;
use crate::mode::Sht31Reader;
use crate::{Accuracy, Reading, SHT31};
use crate::error::Result;
use crate::error::SHTError::PlaceholderError;
use crate::mode::single_shot::single_shot_read;

/// A simple reading that blocks until the measurement is obtained
pub struct SimpleSingleShot {
    max_retries: u8,
    ms_delay: u64
}

impl SimpleSingleShot {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { max_retries: 8, ms_delay: 100 }
    }
    /// Sets the max number of retries to read a sensor before giving up
    pub fn set_max_retries(&mut self, max_retries: u8) {self.max_retries=max_retries}
    /// Sets the max number of retries to read a sensor before giving up
    pub fn with_max_retries(mut self, max_retries: u8) -> Self {
        self.set_max_retries(max_retries);
        self
    }
    /// Sets the millisecond delay between each try
    pub fn set_delay(&mut self, ms_delay: u64) {self.ms_delay=ms_delay}
    /// Sets the millisecond delay between each try
    pub fn with_delay(mut self, ms_delay: u64) -> Self {
        self.set_delay(ms_delay);
        self
    }
}

impl<I2C> Sht31Reader for SHT31<SimpleSingleShot, I2C>
    where
        I2C: i2c::WriteRead + i2c::Write,
{
    /// Try reading
    fn read(&mut self) -> Result<Reading> {
        // Commence reading
        let lsb = match self.accuracy {
            Accuracy::High => 0x06,
            Accuracy::Medium => 0x0D,
            Accuracy::Low => 0x10,
        };

        self.i2c_write(&[0x2C, lsb])?;

        // TODO: figure out clock stretching
        let mut read_attempt = Err(PlaceholderError);

        for _ in 0..self.mode.max_retries {
            read_attempt = single_shot_read(self);

            if read_attempt.is_err() {
                sleep(Duration::from_millis(self.mode.ms_delay))
            }
            else {
                return read_attempt
            }
        }
        return read_attempt;
    }
}