pub(super) fn box_char(mask: u8) -> Option<char> {
    // mask is one byte per edge
    //    N E S W
    // 0b00000000
    //
    // 00: no line
    // 01: single line
    // 10: double line
    // 11: double line
    // this bit fuckery makes sure that 2 becomes 3
    Some(match mask | ((mask & 0b10101010) >> 1) {
        0b00_00_00_00 => return None,
        0b01_00_00_00 => '┴',  // optional case
        0b00_00_01_00 => '┬',  // optional case
        0b01_00_01_00 |
        0b11_00_01_00 |
        0b01_00_11_00 =>
            '│',
        0b01_00_01_01 =>
            '┤',
        0b01_00_01_11 =>
            '╡',
        0b11_00_11_01 |
        0b01_00_11_01 |
        0b11_00_01_01 =>
            '╢',
        0b00_00_11_01 =>
            '╖',
        0b00_00_01_11 =>
            '╕',
        0b11_00_11_11 |
        0b01_00_11_11 |
        0b11_00_01_11 =>
            '╣',
        0b11_00_00_00 => '╨',  // optional case
        0b00_00_11_00 => '╥',  // optional case
        0b11_00_11_00 =>
            '║',
        0b00_00_11_11 =>
            '╗',
        0b11_00_00_11 =>
            '╝',
        0b11_00_00_01 =>
            '╜',
        0b01_00_00_11 =>
            '╛',
        0b00_00_01_01 =>
            '┐',
        0b01_01_00_00 =>
            '└',
        0b01_01_00_01 |
        0b01_11_00_01 |
        0b01_01_00_11 =>
            '┴',
        0b00_01_01_01 |
        0b00_11_01_01 |
        0b00_01_01_11 =>
            '┬',
        0b01_01_01_00 =>
            '├',
        0b00_01_00_00 => 
            '├', // optional case
        0b00_00_00_01 => 
            '┤', // optional case
        0b00_01_00_01 |
        0b00_11_00_01 |
        0b00_01_00_11 =>
            '─',
        0b01_01_01_01 |
        0b11_01_01_01 |
        0b01_11_01_01 |
        0b01_01_11_01 |
        0b01_01_01_11 |
        0b11_11_01_01 |
        0b01_11_11_01 |
        0b01_01_11_11 |
        0b11_01_01_11 =>
            '┼',
        0b01_11_01_00 =>
            '╞',
        0b11_01_11_00 |
        0b01_01_11_00 |
        0b11_01_01_00 =>
            '╟',
        0b11_11_00_00 =>
            '╚',
        0b00_11_11_00 =>
            '╔',
        0b11_11_00_11 |
        0b11_01_00_11 |
        0b11_11_00_01 =>
            '╩',
        0b00_11_11_11 |
        0b00_01_11_11 |
        0b00_11_11_01 =>
            '╦',
        0b11_11_11_00 |
        0b01_11_11_00 |
        0b11_11_01_00 =>
            '╠',
        0b00_11_00_00 => '╞',  // optional case
        0b00_00_00_11 => '╡',  // optional case
        0b00_11_00_11 =>
            '═',
        0b11_11_11_11 |
        0b01_11_11_11 |
        0b11_01_11_11 |
        0b11_11_01_11 |
        0b11_11_11_01 =>
            '╬',
        0b01_11_00_11 =>
            '╧',
        0b11_01_00_01 =>
            '╨',
        0b00_11_01_11 =>
            '╤',
        0b00_01_11_01 =>
            '╥',
        0b11_01_00_00 =>
            '╙',
        0b01_11_00_00 =>
            '╘',
        0b00_11_01_00 =>
            '╒',
        0b00_01_11_00 =>
            '╓',
        0b11_01_11_01 =>
            '╫',
        0b01_11_01_11 =>
            '╪',
        0b01_00_00_01 =>
            '┘',
        0b00_01_01_00 =>
            '┌',
        _ => unreachable!("confused about mask: {}", mask)
    })
}