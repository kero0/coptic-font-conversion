use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use clap::{Parser, ValueHint};
mod conversions;
pub use conversions::{c2e, e2c};

#[derive(Parser)]
pub struct Cli {
    /// File to read text from, otherwise stdin
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    in_file: Option<std::path::PathBuf>,

    /// File to write text to, otherwise stdout
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    out_file: Option<std::path::PathBuf>,

    /// Convert coptic unicode to coptic standard characters instead of the default unicode to cs
    #[clap(short, long, takes_value = false)]
    reverse: bool,

    /// Line buffered output
    #[clap(short, long, takes_value = false, default_value = "true")]
    line_buffered: bool,
}

fn main() {
    let args: Cli = Cli::parse();

    let mut reader: Box<dyn BufRead> = match args.in_file {
        // use input file arg or default to stdin
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
        None => Box::new(BufReader::new(std::io::stdin().lock())),
    };

    let interactive = args.line_buffered;
    let mut writer: BufWriter<Box<dyn std::io::Write>> = match args.out_file {
        // use output file arg or default to stdout
        Some(path) => {
            let file = File::create(path).unwrap();
            BufWriter::new(Box::new(file))
        }
        None => {
            let stdout = std::io::stdout();
            BufWriter::new(Box::new(stdout.lock()))
        }
    };

    // convert coptic to unicode or unicode to coptic
    let conv = if args.reverse { c2e } else { e2c };

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                match write!(writer, "{}", conv(line)) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error writing to output: {}", e);
                        break;
                    }
                }
                if interactive {
                    match writer.flush() {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("error: {}", e);
                std::process::exit(1);
            }
        }
    }

    match writer.flush() {
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
        Ok(_) => std::process::exit(0),
    }
}
