pub fn get_argv() -> Vec<String> {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 {
        panic!("Too few arguments provided. Correct usage:\njk <path>");
    }
    argv
}

pub fn get_file(path: &String) -> String {
    let file_string = std::fs::read_to_string(path).unwrap();
    file_string
}

pub fn run(content: &String) {
    use std::fs;
    use std::process::{Command, Stdio};

    fs::write("lang/a.s", content).expect("Failed to write assembly file");

    let assemble = Command::new("as")
        .args(["lang/a.s", "-o", "lang/a.o"])
        .output()
        .expect("Failed to assemble");

    if !assemble.status.success() {
        eprintln!("Assembler error: {:?}", assemble);
        return;
    }

    let xcrun_output = Command::new("xcrun")
        .args(["-sdk", "macosx", "--show-sdk-path"])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to get SDK path");

    let syslibroot = String::from_utf8_lossy(&xcrun_output.stdout).trim().to_string();

    let link = Command::new("ld")
        .args([
            "-macos_version_min", "15.0.0",
            "-o", "lang/a.out",
            "lang/a.o",
            "-lSystem",
            "-syslibroot", &syslibroot,
            "-e", "_start",
            "-arch", "arm64",
        ])
        .output()
        .expect("Failed to link");

    if !link.status.success() {
        eprintln!("Linker error: {:?}", link);
    }
}
