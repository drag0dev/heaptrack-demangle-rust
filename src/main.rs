use std::{fs::OpenOptions, io::{self, Write}};
use rustc_demangle::try_demangle;

fn main() {
    let mut args = args();
    if args.len() < 2 {
        println!("missing filename");
        return;
    }
    let file_name = args.nth(1).unwrap();
    let input_file = OpenOptions::new()
        .read(true)
        .open(file_name.clone());

    if input_file.is_err() {
        println!("error opening input file: {}", input_file.err().unwrap());
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
    let buffer_size = res.len();
    let mut lines: Vec<&str> = res.lines().collect::<Vec<&str>>();

    for (index, line) in res.lines().enumerate() {
        if line.starts_with('s') && line.matches(" ").count() > 1 {
            let line_parts = line.split(" ").collect::<Vec<&str>>();

            let mangled_name = line_parts[2];
            let unmangled_name = try_demangle(mangled_name);
            if unmangled_name.is_err() { continue; }
            let unmangled_name = unmangled_name.unwrap();

            let mut new_line = String::new();
            new_line += line_parts[0];
            new_line += " ";
            new_line += line_parts[1];
            new_line += " ";
            new_line += &unmangled_name.to_string();

            lines[index] = Box::leak(new_line.into_boxed_str());
        }
    }

    // construct a buffer
    let mut buff: Vec<u8> = Vec::with_capacity(buffer_size);
    for line in lines {
        for b in line.bytes() {
            buff.push(b);
        }
        buff.push(b'\n');
    }
    buff.pop();

    // open the same file and truncate it
    let input_file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(file_name);
    if input_file.is_err() {
        println!("error opening output file: {}", input_file.err().unwrap());
        return;
    }
    let input_file = input_file.unwrap();

    let encoder = zstd::Encoder::new(input_file, 3);
    if encoder.is_err() {
        println!("error feeding encoder: {}", encoder.err().unwrap());
        return;
    }
    let mut encoder = encoder.unwrap();
    let res = encoder.write(buff.as_slice());
    if res.is_err() {
        println!("error feeding encoder: {}", res.err().unwrap());
        return;
    }

    let res = encoder.finish();
    if res.is_err() {
        println!("error finishing: {}", res.err().unwrap());
        return;
    }
}
