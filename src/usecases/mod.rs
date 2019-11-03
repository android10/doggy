
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
        CheckPackmanCache::new()
    ]; 

    return usecases;
} 

#[cfg(test)]
mod tests {
    use super::UseCase;
    use super::items; 

    #[test]
    fn test_number_of_use_cases() {
        assert_eq!(1, items().len());
    }

    #[test]
    fn test_order_of_elements() {
        use std::convert::TryInto;
        let usecases = items();

        for (i, item) in usecases.iter().enumerate() {
            let current_element_plus_one: u32 = (i + 1).try_into().unwrap();
            let item_id = item.id().parse::<u32>();

            assert_eq!(item_id, Ok(current_element_plus_one));
        }
    }
}