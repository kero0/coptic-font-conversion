use clap::{Parser, ValueHint};
use conversions::*;
use data::*;
use rustyline::error::ReadlineError;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
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

fn handle_file(
    filename: PathBuf,
    converter: Box<dyn Fn(String) -> String>,
    mut writer: BufWriter<Box<dyn Write>>,
    line_buffered: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    for line in BufReader::new(File::open(filename)?).lines() {
        // read and convert each line
        let line = converter(line?);
        // write the converted line to the output
        writeln!(writer, "{}", line)?;
        // flush the output if we're in interactive mode
        if line_buffered {
            writer.flush()?;
        }
    }

    writer.flush()?;

    Ok(())
}

fn handle_readline(
    converter: Box<dyn Fn(String) -> String>,
    mut writer: BufWriter<Box<dyn Write>>,
    line_buffered: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = rustyline::Editor::<()>::new()?;
    loop {
        match rl.readline("↦ ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let line = converter(line);
                writeln!(writer, "{}", line)?;
                if line_buffered {
                    writer.flush()?;
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                writer.flush()?;
                return Ok(());
            }
            Err(err) => Err(err)?,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();

    let out_file: Box<dyn Write> = match args.out_file {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(stdout()),
    };
    let writer = BufWriter::new(out_file);

    let converter = mk_converter(args.conversion, args.abbreviations);

    if let Some(filename) = args.in_file {
        handle_file(filename, converter, writer, args.line_buffered)?;
    } else {
        handle_readline(converter, writer, args.line_buffered)?
    }

    Ok(())
}
