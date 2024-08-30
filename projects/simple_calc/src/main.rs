enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn op(operand: String) -> Operator {
        match operand.as_str() {
            "+" => return Operator::Add,
            "-" => return Operator::Sub,
            "x" => return Operator::Mul,
            "/" => return Operator::Div,
            _ => {
                eprintln!("Not an operator");
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 4 {
        panic!("Insufficient ammount of arguments given");
    }

    let num1: i32 = argv[1].trim().parse().unwrap_or_else(|e| {
        eprintln!("Invalid number: {e}");
        std::process::exit(1);
    });
    let num2: i32 = argv[3].trim().parse().unwrap_or_else(|e| {
        eprintln!("Invalid number: {e}");
        std::process::exit(1);
    });

    let str: String = argv[2].clone();

    let result: i32 = match Operator::op(str) {
        Operator::Add => num1 + num2,
        Operator::Sub => num1 - num2,
        Operator::Mul => num1 * num2,
        Operator::Div => {
            if num2 == 0 {
                panic!("Division by Zero Error");
            } else {
                num1 / num2
            }
        }
    };

    println!("{result}");
}
