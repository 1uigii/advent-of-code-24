use advent_of_code as aoc;
use clap::Parser;

fn main() {
    let args = aoc::Day::parse();

    match aoc::process_args(&args) {
        Ok(ok) => println!("aoc :: {args} :: ok => {ok}"),
        Err(err) => println!("aoc :: {args} :: err => {err}"),
    }
}
