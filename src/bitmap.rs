
use crate::constants::*;
use crate::structs::*; 

use std::fs::File;
use std::io::Write;

pub fn blackBox(image: &JPEG) -> Vec<rgbMCU>
{
    Vec::new()
}

//little-endian writing
pub fn put4(bytes: &mut Vec<u8>,x:u32)
{
    let v = x.to_le_bytes();
    bytes.push(v[0]);
    bytes.push(v[1]);
    bytes.push(v[2]);
    bytes.push(v[3]);
}
pub fn put2(bytes: &mut Vec<u8>,x:u16)
{
    let v = x.to_le_bytes();
    bytes.push(v[0]);
    bytes.push(v[1]);
}

pub fn makeBitmap(data: &JPEG,MCUs: Vec<rgbMCU>,old_name: &String)
{
    let mut new_name = old_name.to_string();
    if let Some(index) = new_name.rfind('.')
    {
        new_name.split_off(index+1);
        
    }
    new_name += "bmp";

    print!("Making {new_name} from {old_name}....");

    let mut f = File::create(new_name).expect("Creating file error!\n");
    let mut bytes: Vec<u8> = Vec::new();
    
    //.....
    let h = data.sof_data.height;
    let w = data.sof_data.width;
    let mcuHeight = (h+7)/8;
    let mcuWidth = (w+7)/8;
    let paddingSize = w % 4;
    let size = 14+12 + h*w*3 + paddingSize*h;
    //first 14 bytes:
    bytes.push('B' as u8);
    bytes.push('M' as u8);
    put4(&mut bytes, size as u32);
    put4(&mut bytes, 0);
    put4(&mut bytes, 0x1A);
    //next 12 bytes:
    put4(&mut bytes, 12);
    put2(&mut bytes, w as u16);
    put2(&mut bytes, h as u16);
    put2(&mut bytes, 1);
    put2(&mut bytes, 24);
    //writing RGBs...
    for y in (0..h).rev()
    {
        let mcuRow = y/8;
        let pixRow = y%8;
        for x in 0..w
        {
            let mcuCol = x/8;
            let pixCol = x%8;
            let mcuIndex = mcuRow * mcuWidth + mcuCol;
            let pixIndex = pixRow * 8 + pixCol;
            bytes.push(MCUs[mcuIndex as usize].b[pixIndex as usize]);
            bytes.push(MCUs[mcuIndex as usize].g[pixIndex as usize]);
            bytes.push(MCUs[mcuIndex as usize].r[pixIndex as usize]);
        }
        for j in 0..paddingSize
        {
            bytes.push(0);
        }
    }

    f.write_all(&bytes).expect("Writing file error!\n");
    print!("Done.\n");
}
