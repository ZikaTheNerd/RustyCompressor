pub struct QTable
{
    pub table: [u8; 64],
    pub set: bool,
}

impl QTable
{
    pub fn new() -> Self {
        QTable {
            table: [1; 64],
            set: false,
        }
    }
}

pub struct Comp
{
    pub hor_sampling_factor: u8,
    pub ver_sampling_factor: u8,
    pub qtable_id: u8,
    pub huff_dc_id: u8,
    pub huff_ac_id: u8,
    pub set: bool,
    pub used: bool
}

impl Comp
{
    pub fn new() -> Self {
        Comp {
            hor_sampling_factor: 0,
            ver_sampling_factor: 0,
            qtable_id: 0,
            huff_dc_id: 0,
            huff_ac_id: 0,
            set: false,
            used: false
        }
    }
}

pub struct SOFdata
{
    pub height: u16,
    pub width: u16,
    pub numComp: u8,
    pub Components: [Comp; 3],
    pub zero_based: bool,
    pub sos: u8,
    pub eos: u8,
    pub sa_high: u8,
    pub sa_low: u8,
}

impl SOFdata
{
    pub fn new() -> Self {
        SOFdata {
            height: 0,
            width: 0,
            numComp: 0,
            Components: [Comp::new(),Comp::new(),Comp::new()],
            zero_based: false, //mora false ali pokrivamo i pogresne jpeg fajlove
            sos: 0,
            eos: 63,
            sa_high: 0,
            sa_low: 0,
        }
    }
}

pub struct Huffman_table
{
    pub offsets: [u8; 17], // od kog do kog mesta su simboli odgovarajuce duzine
    pub symbols: [u8; 162], 
    pub set: bool,
}

impl Huffman_table
{
    pub fn new() -> Self
    {
        Huffman_table
        {
            offsets: [0; 17],
            symbols: [0; 162],
            set: false
        }
    }
}

pub struct JPEG
{
    pub qtables: [QTable; 4],
    pub sof_data: SOFdata,
    pub restart_interval: u16,
    pub huff_DC_tables: [Huffman_table; 4],
    pub huff_AC_tables: [Huffman_table; 4],
    pub huff_data: Vec<u8>,
}

impl JPEG
{
    pub fn new() -> Self {
        JPEG {
            qtables: [QTable::new(),QTable::new(),QTable::new(),QTable::new()],
            sof_data: SOFdata::new(),
            restart_interval: 0,
            huff_DC_tables: [Huffman_table::new(),Huffman_table::new(),Huffman_table::new(),Huffman_table::new()],
            huff_AC_tables: [Huffman_table::new(),Huffman_table::new(),Huffman_table::new(),Huffman_table::new()],
            huff_data: Vec::new(),
        }
    }
}

pub struct rgbMCU
{
    pub r:[u8;64],
    pub g:[u8;64],
    pub b:[u8;64],
}


