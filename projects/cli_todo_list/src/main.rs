const FILE_PATH: &str = "src/list.txt";

use std::fs::File;
use std::io::Write;

use cli_todo_list;
use cli_todo_list::*;

fn main() {
    let argv: Vec<String> = read_cli();
    let file_content: Result<Vec<String>, String> = read_to_vec(FILE_PATH);
    let mut file_content: Vec<String> = file_content.unwrap_or_else(|e| {
        eprintln!("File reader had returned an error: {e}");
        std::process::exit(1);
    });

    if file_content.len() == 0 {
        panic!("list.txt is corrupted, add a 0 to the front of the file");
    }

    let tag: i32 = file_content[0].parse().unwrap_or_else(|e| {
        eprintln!("Failed to parse information: {e}");
        eprintln!("Recommanded action: Reset list.txt");
        std::process::exit(1);
    });

    if argv[1] == String::from("insert") {
        let content: String = argv[2].clone();

        let todo: Todo = Todo::new(tag, content);

        file_content.push(format!("{0} {1} {2}", todo.tag, todo.content, todo.state));
        let tag = tag + 1;
        file_content[0] = format!("{tag}");
    } else if argv[1] == String::from("markdone") {
        let index: usize = argv[2].parse().unwrap_or_else(|e| {
            eprintln!("Invalid second input: {e}");
            std::process::exit(1);
        });
        if index < file_content.len() {
            file_content.remove(index);
        }
    } else if argv[1] == String::from("display") {
        println!("{:#?}", file_content);
    }

    let mut file = match File::create(FILE_PATH) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {e}");
            std::process::exit(1);
        }
    };
    for line in &file_content {
        if let Err(e) = writeln!(file, "{line\n}") {
            eprintln!("Failed to write into file: {e}");
            std::process::exit(1);
        }
    }
}
