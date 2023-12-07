use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    let raw = read_to_string(filename);

    match raw {
        Ok(data) => {
            return data.lines().map(String::from).collect();
        }
        Err(err) => {
            panic!("FILE READING ERROR\nFILE: {}\nERROR: {}", filename, err)
        }
    }
}