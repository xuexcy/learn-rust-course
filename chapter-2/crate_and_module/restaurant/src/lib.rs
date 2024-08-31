mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn eat_at_restaurant() {
    // crate::front_of_house::hosting::add_to_waiting_list();
    // front_of_house::hosting::add_to_waiting_list();
    hosting::add_to_waiting_list();
}


fn serve_order() {
    self::back_of_house::cook_order();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        // crate::serve_order();
    }
    pub fn cook_order() {}
}
