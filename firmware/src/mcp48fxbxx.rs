use embedded_hal::spi;

// An enumeration of the device status and control registers
// for non-volatile versions, add 0x10
#[repr(u8)]
pub enum Instruction {
    DAC0_REG       = 0x00,
    DAC1_REG       = 0x01,
    DAC2_REG       = 0x02,
    DAC3_REG       = 0x03,
    DAC4_REG       = 0x04,
    DAC5_REG       = 0x05,
    DAC6_REG       = 0x06,
    DAC7_REG       = 0x07,
    VREF_REG       = 0x08,
    PWR_REG        = 0x09,
    GAIN_STAT      = 0x0A,
    WIPERLOCK_STAT = 0x0B,
}

// The main DAC struct, with SPI interface
pub struct MCP48FXBxx<SPI, CS>
{
    spi: SPI,
    cs: CS,
}

impl<SPI, CS> MCP48FXBxx<SPI, CS>
where
    SPI: embedded_hal::blocking::spi::Write<u8>,
    CS:  embedded_hal::digital::v2::OutputPin,
{
    pub fn new(spi: SPI, cs: CS) -> Self {
        Self {spi, cs}
    }

    pub fn init(&mut self) {
        self.cs.set_high();
    }

    pub fn read(&self) {

    }

    pub fn write(&mut self, data: u8) {
        let buf = [data];
        // drive the CS pin low
        self.cs.set_low();
        self.spi.write(&buf);
        self.cs.set_high();
    }

    // test method to write data to device
    pub fn setup(&mut self) {
        // I think this would set DAC1 output to full voltage
        let buf = [0x01 << 3 | 0b000, 0x0F, 0xFF];
        self.cs.set_low();
        self.spi.write(&buf);
        self.cs.set_high();
    }
}