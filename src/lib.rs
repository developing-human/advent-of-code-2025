pub mod problem01 {
    struct Safe {
        position: usize,
    }

    impl Default for Safe {
        fn default() -> Self {
            Safe { position: 50 }
        }
    }

    impl Safe {
        // Moves the dial, returning how many times zero was passed.
        fn turn(&mut self, amount: i32) -> usize {
            let start = self.position as i32;
            let end = start + amount;

            // using % will return negatives, rem_euclid is always non-negative
            // ex: -1 % 100 => -1, -1.rem_euclid(100) => 99
            self.position = end.rem_euclid(100) as usize;

            Safe::count_zero_clicks(start, end)
        }

        /// Counts how many times zero was passed.
        fn count_zero_clicks(start: i32, end: i32) -> usize {
            if end > 0 {
                // for positive, count how many times we passed 100
                (end / 100) as usize
            } else {
                let zero_clicks = (-end / 100) as usize;

                // for negative, 1 -> -1 counts, but 0 -> -1 does not
                // so add 1, but not when starting at zero
                match start {
                    0 => zero_clicks,
                    _ => zero_clicks + 1,
                }
            }
        }

        fn is_zeroed(&self) -> bool {
            self.position == 0
        }
    }

    fn parse_movement(movement: &str) -> i32 {
        let (direction, amount) = movement
            .split_at_checked(1)
            .expect("movement should be letter then digits");

        let amount: i32 = amount.parse().expect("digits should parse to int");

        match direction {
            "L" => -amount,
            "R" => amount,
            _ => panic!("direction should be L or R"),
        }
    }

    pub fn solve(input: &str) -> (usize, usize) {
        let mut safe = Safe::default();

        let mut zeroes = 0;
        let mut zero_clicks = 0;
        for one_movement in input.lines() {
            let amount = parse_movement(one_movement);
            zero_clicks += safe.turn(amount);

            if safe.is_zeroed() {
                zeroes += 1;
            }
        }

        (zeroes, zero_clicks)
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

        #[test]
        fn test_solve_full_input() {
            let input = std::fs::read_to_string("inputs/problem01.txt").unwrap();
            let result = solve(&input);
            assert_eq!(result, (1076, 6379));
        }

        #[test]
        fn test_safe_turn_positive() {
            let mut safe = Safe::default();

            let zero_clicks = safe.turn(49);
            assert!(!safe.is_zeroed());
            assert_eq!(zero_clicks, 0);

            let zero_clicks = safe.turn(1);
            assert!(safe.is_zeroed());
            assert_eq!(zero_clicks, 1);

            let zero_clicks = safe.turn(300);
            assert!(safe.is_zeroed());
            assert_eq!(zero_clicks, 3);
        }

        #[test]
        fn test_safe_turn_negative() {
            let mut safe = Safe::default();

            let zero_clicks = safe.turn(-49);
            assert!(!safe.is_zeroed());
            assert_eq!(zero_clicks, 0);

            let zero_clicks = safe.turn(-1);
            assert!(safe.is_zeroed());
            assert_eq!(zero_clicks, 1);

            let zero_clicks = safe.turn(-300);
            assert!(safe.is_zeroed());
            assert_eq!(zero_clicks, 3);
        }
    }
}
