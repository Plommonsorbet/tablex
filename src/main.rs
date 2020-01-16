use prettytable::{cell, row, Cell, Row, Table};
use rayon::prelude::*; use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::env;
use std::path::{PathBuf, Path};
use structopt::{clap::AppSettings, StructOpt};

type Result<T> = std::result::Result<T, std::io::Error>;

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

#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::SubcommandsNegateReqs, author)]
enum CliSubCommand {
    View {
        name: String
    }
}

#[derive(Debug, StructOpt)]
struct Cli {
    regex: Option<String>,

    input_file: Option<PathBuf>,
    
    #[structopt(subcommand)]
    cmd: Option<CliSubCommand>

}

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}

fn get_labels(re: &Regex) -> Vec<String> {
    re.capture_names()
        .into_iter()
        .flatten()
        .map(|s| s.to_string())
        .collect()
}
fn get_named<'a>(cap: &'a Captures, label: &str) -> &'a str {
    &cap.name(label).map_or("", |m| m.as_str())
}

fn get_config_path() -> PathBuf {
    let base_dir_str = match env::var("XDG_CONFIG_HOME") {
        Ok(path) => path,
        Err(_) => env::var("HOME").expect("No environment variable set for $HOME")
    };
    PathBuf::from(base_dir_str).join("tablex/config.toml")
}
struct RegexTable {
    re: Regex,
    title: String,
    labels: Vec<String>,
}

impl RegexTable {
    fn new(regex_str: &str, title: String) -> Self {
        let re: Regex = Regex::new(regex_str).expect("regex does not compile!");
        Self {
            labels: get_labels(&re),
            re,
            title,
        }
    }
    fn ascii(&self, content: String) {
        let mut tbl = Table::new();

        tbl.add_row(row![self.title]);
        tbl.add_row(Row::new(
            self.labels
                .iter()
                .map(|l| Cell::new(l))
                .collect::<Vec<Cell>>(),
        ));

        let rows: Vec<Row> = content.par_lines().map(|line| self.re.captures(line) )
            .flatten()
            .map(|cap|
                    Row::new(self.labels
                        .iter()
                        .map(|label| Cell::new(get_named(&cap, &label)))
                        .collect::<Vec<Cell>>())
            ).collect();

        for row in rows {
            tbl.add_row(row);
        };

        tbl.printstd();
    }
}

fn try_main() -> Result<()> {
    let config_path = get_config_path();
    let config = Config::from(&config_path)?;

    let cli = Cli::from_args();


    if let Some(subcmd) = cli.cmd {
        match subcmd {
            CliSubCommand::View {
                name,
            } => {
                let view = config.view.get(&name).expect("ERROR: No views by that name found");
                
                let content = read_to_string(&view.file)?;
                let tbl = RegexTable::new(&view.regex, name.clone());

                tbl.ascii(content);
                
            }
        };
    } else {
        if let (Some(regex), Some(file)) = (&cli.regex, &cli.input_file) {
                let content = read_to_string(file)?;
                let tbl = RegexTable::new(regex, "".to_string());

                tbl.ascii(content);
        }
        
    };
    Ok(())
}
