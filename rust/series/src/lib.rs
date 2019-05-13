pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec!["".to_string(); digits.len() + 1],
        len if len > digits.len() => vec![],
        _ => {
            let mut result = vec![];
            for x in 0..digits.len() - len + 1 {
                result.push(digits[x..x + len].to_string());
            }
            result
        }
    }
}
