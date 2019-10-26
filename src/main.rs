use console::Term;
use console::style;

fn main() {
    let term = Term::stdout();
    let _result = term.write_line("Hello World");
    menu();
    let _option = term.read_char();
}

fn menu() {
    println!("---------------------------------------------");
    println!("Welcome to {} - Version 1.0.0", style("Doggy").green());
    println!("---------------------------------------------");
    println!(" 1 - {}", style("Check pacman cache size.").blue());
    println!(" 2 - {}", style("Check pacaur cache size.").cyan());
    println!("---------------------------------------------");
    println!("Choose one option: ");
    println!("---------------------------------------------");
}