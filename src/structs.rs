

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

pub struct JPEG
{
    pub qtables: [QTable; 4]
}

