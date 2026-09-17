use crate::error::RoxError;
use std;

#[derive(Debug)]
pub struct Source {
    pub raw: String,
}

pub fn read_source(filename: &str) -> Result<Source, RoxError> {
    let raw = std::fs::read_to_string(filename)?;

    Ok(Source { raw })
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_read_source() {
//         let result = read_source("hello.txt");
//         assert_eq!(result, ());
//     }
// }
