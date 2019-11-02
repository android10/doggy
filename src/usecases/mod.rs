
mod cache;

// use cache::CheckPackmanCache;
use cache::CheckPackmanCache;

pub trait UseCase {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn execute(&self) -> String;
}

// pub fn use_cases() -> CheckPackmanCache impl UseCase {
pub fn use_cases() -> impl UseCase {
    // let ys: [i32; 500] = [0; 500];
    // let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // let usecases: [CheckPackmanCache; 1] = [
    //     CheckPackmanCache {}
    // ]; 

    return CheckPackmanCache {};
} 

pub fn _test_number() -> i32 {
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