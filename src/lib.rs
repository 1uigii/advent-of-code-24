mod implement;

implement::day![01, 02];

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("day 01 :: {0}")]
    Day01(#[from] day01::Error),
    #[error("day 02 :: {0}")]
    Day02(#[from] day02::Error),
}

pub trait Process {
    fn process(&self) -> Result<String, crate::Error>;
}

pub fn process<T: Process>(process: T) -> Result<String, crate::Error> {
    process.process()
}

pub fn process_dyn(t: &dyn Process) -> Result<String, crate::Error> {
    t.process()
}

#[derive(clap::Parser, Debug, Clone, Copy)]
#[command(version, about, long_about = None)]
pub enum Day {
    Day01 {
        #[arg(value_enum)]
        part: Part,
    },
    Day02 {
        #[arg(value_enum)]
        part: Part,
    },
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum Part {
    Part1,
    Part2,
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::Day01 { part } => write!(f, "day 01 :: {part}"),
            Day::Day02 { part } => write!(f, "day 02 :: {part}"),
        }
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => f.write_str("part 1"),
            Part::Part2 => f.write_str("part 2"),
        }
    }
}

pub fn process_args(args: &Day) -> Result<String, Error> {
    match args {
        Day::Day01 { part: Part::Part1 } => process(day01::Part1::default()),
        Day::Day01 { part: Part::Part2 } => process(day01::Part2::default()),
        Day::Day02 { part: Part::Part1 } => process(day02::Part1::default()),
        Day::Day02 { part: Part::Part2 } => process(day02::Part2::default()),
    }
}
