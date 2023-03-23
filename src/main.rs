use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::PathBuf
};
use clap::Parser;
use rustc_demangle::try_demangle;

pub mod args_structure;

fn main() {
    let args = args_structure::Cli::parse();
    if args.compression_level < 1 || args.compression_level > 21 {
        println!("error: supported compression levels [1, 21]");
        return;
    }

    let input_filename = &args.input_path;

    let output_filename = if args.output.is_some() {
        args.output.unwrap()
    } else {
        let mut path = PathBuf::from(&args.input_path);
        let mut temp = String::from("demangled_");
        temp += path.file_name().unwrap().to_str().unwrap();
        path.set_file_name(temp);
        path.to_str().unwrap().to_owned()
    };

    let input_file = OpenOptions::new()
        .read(true)
        .open(input_filename.clone());

    if input_file.is_err() {
        println!("error: opening input file: {}", input_file.err().unwrap());
        return;
    }
    let input_file = input_file.unwrap();

    let decoder = zstd::Decoder::new(input_file);
    if decoder.is_err() {
        println!("error: instantiating decoder: {}", decoder.err().unwrap());
        return;
    }
    let mut decoder = decoder.unwrap();

    let mut decompressed: Vec<u8> = Vec::new();
    let res = io::copy(&mut decoder, &mut decompressed);
    if res.is_err() {
        println!("error: feeding decoder: {}", res.err().unwrap());
        return;
    }

    let res = decompressed.iter().map(|c| *c as char).collect::<String>();
    let buffer_size = res.len();

    // using &str to avoid making a lot of Strings
    let mut lines: Vec<&str> = res.lines().collect::<Vec<&str>>();

    // look for lines that start with 's' and trying to demangle third word in that line
    // if it fails to demangle line is skipped
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

            // leaking memory to be able to modify line in place
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
    let output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(output_filename);
    if output_file.is_err() {
        println!("error: opening output file: {}", output_file.err().unwrap());
        return;
    }
    let input_file = output_file.unwrap();

    let encoder = zstd::Encoder::new(input_file, args.compression_level);
    if encoder.is_err() {
        println!("error: instantiating encoder: {}", encoder.err().unwrap());
        return;
    }
    let mut encoder = encoder.unwrap();
    let res = encoder.write(buff.as_slice());
    if res.is_err() {
        println!("error: feeding encoder: {}", res.err().unwrap());
        return;
    }

    if encoder.finish().is_err() {
        println!("error: finishing encoder: {}", res.err().unwrap());
        return;
    }
}
