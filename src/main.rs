use std::env;
use std::fs;

mod constants;
use constants::constants::*;

mod structs;
use structs::*;

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

fn read_appn(bytes: &Vec<u8>) -> usize
{
    let mut iterator = 2;
    let mut first: u8 = 0;
    let mut second: u8 = 0;
    loop 
    {
        if bytes.len() < iterator+4
        {
            panic!("APPN error!\n");
        }
        first = bytes[iterator];
        iterator+=1;
        second = bytes[iterator];
        iterator+=1;
        if first != 0xFF || second<APP0 || second>APP15
        {
            iterator-=2;
            break;
        }
        first = bytes[iterator];
        iterator+=1;
        second = bytes[iterator];
        iterator+=1;
        let length= (first as u16) << 8 | second as u16;
        iterator+=(length as usize) - 2;
    }
    iterator
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
        let c_id = bytes[i];
        i+=1;
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

fn print_data(data: JPEG)
{
    println!("QTables:\n");
    for i in 0..4
    {
        if data.qtables[i].set 
        {
            print!("id: {}\n",i);
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
        print!("\n");
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

}

fn read_JPEG(bytes: &Vec<u8>,data: &mut JPEG)
{
    let mut i = 0;
    while(bytes.len() > i+2)
    {
        let second = bytes[i+1];
        if second == SOI
        {
            read_header(&bytes);
            i = 2;
            println!("Succesfully read JPEG header!\n");
        }
        else if second>=APP0 && second<=APP15
        {
            i = read_appn(&bytes);
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
            println!("Next bytes are: {:x} and {:x} and i is {}\n", bytes[i],bytes[i+1],i);
            break;
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let argc = args.len();
    if argc != 2 {
        panic!("Invalid arguments!\n");
    }

    let image_name = &args[1];
    let bytes = fs::read(image_name).expect("Was not able to read the file.\n");
    
    let mut data = JPEG::new();

    read_JPEG(&bytes,&mut data);

    print_data(data);

}