use clap::Parser;
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command[about = "script that demangles rust symbols in a heaptrack capture"]]
pub struct Cli {
    #[arg(value_name = "path to the capture")]
    pub input_path: OsString,

    #[arg(value_name = "output filename (default input path starting with demangled_)",
        short = 'o',
        long
    )]
    pub output: Option<String>,

    #[arg(value_name = "zstd compression level of the output file",
        short = 'l',
        long = "level",
        default_value_t = 3,
    )]
    pub compression_level: i32,
}
