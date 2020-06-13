
mod sibling;
mod sub_module;
mod parent_module;
mod single_file;

use parent_module::{nested_module_1, nested_module_2};
// use single_file::single::nested;
use single_file::nested;

pub fn hello() {
    println!("hello rust_module");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sibling_module() {
        sibling::function();
    }

    #[test]
    fn test_sub_module() {
        sub_module::sub::function();
    }

    #[test]
    fn test_nested_module() {
        nested_module_1::child_1::function();
        nested_module_2::child_2::function();
    }

    #[test]
    fn test_single() {
        single_file::single::function();
        single_file::single::nested::function();
        nested::function();
    }
}
