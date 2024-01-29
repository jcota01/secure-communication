
pub fn rcon(i: u8) -> u8 {
    match i {
        1 => 0x01,
        2 => 0x02,
        3 => 0x04,
        4 => 0x08,
        5 => 0x10,
        6 => 0x20,
        7 => 0x40,
        8 => 0x80,
        9 => 0x1b,
        10 => 0x36,
        _ => panic!("rcon index out of bounds")
    }
}