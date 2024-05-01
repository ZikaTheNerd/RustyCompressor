use std::fs;

use crate::constants::*;
use crate::structs::*;  

fn read_header(bytes: &Vec<u8>) 
{
    if bytes.len()<2
    {
        panic!("Header error!\n");
    }
    let first = bytes[0];
    let second = bytes[1];
    if first != 0xFF || second != SOI
    {
        panic!("Header error!\n");
    }
}

fn read_APPN(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i = iterator;
    if bytes.len() < i + 4
    {
        panic!("Comment error!\n");
    }
    let mut first: u8 = bytes[i];
    i+=1;
    let mut second: u8 = bytes[i];
    i+=1;
    if first!=0xFF || second<APP0 || second>APP15
    {
        panic!("Comment error!\n");
    }
    first=bytes[i];
    i+=1;
    second=bytes[i];
    i+=1;
    let length= (first as u16) << 8 | second as u16;
    i+=length as usize -2;
    if bytes.len() < i
    {
        panic!("Comment error!\n");
    }

    i
}

fn read_TEM(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i = iterator;
    if bytes.len() < i + 2
    {
        panic!("TEM error!\n");
    }
    let first: u8 = bytes[i];
    i+=1;
    let second: u8 = bytes[i];
    i+=1;
    if first!=0xFF || second!=TEM
    {
        panic!("TEM error!\n");
    }
    
    i
}

fn read_comment(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i = iterator;
    if bytes.len() < i + 4
    {
        panic!("Comment error!\n");
    }
    let mut first: u8 = bytes[i];
    i+=1;
    let mut second: u8 = bytes[i];
    i+=1;
    if first!=0xFF || second!=COM
    {
        panic!("Comment error!\n");
    }
    first=bytes[i];
    i+=1;
    second=bytes[i];
    i+=1;
    let length= (first as u16) << 8 | second as u16;
    i+=length as usize -2;
    if bytes.len() < i
    {
        panic!("Comment error!\n");
    }

    i
}

fn read_q_tables(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i = iterator;
    if bytes.len() < i+6
        {
            panic!("QTable error!\n");
        }
    let mut first = bytes[i];
    i+=1;
    let mut second = bytes[i];
    i+=1;
    if first != 0xFF || second != DQT
    {
        panic!("QTable error!\n");
    }
    first=bytes[i];
    i+=1;
    second=bytes[i];
    i+=1;
    let length= (first as u16) << 8 | second as u16;
    if bytes.len() < i + (length as usize)
    {
        panic!("QTable error!\n");
    }
    let mut len=length as i32;
    len-=2;
    while len>0
    {
        let tableInfo=bytes[i];
        i+=1;
        len-=1;
        let tableID = tableInfo & 0x0F;
        let table_size = tableInfo >> 4;
        if tableID > 3
        {
            panic!("QTable error!\n");
        }
        data.qtables[tableID as usize].set=true;
        if table_size!=0
        {//16-bit qtable (skoro nikada se ne desava)
            /*let mut j=0;
            while j < 128{
                data.qtables[tableID as usize].table[zigzag[j]]=((bytes[i+j] as u16)<<8) + (bytes[i+j+1] as u16);
                j+=2;
            }
            len-=128;
            i+=128;
            qtable je niz tipa u8 pa bi trebalo omoguciti da bude i u16....*/
            panic!("16-bit QTable!\n");
        }
        else
        {//8-bit table
            for j in 0..64 {
                data.qtables[tableID as usize].table[zigzag[j]]=bytes[i+j];
            }
            len-=64;
            i+=64;
        }
    }
    if len != 0
    {
        panic!("QTable error!\n");
    }
    if bytes.len() < i+2
    {
        panic!("QTable error!\n");
    }
    if bytes[i+1] == DQT
    {
        i = read_q_tables(&bytes, i,data)
    }
    i
}

