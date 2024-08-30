use rand::Rng;

enum Cmp {
    Greater,
    Equal,
    Smaller,
}

fn compare(inp: i32, num: i32) -> Cmp {
    if inp == num {
        return Cmp::Equal;
    } else if inp < num {
        return Cmp::Smaller;
    } else if inp > num {
        return Cmp::Greater;
    }
    return Cmp::Equal;
}

fn main() {
    let sec = rand::thread_rng().gen_range(0..=100);

    loop {
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap_or_else(|e| {
            eprintln!("Error reading line from stdin: {e}");
            std::process::exit(1);
        });

        if guess == String::from("quit") {
            std::process::exit(1);
        }

        let guess: i32 = guess.trim().parse().unwrap_or_else(|e| {
            eprintln!("Invalid Input: {e}");
            std::process::exit(1);
        });

        match compare(guess, sec) {
            Cmp::Equal => {
                println!("You got it!");
                std::process::exit(1)
            }
            Cmp::Greater => println!("Your guess is too big\n"),
            Cmp::Smaller => println!("Your guess is too small\n"),
        };
    }
}
