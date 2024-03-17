use std::env;
use std::fs;

fn print_bytes(bytes : &[u8]) {
    let n = bytes.len();
    for i in 0..n {
        if i % 8 == 0 {
            print!("\n{:#x}, ", bytes[i]);
        }
        else {
            print!("{:#x}, ", bytes[i]);
        }
    }
    println!();
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let argc = args.len();
    if argc != 2 {
        panic!("Usage: rcomp image");
    }

    let image_name = &args[1];
    let image_bytes = fs::read(image_name).expect("Was not able to read the file.");
    
    if image_name.ends_with(".png") {
        println!("This is a png file!");
        println!("Signature is: ");
        print_bytes(&image_bytes[0..8]);
    }

    else {
        println!("Unknown file!");
        println!("Printing bytes...");
        print_bytes(&image_bytes);
    }

}
