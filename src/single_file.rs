pub mod single {
    pub fn function() {
        println!("hello single");
    }

    pub mod nested {
        use crate::sibling;

        pub fn function() {
            // str 에 - 가 포함되어 있으면 println! 결과에 개행이 적용되지 않음.
            println!("hello single - nested");
        }

        pub fn call_sibling() {
            sibling::function();
        }
    }
}

// see main:10 to see that how to `pub` effects others
pub use self::single::nested;

fn call_self() {
    single::nested::function();
    nested::function();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        single::function();
        single::nested::function();
        single::function();
        single::nested::function();
        single::nested::call_sibling();
    }

    #[test]
    fn test_call_self() {
        call_self();
    }
}