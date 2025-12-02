pub fn solve(input: &str) -> usize {
    input.split(",").map(solve_one_range).sum()
}

fn solve_one_range(range: &str) -> usize {
    let split: Vec<&str> = range.trim().split("-").collect();
    let start: usize = split[0].parse().expect("start of range should be integer");
    let end: usize = split[1].parse().expect("end of range should be integer");

    (start..=end).filter(is_invalid).sum()
}

fn is_invalid(number: &usize) -> bool {
    let number_str = format!("{number}");

    if number_str.len() % 2 == 1 {
        return false; // odd length strings are always valid
    }

    let (first_half, second_half) = number_str.split_at(number_str.len() / 2);

    first_half == second_half
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_basic_input() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let result = solve(input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn solve_one_range_11_22() {
        let input = "11-22";
        let result = solve_one_range(input);
        assert_eq!(result, 33);
    }

    #[test]
    fn solve_one_range_95_115() {
        let input = "95-115";
        let result = solve_one_range(input);
        assert_eq!(result, 99);
    }

    #[test]
    fn solve_one_range_998_1012() {
        let input = "998-1012";
        let result = solve_one_range(input);
        assert_eq!(result, 1010);
    }

    #[test]
    fn solve_one_range_1188511880_1188511890() {
        let input = "1188511880-1188511890";
        let result = solve_one_range(input);
        assert_eq!(result, 1188511885);
    }

    #[test]
    fn solve_one_range_222220_222224() {
        let input = "222220-222224";
        let result = solve_one_range(input);
        assert_eq!(result, 222222);
    }

    #[test]
    fn solve_one_range_1698522_1698528() {
        let input = "1698522-1698528";
        let result = solve_one_range(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn solve_one_range_446443_446449() {
        let input = "446443-446449";
        let result = solve_one_range(input);
        assert_eq!(result, 446446);
    }

    #[test]
    fn solve_one_range_38593856_38593862() {
        let input = "38593856-38593862";
        let result = solve_one_range(input);
        assert_eq!(result, 38593859);
    }
}
