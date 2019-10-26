use console::Term;
use console::style;
use std::io::*;
use std::process::Command;

fn main() {
    print_main_menu();
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
        "1" => execute_command("du", "-sh /var/cache/pacman/pkg"),
        "2" => execute_command("du", "-sh ~/.cache/pacaur/"),
        _ => println!("Any other option pressed"),
    }
}

fn execute_command(_cmd: &str, _arg: &str) {
    let output = Command::new("du")
                         .arg("-s")
                         .arg("-h")
                         .arg("/var/cache/pacman/pkg")
                         .output()
                         .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
}