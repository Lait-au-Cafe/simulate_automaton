extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;


pub struct Data {
    pub input: char,    // オートマトンへの入力
    pub output: char    // オートマトンの出力
}

pub type DataList = Vec<Data>;

pub fn load_data(filename: &str) -> Result<DataList, String> {
    // open file
    let path = Path::new(&filename);
    let display = path.display();
    let file = File::open(&path)
        .map_err(|e|format!("Cannot open {}", display).to_owned() 
            + " : " + e.description())?;

    // read file
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)
        .map_err(|e|"Cannot read file : ".to_owned() + e.description())?;
    
    // convert
    let raw = String::from_utf8(buf)
        .map_err(|e|e.description().to_owned())?;

    let mut list: Vec<Data> = Vec::new();
    for s in raw.split('\n') {
        let cs = s.chars().collect::<Vec<char>>();
        regex::Regex::new("^[gw],[gw]$")
            .map_err(|e| e.description().to_owned())?
            .find(s)
            .and_then(|_| {
                list.push(Data{
                    input: *cs.last().unwrap(),
                    output: *cs.first().unwrap()
                });
                Some(0)
            });
    }
    
    Ok(list)
}
