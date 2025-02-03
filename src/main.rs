use std::fs::File;
use std::io;
use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg()]
    filename: Option<String>,
}

fn parse_vm(file: &mut impl Read) -> () {
    let bytes: String = file.bytes().into_iter()
        .map(|x| x.expect("faulty byte read") as char)
        .collect();

    println!("{}", bytes);
}

fn main() {
    let args = Args::parse();
    let mut fptr: Box<dyn Read> = if let Some(fname) = args.filename {
        Box::new(File::open(fname).expect("unable to open file"))
    } else {
        Box::new(io::stdin())
    };

    parse_vm(&mut fptr)
}
