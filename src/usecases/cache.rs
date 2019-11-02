use std::str;
use std::process::Command;

use crate::usecases::UseCase;

pub struct CheckPackmanCache {}

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

    fn execute(&self) -> String {
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

        "done".to_string()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::_test_number; 

//     #[test]
//     fn test_greet() {
//         assert_eq!(8, _test_number());
//     }
// }