use console::Term;
use console::style;
use std::io::*;

mod usecases;

use usecases::UseCase;

fn main() {
    // temporal
    let test123 = usecases::items();
    println!("{}", test123[0].id());
    println!("{}", test123[0].name());
    println!("{}", test123[0].description());

    let output = test123[0].execute();
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    //

    // print_main_menu();
    read_option(Term::stdout().read_line());
}

fn print_main_menu() {
    println!("---------------------------------------------");
    println!("Welcome to {} - Version 1.0.0", style("Doggy").green());
    println!("---------------------------------------------");
    println!(" 1 - {}", style("Check pacman cache size.").blue());
    println!(" 2 - {}", style("Check pacaur cache size.").cyan());
    println!("---------------------------------------------");
    println!("Your option: ");
}

fn read_option(result: Result<String>) {
    match result {
        Ok(opt) => process_option(opt.as_str()),
        Err(e)  => println!("Error: {}", e),
    }
}

fn process_option(opt: &str) {
    match opt {
        "1" => println!("du -sh /var/cache/pacman/pkg"),
        "2" => println!("du -sh ~/.cache/pacaur/"),
        _ => println!("Any other option pressed"),
    }
}