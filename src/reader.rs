pub type Source = ();

pub fn read_source(filename: &str) -> Source {
    println!("Reading {filename}...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_source() {
        let result = read_source("hello.txt");
        assert_eq!(result, ());
    }
}
