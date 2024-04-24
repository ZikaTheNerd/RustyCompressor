use std::env;
use std::fs;

mod constants;
use constants::constants::*;

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

fn main() {
    let args : Vec<String> = env::args().collect();
    let argc = args.len();
    if argc != 2 {
        panic!("Usage: rcomp image\n");
    }

    let image_name = &args[1];
    let bytes = fs::read(image_name).expect("Was not able to read the file.\n");
    
    read_header(&bytes);

    println!("Succesfully read JPEG header!\n");

    let i = read_appn(&bytes);

    //citanje se nastavlja od mesta i (pocev od njega)

    println!("Succesfully read APPN section!\n");

    println!("Next bytes are {:x} {:x} and i is {}\n", bytes[i],bytes[i+1],i);
    
}