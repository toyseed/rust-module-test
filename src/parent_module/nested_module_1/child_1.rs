use super::super::nested_module_2;

pub fn function() {
    println!("hello child 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_parent_module() {
        nested_module_2::child_2::function();
    }
}