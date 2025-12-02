fn main() {
    let input = std::fs::read_to_string("inputs/problem01.txt").expect("file should load");

    let output = aoc::problem01::solve(&input);

    println!("{output:?}");
}
