pub type Tokens = ();

pub fn tokenize() -> Tokens {
    println!("Tokenizing...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let result = tokenize();
        assert_eq!(result, ());
    }
}
