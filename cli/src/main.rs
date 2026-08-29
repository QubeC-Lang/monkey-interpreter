use monkey_interpreter_core::repl;

fn main() {
    let username = whoami::username().unwrap_or("User".to_string());
    println!("Hello, {username}! Welcome to the Monkey (QubeC) REPL.");

    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    repl::start_repl(stdin.lock(), stdout.lock());

    println!();
    println!("Goodbye, {username}!");
}
