//! `StandardController` auto-read joypad bits.
//! `to_u16()` packs big-endian for `$4218` (JOY1L) / `$4219` (JOY1H).
use intbits::Bits;

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
#[derive(Clone, Default, Debug, Copy, PartialEq, Eq)]
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
        0_u16
            .with_bit(15, self.b)
            .with_bit(14, self.y)
            .with_bit(13, self.select)
            .with_bit(12, self.start)
            .with_bit(11, self.up)
            .with_bit(10, self.down)
            .with_bit(9, self.left)
            .with_bit(8, self.right)
            .with_bit(7, self.a)
            .with_bit(6, self.x)
            .with_bit(5, self.l)
            .with_bit(4, self.r)
            .with_bit(3, self.sig3)
            .with_bit(2, self.sig2)
            .with_bit(1, self.sig1)
            .with_bit(0, self.sig0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_u16_bit_positions() {
        assert_eq!(StandardController::default().to_u16(), 0);
        assert_eq!(
            StandardController {
                b: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 15
        );
        assert_eq!(
            StandardController {
                y: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 14
        );
        assert_eq!(
            StandardController {
                select: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 13
        );
        assert_eq!(
            StandardController {
                start: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 12
        );
        assert_eq!(
            StandardController {
                up: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 11
        );
        assert_eq!(
            StandardController {
                down: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 10
        );
        assert_eq!(
            StandardController {
                left: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 9
        );
        assert_eq!(
            StandardController {
                right: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 8
        );
        assert_eq!(
            StandardController {
                a: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 7
        );
        assert_eq!(
            StandardController {
                x: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 6
        );
        assert_eq!(
            StandardController {
                l: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 5
        );
        assert_eq!(
            StandardController {
                r: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 4
        );
        assert_eq!(
            StandardController {
                sig3: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 3
        );
        assert_eq!(
            StandardController {
                sig2: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 2
        );
        assert_eq!(
            StandardController {
                sig1: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 1
        );
        assert_eq!(
            StandardController {
                sig0: true,
                ..Default::default()
            }
            .to_u16(),
            1 << 0
        );

        let combined = StandardController {
            b: true,
            start: true,
            right: true,
            a: true,
            l: true,
            sig1: true,
            ..Default::default()
        };
        assert_eq!(combined.to_u16(), 0b1001_0001_1010_0010);
    }
}