fn read_SOF_data(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i = iterator;
    if bytes.len() < i+6
    {
        panic!("SOF error!\n");
    }
    let mut first = bytes[i];
    i+=1;
    let mut second = bytes[i];
    i+=1;
    if first != 0xFF || second != SOF
    {
        if first == 0xFF && second!=0xC4 && second!=0xC8 && second!=0xCC && (second & 0x0F)==0xC0
        {
            panic!("Not a baseline jpeg!\n")
        }
        else {
            panic!("SOF error!\n");
        }
    }
    first=bytes[i];
    i+=1;
    second=bytes[i];
    i+=1;
    let length= (first as u16) << 8 | second as u16;
    if bytes.len() < i + (length as usize)
    {
        panic!("SOF error!\n");
    }
    let precission=bytes[i]; //not used
    if precission != 8
    {
        panic!("SOF error!\n");
    }
    i+=1;
    // width 2bytes
    // heigth 2bytes
    first=bytes[i];
    i+=1;
    second=bytes[i];
    i+=1;
    data.sof_data.height = (first as u16) << 8 | second as u16;
    first=bytes[i];
    i+=1;
    second=bytes[i];
    i+=1;
    data.sof_data.width = (first as u16) << 8 | second as u16;
    if data.sof_data.height==0 || data.sof_data.width==0
    {
        panic!("SOF error!\n");
    }
    // num of comp 1byte 1-greyscale, 3-ycbcr
    let n = bytes[i];
    data.sof_data.numComp = n;
    i+=1;
    if n>3 || n==0
    {
        panic!("SOF error!\n");
    }
    //component: ID,sampling factor,qtable id  (1,1,1 bytes)
    for j in 0..n
    {
        let mut c_id = bytes[i];
        i+=1;
        if c_id == 0 //ne sme ali pokusavamo da pokrijemo
        {
            data.sof_data.zero_based = true;
        }
        if data.sof_data.zero_based == true // popravljamo
        {
            c_id+=1;
        }
        if c_id==0 || c_id>3
        {
            panic!("SOF error!\n");
        }
        data.sof_data.Components[c_id as usize -1].set = true;
        let sf = bytes[i];
        i+=1;
        data.sof_data.Components[c_id as usize -1].hor_sampling_factor = sf >> 4;
        data.sof_data.Components[c_id as usize -1].ver_sampling_factor = sf & 0x0F;
        data.sof_data.Components[c_id as usize -1].qtable_id = bytes[i];
        i+=1;
    }
    if length != 8 + 3 * (data.sof_data.numComp as u16)  // dodatna provera....
    {
        panic!("SOF error!\n");
    }
    i
}

fn read_DRI_data(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i=iterator;
    if bytes.len()<6
    {
        panic!("DRI error!\n");
    }
    if bytes[i]!=0xFF || bytes[i+1]!=DRI
    {
        panic!("DRI error!\n");
    }
    i+=2;
    let mut first=bytes[i];
    i+=1;
    let mut second=bytes[i];
    i+=1;
    let length = (first as u16) << 8 | second as u16;
    if length != 4
    {
        panic!("DRI error!\n");
    }
    first=bytes[i];
    i+=1;
    second=bytes[i];
    i+=1;
    data.restart_interval = (first as u16) << 8 | second as u16;

    i
}

fn read_Huffman_tables(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i=iterator;
    if bytes.len()<4
    {
        panic!("Huffman tables error!\n");
    }
    if bytes[i]!=0xFF || bytes[i+1]!=DHT
    {
        panic!("Huffman tables error!\n");
    }
    i+=2;
    let mut first=bytes[i];
    i+=1;
    let mut second=bytes[i];
    i+=1;
    let length = (first as u16) << 8 | second as u16;
    if bytes.len() < i + length as usize
    {
        panic!("Huffman tables error!\n");
    }
    let mut len = length as i32;
    len-=2;
    while len > 0
    {
        let tableInfo = bytes[i];
        i+=1;
        len-=1;
        let tableID = (tableInfo & 0x0F) as usize;
        let ifAC = tableInfo >> 4;  // 1 if ac, 0 if dc

        if tableID > 3
        {
            panic!("Huffman tables error!\n");
        }
        let mut ht = Huffman_table::new();
        ht.set = true;
        ht.offsets[0] = 0;

        let mut total: u8 = 0;
        for j in 1..17
        {
            total+=bytes[i];
            i+=1;
            ht.offsets[j] = total;
            if total > 162
            {
                panic!("Huffman tables error!\n");
            }
        }
        len-=16;
        for j in 0..total as usize
        {
            ht.symbols[j]=bytes[i];
            i+=1;
        }
        len-=total as i32;

        if ifAC == 0
        {
            data.huff_DC_tables[tableID] = ht;
        }
        else
        {
            data.huff_AC_tables[tableID] = ht;
        }
    }

    if len!=0
    {
        panic!("Huffman tables error!\n");
    }
   
    i
}

