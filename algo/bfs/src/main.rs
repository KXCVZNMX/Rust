use std::fs;

fn parse_input() -> std::io::Result<(usize, usize, usize, Vec<Vec<char>>)> {
    // Read the input from a file (replace "input.txt" with the path to your input)
    let input = fs::read_to_string("src/oilin.txt")?;

    // Split the input by lines
    let mut lines = input.lines();

    // Parse the first three integers
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let a: usize = parts.next().unwrap().parse().unwrap();
    let b: usize = parts.next().unwrap().parse().unwrap();
    let c: usize = parts.next().unwrap().parse().unwrap();

    // Initialize a 2D vector to store the grid
    let mut grid: Vec<Vec<char>> = Vec::new();

    // Parse the remaining lines into the 2D vector
    for line in lines {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    Ok((a, b, c, grid))
}

fn valid(x: i32, y: i32, boundx: i32, boundy: i32) -> bool {
    return x >= 0 && x < boundx && y >= 0 && y < boundy;
}

fn bfs(&mut grid: Vec<Vec<char>>, )

fn main() {
    let (a, b, c, mut grid) = parse_input().unwrap();
}
