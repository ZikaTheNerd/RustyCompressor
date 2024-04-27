

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
    pub set: bool
}

impl Comp
{
    pub fn new() -> Self {
        Comp {
            hor_sampling_factor: 0,
            ver_sampling_factor: 0,
            qtable_id: 0,
            set: false
        }
    }
}

pub struct SOFdata
{
    pub height: u16,
    pub width: u16,
    pub numComp: u8,
    pub Components: [Comp; 3]
}

impl SOFdata
{
    pub fn new() -> Self {
        SOFdata {
            height: 0,
            width: 0,
            numComp: 0,
            Components: [Comp::new(),Comp::new(),Comp::new()]
        }
    }
}

pub struct JPEG
{
    pub qtables: [QTable; 4],
    pub sof_data: SOFdata,
}

impl JPEG
{
    pub fn new() -> Self {
        JPEG {
            qtables: [QTable::new(),QTable::new(),QTable::new(),QTable::new()],
            sof_data: SOFdata::new()
        }
    }
}