fn read_SOS(bytes: &Vec<u8>,iterator: usize,data: &mut JPEG) -> usize
{
    let mut i=iterator;
    if bytes.len() < i+4
    {
        panic!("SOS error!\n");
    }
    if data.sof_data.numComp == 0
    {
        panic!("SOS error!\n");
    }
    if bytes[i]!=0xFF || bytes[i+1]!=SOS
    {
        panic!("SOS error!\n");
    }
    i+=2;
    let mut first=bytes[i];
    i+=1;
    let mut second=bytes[i];
    i+=1;
    let length = (first as u16) << 8 | second as u16;
    if bytes.len() < i + length as usize
    {
        panic!("SOS error!\n");
    }
    //......
    let numC = bytes[i];
    i+=1;
    for j in 0..numC as usize
    {
        let mut compID = bytes[i];
        i+=1;
        if data.sof_data.zero_based == true //opet ako je jpeg lose napravljen...
        {
            compID+=1;
        }
        if compID > data.sof_data.numComp
        {
            panic!("SOS error!\n");
        }
        let mut component = &mut data.sof_data.Components[compID as usize-1]; //radi preglednosti
        if component.used == true
        {
            panic!("SOS error!\n");
        }
        component.used = true;
        component.huff_dc_id = bytes[i] >> 4;
        component.huff_ac_id = bytes[i] & 0x0F;
        i+=1;
        if component.huff_ac_id > 3 || component.huff_dc_id > 3
        {
            panic!("SOS error!\n");
        }
    }
    data.sof_data.sos = bytes[i];
    i+=1;
    data.sof_data.eos = bytes[i];
    i+=1;
    data.sof_data.sa_high = bytes[i] >> 4;
    data.sof_data.sa_low = bytes[i] & 0x0F;
    i+=1;
    if data.sof_data.sos !=0 || data.sof_data.eos !=63 // samo standradno radimo
    {
        panic!("SOS error!\n");
    }
    if data.sof_data.sa_high !=0 || data.sof_data.sa_low !=0 // samo standardno radimo
    {
        panic!("SOS error!\n");
    }
    if length != 6 + 2 * numC as u16
    {
        panic!("SOS error!\n");
    }
   
    i
}


fn print_data(data: &JPEG)
{
    println!("QTables:");
    for i in 0..4
    {
        if data.qtables[i].set 
        {
            print!("id: {}",i);
            for j in 0..64
            {
                if j%8==0
                {
                    print!("\n");
                }
                print!("{} ",data.qtables[i].table[j]);
            }
            print!("\n");
        }
    }
    
    println!("SOF data:");
    println!("height: {}",data.sof_data.height);
    println!("width: {}",data.sof_data.width);
    for j in 0..data.sof_data.numComp as usize
    {
        print!("Component: {}\n",j+1);
        print!("Hor. samp. factor: {}\n",data.sof_data.Components[j].hor_sampling_factor);
        print!("Ver. samp. factor: {}\n",data.sof_data.Components[j].ver_sampling_factor);
        print!("QTable ID: {}\n",data.sof_data.Components[j].qtable_id);
    }

    println!("DRI:");
    println!("restart_interval: {}\n",data.restart_interval);

    println!("Huffman DC tables:");
    for j in 0..4
    {
        if data.huff_DC_tables[j].set == true
        {
            println!("DC table:{}\nSymbols:",j);
            for k in 1..17
            {
                print!("{}: ",k);
                for n in data.huff_DC_tables[j].offsets[k-1]..data.huff_DC_tables[j].offsets[k]
                {
                    print!("{:x} ",data.huff_DC_tables[j].symbols[n as usize]);
                }
                print!("\n");
            }
        }
    }

    println!("Huffman AC tables:");
    for j in 0..4
    {
        if data.huff_AC_tables[j].set == true
        {
            println!("AC table:{}\nSymbols:",j);
            for k in 1..17
            {
                print!("{}: ",k);
                for n in data.huff_AC_tables[j].offsets[k-1]..data.huff_AC_tables[j].offsets[k]
                {
                    print!("{:x} ",data.huff_AC_tables[j].symbols[n as usize]);
                }
                print!("\n");
            }
        }
    }
    //...
    println!("SOS data:");
    println!("SOS: {}",data.sof_data.sos);
    println!("EOS: {}",data.sof_data.eos);
    println!("SA high: {}",data.sof_data.sa_high);
    println!("SA low: {}",data.sof_data.sa_low);
    println!("Color components:");
    for j in 0..data.sof_data.numComp as usize
    {
        println!("Component ID: {}",j+1);
        println!("DC table ID: {}",data.sof_data.Components[j].huff_dc_id);
        println!("AC table ID: {}",data.sof_data.Components[j].huff_ac_id);
    }

    println!("Huffman bitstream size: {}",data.huff_data.len());

    println!("Restart interval: {}",data.restart_interval);

}

