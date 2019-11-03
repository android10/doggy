
mod cache;

use cache::CheckPackmanCache;

pub trait UseCase {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn execute(&self) -> String;
}

pub fn items() -> [impl UseCase; 1] {
    let usecases: [CheckPackmanCache; 1] = [
        CheckPackmanCache {}
    ]; 

    return usecases;
} 

#[cfg(test)]
mod tests {
    use super::items; 

    #[test]
    fn test_number_of_use_cases() {
        assert_eq!(1, items().len());
    }
}