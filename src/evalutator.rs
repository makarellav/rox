pub fn evaluate() -> () {
    println!("Evaluating...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let result = evaluate();
        assert_eq!(result, ());
    }
}
