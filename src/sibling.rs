use crate::sub_module::sub;

pub fn function() {
    println!("hello sibling");
    sub::function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        function();
    }
}