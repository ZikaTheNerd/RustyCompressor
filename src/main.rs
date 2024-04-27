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
        {//16-bit qtable (almost never happens)
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

fn print_data(data: JPEG)
{
    println!("QTables:\n");
    for i in 0..4
    {
        if data.qtables[i].set 
        {
            for j in 0..64
            {
                if j%8==0
                {
                    print!("\n");
                }
                print!("{} ",data.qtables[i].table[j]);
            }
        }
        print!("\n\n");
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
    
    let mut data = JPEG{
        qtables: [QTable::new(),QTable::new(),QTable::new(),QTable::new()],
    };

    read_header(&bytes);

    println!("Succesfully read JPEG header!\n");

    let mut i = read_appn(&bytes);

    //citanje se nastavlja od mesta i (pocev od njega)

    println!("Succesfully read APPN section!\n");

    i = read_q_tables(&bytes,i,&mut data);

    println!("Succesfully read Quantization tables!\n");

    println!("Next bytes are: {:x} and {:x} and i is {}\n", bytes[i],bytes[i+1],i);
    
    print_data(data);

}