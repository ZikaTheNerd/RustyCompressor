use std::env;

mod structs;
mod constants;
mod bitmap;
mod decoder;
use bitmap::*;
use decoder::decode;


fn main()
{
    let args : Vec<String> = env::args().collect();
    let argc = args.len();
    if argc != 2 {
        panic!("Invalid arguments!\n");
    }

    let image_name = &args[1];

    let data = decode(image_name);

    //TODO: finish decoding....
    let MCUs = blackBox(&data);

    makeBitmap(&data,MCUs,image_name);

}