mod rust_shared;
use rust_shared::*;

const DAY: usize = %day%;
const PART: usize = %part%;

// Solution ───────────────────────────────────────────────────────────────── //

fn solve(lines: Vec<String>) -> isize {
	lines.iter().map(|line| solve_line(line)).sum()
}

fn solve_line(line: &str) -> isize {
	0
}

// Main ───────────────────────────────────────────────────────────────────── //

fn main() {
	// run_test(DAY, PART, solve);
	run_test_and_actual(DAY, PART, solve);
}