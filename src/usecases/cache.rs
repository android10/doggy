use std::process::Command;
use std::process::Output;

use crate::usecases::UseCase;

pub struct CheckPackmanCache {}

impl CheckPackmanCache {
    pub fn new() -> CheckPackmanCache {
        CheckPackmanCache {}
    }
}

impl UseCase for CheckPackmanCache {
    fn id(&self) -> String { 
        "1".to_string()
    }

    fn name(&self) -> String { 
        "name".to_string()
    }

    fn description(&self) -> String { 
        "description".to_string() 
    }

    fn execute(&self) -> Output {
        let output = Command::new("du")
                         .arg("-s")
                         .arg("-h")
                         .arg("/var/cache/pacman/pkg")
                         .output()
                         .expect("failed to execute process");

        assert!(output.status.success());

        return output;
    }
}