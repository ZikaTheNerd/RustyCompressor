
pub mod constants {
    pub const SOI: u8 = 0xD8;
    pub const DQT: u8 = 0xDB;
    pub const SOF: u8 = 0xC0;
    pub const DRI: u8 = 0xDD;
    pub const DHT: u8 = 0xC4;

    
    pub const APP0: u8 = 0xE0;
    pub const APP15: u8 = 0xEF;

    pub const zigzag: [usize;64] = [
        0,   1,  8, 16,  9,  2,  3, 10,
        17, 24, 32, 25, 18, 11,  4,  5,
        12, 19, 26, 33, 40, 48, 41, 34,
        27, 20, 13,  6,  7, 14, 21, 28,
        35, 42, 49, 56, 57, 50, 43, 36,
        29, 22, 15, 23, 30, 37, 44, 51,
        58, 59, 52, 45, 38, 31, 39, 46,
        53, 60, 61, 54, 47, 55, 62, 63
    ];

}

