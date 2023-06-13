// https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf
#![allow(dead_code)]

use embedded_hal::blocking::i2c::Write;

pub const SSD_1306_WIDTH: u8 = 128;
pub const SSD_1306_HEIGHT: u8 = 64;

pub mod command;

use command::{Command, AddressMode};

pub struct SSD1306<I> where I: Write {
    i2c: I,
    address: u8,
}

impl<I> SSD1306<I> where I: Write {
    const DATA_BYTE: u8 = 0x40;
    
    pub fn new(address: u8, i2c: I) -> Self {
        Self {
            i2c,
            address,
        }
    }
    
    pub fn initialize(&mut self) -> Result<(), I::Error> {
        self.send_command(Command::DisplayEnable(false))?;
        self.send_command(Command::DisplayClockDiv{oscillator_freq: 0x8, divide_ratio: 0x0})?;
        self.send_command(Command::SetMultiplexRatio{ratio: 63})?;
        self.i2c.write(self.address, &[0x00, 0xD3, 0x00])?; // Set Display Offset 0
        self.i2c.write(self.address, &[0x00, 0x40])?; // Set Display Start Line 0
        self.send_command(Command::ChargePump(true))?;
        self.send_command(Command::SetAddressMode(AddressMode::Page))?;
        
        self.i2c.write(self.address, &[0x00, 0xDA, 0x12])?; // SetComPinConfig(true, false) (wut?)
        
        // Set the rotation to zero
        self.i2c.write(self.address, &[0x00, 0xA1])?; // SetSegmentRemap(true)
        self.i2c.write(self.address, &[0x00, 0xC8])?; // ReverseComDirection(true)
        
        // set default brightness
        self.i2c.write(self.address, &[0x00, 0xD9, 0x21])?; // SetPreChargePeriod(1, 2)
        self.send_command(Command::SetContrast{contrast: 0x5F})?;
        
        self.i2c.write(self.address, &[0x00, 0xDB, 0x40])?; // SetVcomhDeselect(VcomhLevel::Auto) (what???)
        self.send_command(Command::AllPixelsOn(false))?; // (should be false)
        self.i2c.write(self.address, &[0x00, 0xA6])?; // InvertDisplay(false)
        self.i2c.write(self.address, &[0x00, 0x2E])?; // EnableScroll(false)
        
        self.send_command(Command::DisplayEnable(true))?;
        
        Ok(())
    }
    
    #[inline(always)]
    pub fn send_data(&mut self, data: &[u8]) -> Result<(), I::Error> {
        // only write to the display in small chunks of bytes
        const CHUNK_SIZE: usize = 16;
        
        let mut buffer: [u8; CHUNK_SIZE + 1] = [Self::DATA_BYTE; CHUNK_SIZE + 1];
        
        for chunk in data.chunks(CHUNK_SIZE) {
            buffer[1..=chunk.len()].copy_from_slice(chunk);
            self.i2c.write(self.address, &buffer[..=chunk.len()])?
        }
        
        Ok(())
    }
    
    #[inline(always)]
    pub fn send_command(&mut self, command: Command) -> Result<(), I::Error> {
        macro_rules! cmd {
            [$($x:expr),+ $(,)?] => { self.i2c.write(self.address, &[$($x),+]) }
        }
        
        match command {
            // Fundamental commands
            Command::SetContrast{contrast} => cmd![0x00, 0x81, contrast],
            Command::AllPixelsOn(enable) => cmd![0x00, 0xA4 | (enable as u8), 0xE3],
            Command::InvertDisplay(enable) => cmd![0x00, 0xA6 | (enable as u8), 0xE3],
            Command::DisplayEnable(enable) => cmd![0x00, 0xAE | (enable as u8), 0xE3],
            
            // Scrolling commands
            // [0x00, 0x26 | (direction as u8), 0x00, start_page as u8, interval as u8, end_page as u8, 0x00, 0xFF],
            Command::SetupHorizontalScroll { .. } => unimplemented!(),
            // [0x00, 0x28 | (direction as u8), 0x00, start_page as u8, interval as u8, end_page as u8, vertical_offset as u8],
            Command::SetupVerticalAndHorizontalScroll { .. } => unimplemented!(),
            
            // Starting position commands
            Command::ColumnStart(addr) => cmd![0x00, 0x0F & addr, 0x10 | ((addr >> 4) & 0x0F)],
            Command::LowerColumnStart(addr) => cmd![0x00, 0x00 | (addr & 0x0F), 0xE3],
            Command::UpperColumnStart(addr) => cmd![0x00, 0x10 | (addr & 0x0F), 0xE3],
            Command::PageStart(page) => cmd![0x00, 0xB0 | (page as u8), 0xE3],
            
            // Addressing mode commands
            Command::SetAddressMode(mode) => cmd![0x00, 0x20, mode as u8],
            // Command::SetColumnAddress{start_col: u8, end_col: u8} => [0x00, 0x21, start_col, end_col],
            // Command::SetPageAddress{start_page, end_page} => [0x00, 0x22, start_page as u8, end_page as u8],
            Command::EnableScroll(enable) => cmd![0x00, 0x2E | (enable as u8), 0xE3],
            
            // Brightness-related commands
            Command::ChargePump(enable) => cmd![0x00, 0x8D, 0x10 | ((enable as u8) << 2)],
            
            // Display-related commands
            Command::SetMultiplexRatio { ratio } => cmd![0x00, 0xA8, ratio],
            Command::DisplayClockDiv {
                oscillator_freq: osc,
                divide_ratio: div
            } => cmd![0x00, 0xD5, ((osc & 0x0F) << 4) | (div & 0x0F)],
            
            // No operation
            Command::NoOp => cmd![0x00, 0xE3, 0xE3],
            
            _ => todo!()
        }
    }
    
}
