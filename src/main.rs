use std::{env::args, fs::OpenOptions, io};

fn main() {
    let mut args = args();
    if args.len() < 2 {
        println!("missing filename");
        return;
    }

    let input_file = OpenOptions::new()
        .read(true)
        .open(args.nth(1).unwrap());

    if input_file.is_err() {
        println!("error opening file: {}", input_file.err().unwrap());
        return;
    }
    let input_file = input_file.unwrap();

    let decoder = zstd::Decoder::new(input_file);
    if decoder.is_err() {
        println!("error feeding decoder: {}", decoder.err().unwrap());
        return;
    }
    let mut decoder = decoder.unwrap();

    let mut decompressed: Vec<u8> = Vec::new();
    let res = io::copy(&mut decoder, &mut decompressed);
    if res.is_err() {
        println!("error decompressing: {}", res.err().unwrap());
        return;
    }

    let res = decompressed.iter().map(|c| *c as char).collect::<String>();
    println!("{res}");
}
