#![cfg(test)]

use super::*;

#[test]
fn test_read_write_register() {
    let mut s_dsp = SDsp::default();
    s_dsp.write_register(0x0, 0x12);
    assert_eq!(s_dsp.read_register(0x0), 0x12);
}
