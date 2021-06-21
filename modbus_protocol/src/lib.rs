extern crate num_derive;
extern crate num_traits;

pub mod coils;
pub mod requests;
pub mod exception_code;
pub mod function_code;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

