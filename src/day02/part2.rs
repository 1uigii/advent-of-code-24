use itertools::Itertools;
use smallvec::SmallVec;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("parse :: no report in line")]
    NoReportInLine,
    #[error("parse :: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

fn count_inc_errors_ext(beg: usize, rest: usize, slice: &[i32]) -> u32 {
    if rest >= slice.len() {
        return 0;
    }

    if (1..=3).contains(&(slice[rest] - slice[beg])) {
        count_inc_errors(&slice[rest..])
    } else {
        count_inc_errors_ext(beg, rest + 1, slice) + 1
    }
}

fn count_inc_errors(slice: &[i32]) -> u32 {
    for i in 1..slice.len() {
        if (1..=3).contains(&(slice[i] - slice[i - 1])) {
            continue;
        }

        let val1 = if i == 1 {
            count_inc_errors(&slice[1..])
        } else {
            count_inc_errors_ext(i - 2, i, slice)
        };
        let val2 = count_inc_errors_ext(i - 1, i + 1, slice);

        return u32::min(val1, val2) + 1;
    }
    0
}

fn count_dec_errors_ext(beg: usize, rest: usize, slice: &[i32]) -> u32 {
    if rest >= slice.len() {
        return 0;
    }

    if (1..=3).contains(&(slice[beg] - slice[rest])) {
        count_dec_errors(&slice[rest..])
    } else {
        count_dec_errors_ext(beg, rest + 1, slice) + 1
    }
}

fn count_dec_errors(slice: &[i32]) -> u32 {
    for i in 1..slice.len() {
        if (1..=3).contains(&(slice[i - 1] - slice[i])) {
            continue;
        }

        let val1 = if i == 1 {
            count_dec_errors(&slice[1..])
        } else {
            count_dec_errors_ext(i - 2, i, slice)
        };
        let val2 = count_dec_errors_ext(i - 1, i + 1, slice);

        return u32::min(val1, val2) + 1;
    }
    0
}

fn count_errors(slice: &[i32]) -> u32 {
    u32::min(count_inc_errors(slice), count_dec_errors(slice))
}

pub fn process(input: &str) -> Result<String, Error> {
    input
        .lines()
        .try_fold(0u32, |acc, line| {
            let entries: SmallVec<[i32; 10]> = line
                .split_whitespace()
                .map(str::parse::<i32>)
                .map(|r| r.map_err(Error::from))
                .try_collect()?;

            if entries.is_empty() {
                return Err(Error::NoReportInLine);
            }

            if count_errors(&entries) <= 1 {
                Ok(acc + 1)
            } else {
                Ok(acc)
            }
        })
        .map(|count| count.to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        process("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n"),
        Ok("4".to_string())
    );
}
