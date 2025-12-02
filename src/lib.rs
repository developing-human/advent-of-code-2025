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
        // returns the number of clicks at 0
        fn turn(&mut self, amount: i32) -> usize {
            let updated = self.position as i32 + amount;

            // counts number of times going past 100 (which hits zero)
            // and how many times going past zero... but there are edge
            // cases around landing on zero
            let mut zero_clicks = if updated > 0 {
                (updated / 100) as usize
            } else {
                (updated / -100) as usize
            };

            if updated <= 0 && self.position != 0 {
                zero_clicks += 1;
            }

            self.position = updated.rem_euclid(100) as usize;
            zero_clicks
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
