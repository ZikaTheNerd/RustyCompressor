
pub mod constants {
    //used markers:
    pub const SOI: u8 = 0xD8;
    pub const DQT: u8 = 0xDB;
    pub const SOF: u8 = 0xC0;
    pub const DRI: u8 = 0xDD;
    pub const DHT: u8 = 0xC4;
    pub const SOS: u8 = 0xDA;
    pub const EOI: u8 = 0xD9;

    
    //unused markers:
    pub const APP0: u8 = 0xE0;
    pub const APP15: u8 = 0xEF;
    pub const COM: u8 = 0xFE;
    pub const JPG0: u8 = 0xF0;
    pub const JPG13: u8 = 0xFD;
    pub const DNL: u8 = 0xFD;
    pub const DHP: u8 = 0xFD;
    pub const EXP: u8 = 0xFD;
    pub const TEM: u8 = 0x01;
    pub const DAC: u8 = 0xCC;
    pub const RST0: u8 = 0xD0;
    pub const RST7: u8 = 0xD7;
    pub const SOF1: u8 = 0xC1;
    pub const SOF15: u8 = 0xCF;



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

