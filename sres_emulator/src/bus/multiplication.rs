use crate::util::memory::AddressU24;
use crate::util::uint::U16Ext;

pub struct MultiplicationUnit {
    pub mul_a: u8,
    pub mul_b: u8,
    pub mul_result: u16,
    pub dividend: u16,
    pub divisor: u8,
    pub div_result: u16,
}

impl MultiplicationUnit {
    pub fn new() -> MultiplicationUnit {
        MultiplicationUnit {
            mul_a: 0,
            mul_b: 0,
            mul_result: 0,
            dividend: 0,
            divisor: 0,
            div_result: 0,
        }
    }

    pub fn bus_peek(&self, addr: AddressU24) -> Option<u8> {
        match addr.offset {
            0x4214 => Some(self.peek_rddivl()),
            0x4215 => Some(self.peek_rddivh()),
            0x4216 => Some(self.peek_rdmpyl()),
            0x4217 => Some(self.peek_rdmpyh()),
            _ => unreachable!(),
        }
    }

    pub fn bus_read(&mut self, addr: AddressU24) -> u8 {
        match addr.offset {
            0x4214 => self.peek_rddivl(),
            0x4215 => self.peek_rddivh(),
            0x4216 => self.peek_rdmpyl(),
            0x4217 => self.peek_rdmpyh(),
            _ => unreachable!(),
        }
    }

    pub fn bus_write(&mut self, addr: AddressU24, value: u8) {
        match addr.offset {
            0x4202 => self.write_wrmpya(value),
            0x4203 => self.write_wrmpyb(value),
            0x4204 => self.write_wrdivl(value),
            0x4205 => self.write_wrdivh(value),
            0x4206 => self.write_wrdivb(value),
            _ => unreachable!(),
        }
    }

    ///   WRMPYA
    ///   $4202
    /// 7  bit  0
    /// ---- ----
    /// NNNN NNNN
    /// |||| ||||
    /// ++++-++++- First number to multiply (8-bit, unsigned)
    fn write_wrmpya(&mut self, value: u8) {
        self.mul_a = value;
    }

    ///   WRMPYB
    ///   $4203
    /// 7  bit  0
    /// ---- ----
    /// NNNN NNNN
    /// |||| ||||
    /// ++++-++++- Second number to multiply (8-bit unsigned)
    fn write_wrmpyb(&mut self, value: u8) {
        self.mul_b = value;
        self.mul_result = (self.mul_a as u16) * (self.mul_b as u16);
    }

    ///   RDMPYH      RDMPYL
    ///   $4217       $4216
    /// 7  bit  0   7  bit  0
    /// ---- ----   ---- ----
    /// HHHH HHHH   LLLL LLLL
    /// |||| ||||   |||| ||||
    /// ++++-++++---++++-++++- Multiplication result (16-bit unsigned)
    fn peek_rdmpyh(&self) -> u8 {
        self.mul_result.high_byte()
    }

    fn peek_rdmpyl(&self) -> u8 {
        self.mul_result.low_byte()
    }

    ///   WRDIVH      WRDIVL
    ///   $4205       $4204
    /// 7  bit  0   7  bit  0
    /// ---- ----   ---- ----
    /// HHHH HHHH   LLLL LLLL
    /// |||| ||||   |||| ||||
    /// ++++-++++---++++-++++- Dividend (16-bit unsigned)
    fn write_wrdivh(&mut self, value: u8) {
        self.dividend.set_high_byte(value);
    }

    fn write_wrdivl(&mut self, value: u8) {
        self.dividend.set_low_byte(value);
    }

    ///               WRDIVB
    ///               $4206
    ///             7  bit  0
    ///             ---- ----
    ///             NNNN NNNN
    ///             |||| ||||
    ///             ++++-++++- Divisor (8-bit unsigned)
    fn write_wrdivb(&mut self, value: u8) {
        self.divisor = value;
        if self.divisor == 0 {
            self.div_result = 0xffff;
            self.mul_result = 0xffff;
        } else {
            self.div_result = self.dividend / (self.divisor as u16);
            self.mul_result = self.dividend % (self.divisor as u16);
        }
    }

    ///   RDDIVH      RDDIVL
    ///   $4215       $4214
    /// 7  bit  0   7  bit  0
    /// ---- ----   ---- ----
    /// HHHH HHHH   LLLL LLLL
    /// |||| ||||   |||| ||||
    /// ++++-++++---++++-++++- Division result (16-bit unsigned)
    fn peek_rddivh(&self) -> u8 {
        self.div_result.high_byte()
    }
    fn peek_rddivl(&self) -> u8 {
        self.div_result.low_byte()
    }
}

impl Default for MultiplicationUnit {
    fn default() -> Self {
        Self::new()
    }
}
