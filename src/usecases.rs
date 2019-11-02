use std::io::*;

pub trait UseCase {
    fn execute(&self) -> Result<String>;
}

fn _test_number() -> i32 {
    8
}

#[cfg(test)]
mod tests {
    use super::_test_number; 

    #[test]
    fn test_greet() {
        assert_eq!(8, _test_number());
    }
}