use lazy_static::lazy_static;
use prettytable::{cell, Cell, row, Row, Table, table};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::read_to_string;

type Result<T> = std::result::Result<T, std::io::Error>;

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    view: BTreeMap<String, View>,
}
#[derive(Deserialize, Serialize, Debug)]
struct View {
    regexes: Vec<String>,
    file: String,
    labels: Vec<String>
}

fn main() {
    if let Err(error) = try_main() {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}


fn try_main() -> Result<()> {
    use std::collections::BTreeMap;

    let config_str = read_to_string("./config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    println!("{:#?}", &config);


    for (view_name, view_config) in &config.view {
        let content = read_to_string(&view_config.file)?;

        let mut regex_vec: Vec<Regex> = Vec::new();
        for regex_str in &view_config.regexes {
            let regex: Regex = Regex::new(regex_str).expect("regex does not compile!");
            regex_vec.push(regex);
        };

        let mut output_table = Table::new();
        output_table.add_row(row![view_name]);
        let mut label_row: Vec<Cell> = Vec::new();
        for label in &view_config.labels {
            label_row.push(Cell::new(&label))
        };
        output_table.add_row(Row::new(label_row));

        for line in content.lines() {
            //println!("{:#?}", &line);
            for regex in &regex_vec {
                if let Some(cap) = regex.captures(line) {
                    //println!("{:#?}", cap);
                    let mut tbl_row: Vec<Cell> = Vec::new();

                    for label in &view_config.labels {
                        let val = &cap.name(label).map_or("", |m | m.as_str());
                        tbl_row.push(Cell::new(val))
                    };
                    output_table.add_row(Row::new(tbl_row));
                };

            };
        };
        output_table.printstd();
    };

    Ok(())
}
