
#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_panic() {
        fn natural(n:i32) -> bool {
            if n < 0 {
                panic!("should be positive number!");
            }

            n > 0
        }

        natural(-1);
    }
}
