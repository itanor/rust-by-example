pub fn dummy_function() -> String {
    String::from("some text")
}

#[cfg(test)]
mod tests {
    use super::dummy_function;

    #[test]
    fn test_dummy_function() {
        assert_eq!(dummy_function(), "some text");
    }
}
