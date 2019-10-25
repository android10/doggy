use console::Term;
use console::style;

fn main() {
    let term = Term::stdout();
    let _result = term.write_line("Hello World");
    println!("Hello {} with Style", style("Fernando").cyan());
}