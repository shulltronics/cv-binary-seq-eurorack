Firmware for CV Binary Sequencer
================================
### By Shulltronics

## Feature testing
- [X] USB firmware upload (**TODO** make connector double sided)
- [ ] BOOTSEL button (**Issue** don't think this will work without the ROM chip)
- [ ] USB Serial Comms (**Issue** don't think this will work without the ROM chip)
- [X] Status LED over GPIO
- [X] LCD screen over SPI (**TODO** Test various SPI speeds, look at signal integrity)
- [X] Encoder button over GPIO
- [ ] Encoder rotation (over PIO?)
- [ ] MIDI IO over UART
- [ ] Trigger inputs over Interrupts
- [X] DAC comms over SPI (**TODO** Improve driver)
