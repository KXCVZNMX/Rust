use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Todo {
    pub tag: i32,
    pub content: String,
    pub state: u8,
}

pub fn read_cli() -> Vec<String> {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 3 {
        panic!(
            "\nInsufficient argument given, example below:\n\tcargo run -- <operator> <string/tag>"
        )
    }
    return argv;
}

pub fn read_to_vec(file_path: &str) -> Result<Vec<String>, String> {
    let file = File::open(&file_path).map_err(|e| format!("Failed to open file: {e}"))?;
    let buf_reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in buf_reader.lines() {
        match line {
            Ok(l) => lines.push(l),
            Err(e) => return Err(format!("Failed to read line: {e}")),
        }
    }

    Ok(lines)
}

impl Todo {
    pub fn new(tag: i32, content: String) -> Todo {
        let state: u8 = 0;
        Todo {
            tag,
            content,
            state,
        }
    }
}
