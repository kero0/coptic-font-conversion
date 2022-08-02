use clap::{Parser, ValueHint};
use conversions::*;
use data::*;
use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

mod conversions;
mod data;

#[derive(Parser)]
pub struct Cli {
    /// File to read text from, otherwise stdin
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    in_file: Option<std::path::PathBuf>,

    /// File to write text to, otherwise stdout
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath)]
    out_file: Option<std::path::PathBuf>,

    /// Conversion direction, to or from Coptic Standard and Unicode
    #[clap(
        arg_enum,
        short,
        long,
        takes_value = false,
        default_value = "coptic-standard-to-unicode"
    )]
    conversion: ConversionType,

    /// Line buffered output
    #[clap(short, long, takes_value = false, default_value = "true")]
    line_buffered: bool,

    /// Handling of coptic abbreviations
    #[clap(arg_enum, short, long, takes_value = false, default_value = "preserve")]
    abbreviations: AbbreviationHandling,
}

fn main() {
    let args: Cli = Cli::parse();

    let reader: Box<dyn BufRead> = match args.in_file {
        // use input file arg or default to stdin
        Some(filename) => Box::new(BufReader::new(
            File::open(filename).expect("Could not open input file"),
        )),
        None => Box::new(BufReader::new(stdin().lock())),
    };

    let mut writer: BufWriter<Box<dyn Write>> = BufWriter::new(match args.out_file {
        // use output file arg or default to stdout
        Some(path) => Box::new(File::create(path).expect("Could not open output file")),
        None => Box::new(stdout().lock()),
    });

    // convert coptic to unicode or unicode to coptic
    let converter = mk_converter(args.conversion, args.abbreviations);

    // get an iterator over the lines of the input
    reader.lines().for_each(|line| {
        // read and convert each line
        let line = converter(line.expect("Error reading line"));
        // write the converted line to the output
        writeln!(writer, "{}", line).expect("Error writing line");
        // flush the output if we're in interactive mode
        if args.line_buffered {
            writer.flush().expect("Error flushing");
        }
    });

    // flush the output on exit
    writer.flush().expect("Error flushing");
}
