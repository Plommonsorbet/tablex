use prettytable::{cell, row, Cell, Row, Table};
use rayon::prelude::*;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use structopt::{clap::AppSettings, StructOpt};

type Result<T> = std::result::Result<T, std::io::Error>;

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::SubcommandsNegateReqs, author)]
enum CliSubCommand {
    View { name: String },
}

#[derive(Debug, StructOpt)]
struct Cli {
    regex: Option<String>,

    input_file: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Option<CliSubCommand>,
}

fn get_labels(re: &Regex) -> Vec<String> {
    re.capture_names()
        .into_iter()
        .flatten()
        .map(|s| s.to_string())
        .collect()
}
macro_rules! get_named_capture {
    ($cap:expr, $label:expr) => {
        $cap.name($label)
            .map_or(String::from(""), |m| m.as_str().to_string())
    };
}

fn get_config_path() -> PathBuf {
    let base_dir_str = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => path,
        Err(_) => env::var("HOME").expect("No environment variable set for $HOME"),
    };
    PathBuf::from(base_dir_str).join("tablex/config.toml")
}

fn parse_capture(labels: &Vec<String>, cap: Captures) -> Vec<String> {
    labels
        .iter()
        .map(|label| get_named_capture!(&cap, &label))
        .collect()
}

fn parse_table(regex_str: &str, content: String) -> Result<Vec<Vec<String>>> {
    let re: Regex = Regex::new(regex_str).expect("regex does not compile!");
    let labels = get_labels(&re);
    let table: Vec<Vec<String>> = content
        .lines()
        .map(|line| re.captures(line))
        .flatten()
        .map(|cap| parse_capture(&labels, cap))
        .collect();
    Ok(table)
}

fn into_ascii_table(table: Vec<Vec<String>>) {
    let mut tbl = Table::new();

    for table_row in table.into_iter() {
        tbl.add_row(Row::new(
            table_row
                .into_iter()
                .map(|cell| Cell::new(&cell))
                .collect::<Vec<Cell>>(),
        ));
    }
    tbl.printstd();
}

pub fn try_main() -> Result<()> {
    let config_path = get_config_path();
    let config = Config::from(&config_path)?;

    let cli = Cli::from_args();

    if let Some(subcmd) = cli.cmd {
        match subcmd {
            CliSubCommand::View { name } => {
                let view = config
                    .view
                    .get(&name)
                    .expect("ERROR: No views by that name found");

                let content = read_to_string(&view.file)?;
                let tbl = parse_table(&view.regex, content)?;
                into_ascii_table(tbl);
                //let tbl = RegexTable::new(&view.regex, name.clone());

                //tbl.ascii(content);
            }
        };
    } else {
        if let (Some(regex), Some(file)) = (&cli.regex, &cli.input_file) {
            let content = read_to_string(file)?;
            let tbl = parse_table(regex, content)?;
            into_ascii_table(tbl);
            //let content = read_to_string(file)?;
            //let tbl = RegexTable::new(regex, "".to_string());

            //tbl.ascii(content);
        }
    };
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    view: BTreeMap<String, View>,
}
#[derive(Deserialize, Serialize, Debug)]
struct View {
    regex: String,
    file: String,
}
impl Config {
    pub fn from(path: &Path) -> Result<Self> {
        let config_str = read_to_string(path)?;
        Ok(toml::from_str(&config_str)?)
    }
}

//struct RegexTable {
//    re: Regex,
//    title: String,
//    labels: Vec<String>,
//}
//
//impl RegexTable {
//    fn new(regex_str: &str, title: String) -> Self {
//        let re: Regex = Regex::new(regex_str).expect("regex does not compile!");
//        Self {
//            labels: get_labels(&re),
//            kre,
//            title,
//        }
//    }
//fn ascii(&self, content: String) {
//    let mut tbl = Table::new();

//    tbl.add_row(row![self.title]);
//    tbl.add_row(Row::new(
//        self.labels
//            .iter()
//            .map(|l| Cell::new(l))
//            .collect::<Vec<Cell>>(),
//    ));
//    let rows: Vec<Row> = content.par_lines().map(|line| self.re.captures(line) )
//        .flatten()
//        .map(|cap|
//                Row::new(self.labels
//                    .iter()
//                    .map(|label| Cell::new(get_named(&cap, &label)))
//                    .collect::<Vec<Cell>>())
//        ).collect();
//    for row in rows {
//        tbl.add_row(row);
//    };

//    tbl.printstd();
//}
//}
