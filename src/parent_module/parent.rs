use super::nested_module_1;
use crate::parent_module::nested_module_2;

fn function() {
    println!("hello parent");
    println!("call children");

    nested_module_1::child_1::function();
    nested_module_2::child_2::function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        function();
    }
}

