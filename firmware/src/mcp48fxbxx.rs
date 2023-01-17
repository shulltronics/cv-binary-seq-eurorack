use embedded_hal::spi;

pub enum Instruction {
    
}

pub struct MCP48FXBxx<SPI, CS>
{
    spi: SPI,
    cs: CS,
}

impl<SPI, CS> MCP48FXBxx<SPI, CS>
where
    SPI: embedded_hal::blocking::spi::Write<u8>,
    CS: embedded_hal::digital::v2::OutputPin,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self {spi, cs}
    }
}