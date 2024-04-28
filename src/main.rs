use std::env;

mod structs;
mod constants;

mod decoder;
use decoder::decode;

fn main()
{
    let args : Vec<String> = env::args().collect();
    let argc = args.len();
    if argc != 2 {
        panic!("Invalid arguments!\n");
    }

    let image_name = &args[1];

    decode(image_name);

}