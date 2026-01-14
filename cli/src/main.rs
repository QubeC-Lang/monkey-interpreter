use monkey_interpreter_core::add;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <num1> <num2>", args[0]);
        std::process::exit(1);
    }

    let num1: u64 = args[1].parse().expect("Please provide a valid number for num1.");
    let num2: u64 = args[2].parse().expect("Please provide a valid number for num2.");
    let result = add(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, result);
}
