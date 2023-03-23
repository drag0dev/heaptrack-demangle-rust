use std::{env::args, fs::OpenOptions, io};
use rustc_demangle::try_demangle;

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
    let mut lines: Vec<&str> = res.lines().collect::<Vec<&str>>();

    for (index, line) in res.lines().enumerate() {
        if line.starts_with('s') && line.matches(" ").count() > 1 {
            let mut new_line = line.split(" ");
            let mangled_name = new_line.nth(2).unwrap();
            let unmangled_name = try_demangle(mangled_name);
            if unmangled_name.is_err() { continue; }
            let unmangled_name = unmangled_name.unwrap();
            let new_line = new_line.collect::<String>();

            let new_line = new_line + &unmangled_name.to_string();
            lines[index] = Box::leak(new_line.into_boxed_str());
        }
    }
}
