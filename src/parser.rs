pub type AST = ();

pub fn parse() -> AST {
    println!("Parsing...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = parse();
        assert_eq!(result, ());
    }
}
