use packed_struct::prelude::*;

/// Standard Controller bit layout:
///    JOY1H       JOY1L
///    $4219       $4218
/// 15  bit  8   7  bit  0
///  ---- ----   ---- ----
///  BYsS UDLR   AXlr 0000
///  |||| ||||   |||| ||||
///  |||| ||||   |||| ++++- Signature
///  |||| ||||   ||++------ L/R shoulder buttons
///  |||| ||||   ++-------- A/X buttons
///  |||| ++++------------- D-pad
///  ||++------------------ Select (s) and Start (S)
///  ++-------------------- B/Y buttons
#[derive(PackedStruct, Clone, Default, Debug, Copy, PartialEq, Eq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct StandardController {
    pub b: bool,
    pub y: bool,
    pub select: bool,
    pub start: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub x: bool,
    pub l: bool,
    pub r: bool,
    pub sig3: bool,
    pub sig2: bool,
    pub sig1: bool,
    pub sig0: bool,
}

impl StandardController {
    pub fn to_u16(&self) -> u16 {
        u16::from_be_bytes(self.pack().unwrap())
    }
}
