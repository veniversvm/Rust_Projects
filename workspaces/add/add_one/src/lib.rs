pub fn add_one(x: u64) -> u64 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_test() {
        let result = add_one(6);
        assert_eq!(result, 7);
    }
}
