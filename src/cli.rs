use clap::{Parser, ValueHint, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// File to read text from, otherwise stdin
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub in_file: Option<std::path::PathBuf>,

    /// File to write text to, otherwise stdout
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub out_file: Option<std::path::PathBuf>,

    /// Conversion direction, to or from Coptic Standard and Unicode
    #[arg(
        short,
        long,
        value_enum,
    )]
    pub conversion: ConversionType,

    /// Line buffered output
    #[arg(short, long, default_value = "true")]
    pub line_buffered: bool,

    /// Handling of coptic abbreviations
    #[arg(
        short,
        long,
        value_enum,
    )]
    pub abbreviations: AbbreviationHandling,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ConversionType {
    CopticStandardToUnicode,
    UnicodeToCopticStandard,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum AbbreviationHandling {
    Preserve,
    Unabbreviate,
    Abbreviate,
}
