fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_main() {
        main();
    }
}
