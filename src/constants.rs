
pub mod constants {
    pub const SOI: u8 = 0xD8;
    pub const DQT: u8 = 0xDB;

    //APPN macros
    pub const APP0: u8 = 0xE0;
    pub const APP1: u8 = 0xE1;
    pub const APP2: u8 = 0xE2;
    pub const APP3: u8 = 0xE3;
    pub const APP4: u8 = 0xE4;
    pub const APP5: u8 = 0xE5;
    pub const APP6: u8 = 0xE6;
    pub const APP7: u8 = 0xE7;
    pub const APP8: u8 = 0xE8;
    pub const APP9: u8 = 0xE9;
    pub const APP10: u8 = 0xEA;
    pub const APP11: u8 = 0xEB;
    pub const APP12: u8 = 0xEC;
    pub const APP13: u8 = 0xED;
    pub const APP14: u8 = 0xEE;
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