fn read_JPEG(bytes: &Vec<u8>,data: &mut JPEG)
{
    let mut i = 0;
    read_header(&bytes);
    i = 2;
    println!("Succesfully read header!\n");

    while(bytes.len() > i+2)
    {
        let first = bytes[i];
        let second = bytes[i+1];
        if second>=APP0 && second<=APP15 //unused markers
        {
            i = read_APPN(&bytes,i,data);
            println!("Succesfully read APPN section!\n");
        }
        else if second == DQT
        {
            i = read_q_tables(&bytes,i,data);
            println!("Succesfully read Quantization tables!\n");
        }
        else if second == SOF
        {
            i = read_SOF_data(&bytes,i,data);
            println!("Succesfully read SOF data!\n");
        }
        else if second == DRI
        {
            i = read_DRI_data(&bytes,i,data);
        }
        else if second == DHT
        {
            i = read_Huffman_tables(&bytes,i,data);
            println!("Succesfully read Huffman tables!\n");
        }
        else if second == SOS
        {
            i = read_SOS(&bytes,i,data);
            println!("Succesfully read Start of scan!\n");
            break;
        }
        else if second == COM //unused marker
        {
            i = read_comment(&bytes,i,data);
            println!("Succesfully read a comment!\n");
        }
        else if (second >= JPG0 && second <= JPG13) ||
        second == DNL || second == DHP || second == EXP //unused markers....
        {
            i = read_comment(&bytes,i,data);
            println!("Succesfully read an unused marker!\n");
        }
        else if second == TEM //unused but doesn't have size
        {
            i = read_TEM(&bytes,i,data);
            println!("Succesfully read an unused marker!\n");
        }
        else if first == 0xFF && second == 0xFF //allowed to skip
        {
            if bytes.len() > i+1
            {
                second == bytes[i];
                i+=1;
            }
            else 
            {
                panic!("0xFF at the end of file!\n")    
            }
        }
        else if second == SOI || second == EOI //forbidden or unsupported markers
        || second == DAC || (second >= SOF1 && second <= SOF15)
        || (second >= RST0 && second <= RST7)
        {
            panic!("Forbidden or unsupported marker!\n")
        }
        else 
        {
            panic!("Unknown marker!\n");
        }   
    }
    //end of loop
        //only huffman bitstream left...
        let mut first = bytes[i];
        let mut second = first;
        i+=1;
        loop
        {
            if bytes.len() < i
            {
            panic!("File ended prematurely!\n");
            }
            first = second;
            second = bytes[i];
            i+=1;
            // first and second ready....
            if first == 0xFF //reading marker...
            {
                if second == EOI //end of image
                {
                    break;
                }
                else if second == 0x00 //FF included in bitstream and 00 ignored
                {
                    data.huff_data.push(first);
                    if bytes.len() < i
                    {
                    panic!("File ended prematurely!\n");
                    }
                    second = bytes[i];
                    i+=1;
                }
                else if second>=RST0 && second<=RST7 //ignored
                {
                    if bytes.len() < i
                    {
                    panic!("File ended prematurely!\n");
                    }
                    second = bytes[i];
                    i+=1;
                }
                else if second == 0xFF //ignored
                {
                    continue;
                }
                else 
                {
                    panic!("Bitstream error!\n");
                }
            }
            else //not reading a marker... 
            {
                data.huff_data.push(first);
            }
        }
        //checking some errors....
        if data.sof_data.numComp!=1 && data.sof_data.numComp!=3
        {
            panic!("Number of components not supported!\n");
        }
        for j in 0..data.sof_data.numComp as usize
        {
            if data.qtables[data.sof_data.Components[j].qtable_id as usize].set == false
            {
                panic!("QTable not set!\n");
            }
            if data.huff_AC_tables[data.sof_data.Components[j].huff_ac_id as usize].set == false
            {
                panic!("Huffman AC table not set!\n");
            }
            if data.huff_DC_tables[data.sof_data.Components[j].huff_dc_id as usize].set == false
            {
                panic!("Huffman DC table not set!\n");
            }
        }
}

pub fn decode(image_name: &String) -> JPEG {
    
    let bytes = fs::read(image_name).expect("Was not able to read the file.\n");
    
    let mut data = JPEG::new();

    read_JPEG(&bytes,&mut data);

    print_data(&data);


    data
}