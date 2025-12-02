pub fn solve(_input: &str) -> (usize, usize) {
    unimplemented!("not there yet...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_basic_input() {
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        let result = solve(input);
        assert_eq!(result, (3, 6));
    }
}
