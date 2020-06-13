fn main() {
    println!("hello not included");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        main();
    }
}
