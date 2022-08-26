use super::*;
use embedded_hal_one::{serial::blocking, serial::nb as serial, serial::ErrorType};

impl<USART: Instance> ErrorType for Tx<USART> {
    type Error = Infallible;
}

impl<USART: Instance> ErrorType for Rx<USART> {
    type Error = Error;
}

impl<USART: Instance, PINS> ErrorType for Serial<USART, PINS> {
    type Error = Error;
}

impl<USART: Instance> serial::Write<u8> for Tx<USART> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.flush()
    }
}

impl<USART: Instance> serial::Write<u16> for Tx<USART> {
    fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
        self.write_u16(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.flush()
    }
}

impl<USART: Instance> serial::Read<u8> for Rx<USART> {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.read()
    }
}

impl<USART: Instance> serial::Read<u16> for Rx<USART> {
    fn read(&mut self) -> nb::Result<u16, Self::Error> {
        self.read_u16()
    }
}

impl<USART: Instance, PINS> serial::Write<u8> for Serial<USART, PINS> {
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.tx.write(word).unwrap();
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush().unwrap();
        Ok(())
    }
}

impl<USART: Instance, PINS> serial::Write<u16> for Serial<USART, PINS> {
    fn write(&mut self, word: u16) -> nb::Result<(), Self::Error> {
        self.tx.write_u16(word).unwrap();
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.tx.flush().unwrap();
        Ok(())
    }
}

impl<USART: Instance, PINS> serial::Read<u8> for Serial<USART, PINS> {
    fn read(&mut self) -> nb::Result<u8, Error> {
        self.rx.read()
    }
}

impl<USART: Instance, PINS> serial::Read<u16> for Serial<USART, PINS> {
    fn read(&mut self) -> nb::Result<u16, Error> {
        self.rx.read_u16()
    }
}

// Blocking

impl<USART: Instance> blocking::Write<u8> for Tx<USART> {
    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.bwrite_all(buffer)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.bflush()
    }
}

impl<USART: Instance> blocking::Write<u16> for Tx<USART> {
    fn write(&mut self, buffer: &[u16]) -> Result<(), Self::Error> {
        self.bwrite_all_u16(buffer)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.bflush()
    }
}

impl<USART: Instance, PINS> blocking::Write<u8> for Serial<USART, PINS> {
    fn write(&mut self, buffer: &[u8]) -> Result<(), Self::Error> {
        self.tx.bwrite_all(buffer).unwrap();
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.bflush().unwrap();
        Ok(())
    }
}

impl<USART: Instance, PINS> blocking::Write<u16> for Serial<USART, PINS> {
    fn write(&mut self, buffer: &[u16]) -> Result<(), Self::Error> {
        self.tx.bwrite_all_u16(buffer).unwrap();
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.bflush().unwrap();
        Ok(())
    }
}
