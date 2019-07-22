use DigitScan::*;

#[rustfmt::skip]
pub enum DigitScan {
    Unknown,

    // Row 1
    R1_02356789, R1_14,

    // Row 2
    R2_0, R2_1, R2_23, R2_4, R2_56, R2_7, R2_89,

    // Row 3
    R3_0, R3_1, R3_2, R3_3, R3_4, R3_5, R3_6, R3_7, R3_8, R3_9,

    // Row 4
    R4_0, R4_1, R4_2, R4_3, R4_4, R4_5, R4_6, R4_7, R4_8, R4_9,
}

impl DigitScan {
    pub fn step(&mut self, row: &[u8]) {
        *self = match (&self, row) {
            (R1_02356789, b"| |") => R2_0,
            (R1_02356789, b" _|") => R2_23,
            (R1_02356789, b"|_ ") => R2_56,
            (R1_02356789, b"  |") => R2_7,
            (R1_02356789, b"|_|") => R2_89,
            (R1_14, b"  |") => R2_1,
            (R1_14, b"|_|") => R2_4,
            (R2_0, b"|_|") => R3_0,
            (R2_1, b"  |") => R3_1,
            (R2_23, b"|_ ") => R3_2,
            (R2_23, b" _|") => R3_3,
            (R2_4, b"  |") => R3_4,
            (R2_56, b" _|") => R3_5,
            (R2_56, b"|_|") => R3_6,
            (R2_7, b"  |") => R3_7,
            (R2_89, b"|_|") => R3_8,
            (R2_89, b" _|") => R3_9,
            (R3_0, b"   ") => R4_0,
            (R3_1, b"   ") => R4_1,
            (R3_2, b"   ") => R4_2,
            (R3_3, b"   ") => R4_3,
            (R3_4, b"   ") => R4_4,
            (R3_5, b"   ") => R4_5,
            (R3_6, b"   ") => R4_6,
            (R3_7, b"   ") => R4_7,
            (R3_8, b"   ") => R4_8,
            (R3_9, b"   ") => R4_9,
            _ => Unknown,
        };
    }
}

impl From<&[u8]> for DigitScan {
    fn from(row: &[u8]) -> Self {
        match row {
            b" _ " => R1_02356789,
            b"   " => R1_14,
            _ => Unknown,
        }
    }
}

impl From<&DigitScan> for char {
    fn from(scan: &DigitScan) -> Self {
        match scan {
            R4_0 => '0',
            R4_1 => '1',
            R4_2 => '2',
            R4_3 => '3',
            R4_4 => '4',
            R4_5 => '5',
            R4_6 => '6',
            R4_7 => '7',
            R4_8 => '8',
            R4_9 => '9',
            _ => '?',
        }
    }
}
