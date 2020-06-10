use prettytable::{Cell, Row, Table};
use regex::{self, Regex};
use std::fs;
use structopt::clap::arg_enum;
use rayon::prelude::*;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::{clap::AppSettings, StructOpt};


fn parse_regex_str(src: &str) -> Result<Regex, regex::Error> {
    Regex::new(src)
}

fn parse_output_format(src: &str) -> Result<OutputFormat, String> {
    match src.to_lowercase().as_ref() {
        "csv" => Ok(OutputFormat::Csv),
        "ascii" => Ok(OutputFormat::Ascii),
        _ => Err(String::from("Outputformat does not exists!")),
    }
}


arg_enum! {
    #[derive(Debug)]
    enum OutputFormat {
	Csv,
	Ascii,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(setting = AppSettings::ColoredHelp, setting = AppSettings::NextLineHelp, author)]
struct Cli {
    #[structopt(parse(try_from_str = parse_regex_str))]
    regex: Regex,

    #[structopt(short = "f", long = "file")]
    file: Option<PathBuf>,

    /// Table output format. Possible formats: [csv, ascii]
    #[structopt(short = "o", long = "output", default_value = "ascii", parse(try_from_str = parse_output_format))]
    output_format: OutputFormat,
}

fn grab_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn column_headers(re: &Regex) -> Vec<String> {
    re.capture_names()
        .into_iter()
        .flatten()
        .map(|s| s.to_string())
        .collect()
}


fn get_named<'a>(cap: &'a regex::Captures, label: &str) -> &'a str {
    &cap.name(label).map_or("", |m| m.as_str())
}

fn main() {
    let cli = Cli::from_args();
    let labels = column_headers(&cli.regex);
    let rows: Vec<Row> = match &cli.file {
        Some(file) => fs::read_to_string(file)
            .unwrap()
            .par_lines()
            .map(|line| cli.regex.captures(line))
            .flatten()
            .map(|cap| {
                Row::new(
                    labels
                        .iter()
                        .map(|label| Cell::new(get_named(&cap, &label)))
                        .collect::<Vec<Cell>>(),
                )
            })
            .collect(),
        None => grab_stdin()
            .unwrap()
            .par_lines()
            .map(|line| cli.regex.captures(&line))
            .flatten()
            .map(|cap| {
                Row::new(
                    labels
                        .iter()
                        .map(|label| Cell::new(get_named(&cap, &label)))
                        .collect::<Vec<Cell>>(),
                )
            })
            .collect(),
    };
    let mut tbl = Table::init(rows);

    let titles: Row = labels.iter().map(|label| Cell::new(label)).collect();
    tbl.set_titles(titles);

    match &cli.output_format {
        OutputFormat::Csv => {
            tbl.to_csv(io::stdout().lock()).unwrap();
        }
        OutputFormat::Ascii => {
            tbl.printstd();
        }
    };
}
