use itertools::Itertools;
use smallvec::SmallVec;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("parse :: no report in line")]
    NoReportInLine,
    #[error("parse :: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

mod count {
    fn errors_ext<const B: i32, const E: i32>(beg: usize, rest: usize, slice: &[i32]) -> u32 {
        if rest >= slice.len() {
            return 0;
        }

        if (B..=E).contains(&(slice[beg] - slice[rest])) {
            errors_slice::<B, E>(&slice[rest..])
        } else {
            errors_ext::<B, E>(beg, rest + 1, slice) + 1
        }
    }

    fn errors_slice<const B: i32, const E: i32>(slice: &[i32]) -> u32 {
        for i in 1..slice.len() {
            if (B..=E).contains(&(slice[i - 1] - slice[i])) {
                continue;
            }

            let val1 = if i == 1 {
                errors_slice::<B, E>(&slice[1..])
            } else {
                errors_ext::<B, E>(i - 2, i, slice)
            };
            let val2 = errors_ext::<B, E>(i - 1, i + 1, slice);

            return u32::min(val1, val2) + 1;
        }
        0
    }

    #[allow(unused)]
    pub fn errors(slice: &[i32]) -> u32 {
        u32::min(errors_slice::<-3, -1>(slice), errors_slice::<1, 3>(slice))
    }
}

mod save {
    fn check_final<const B: i32, const E: i32>(slice: &[i32]) -> bool {
        for i in 1..slice.len() {
            if (B..=E).contains(&(slice[i - 1] - slice[i])) {
                continue;
            }

            return false;
        }
        true
    }

    fn check_ext<const B: i32, const E: i32>(beg: usize, rest: usize, slice: &[i32]) -> bool {
        if rest >= slice.len() {
            return true;
        }

        if (B..=E).contains(&(slice[beg] - slice[rest])) {
            check_final::<B, E>(&slice[rest..])
        } else {
            false
        }
    }

    fn check_slice<const B: i32, const E: i32>(slice: &[i32]) -> bool {
        for i in 1..slice.len() {
            if (B..=E).contains(&(slice[i - 1] - slice[i])) {
                continue;
            }

            return if i == 1 {
                check_final::<B, E>(&slice[1..])
            } else {
                check_ext::<B, E>(i - 2, i, slice)
            } || check_ext::<B, E>(i - 1, i + 1, slice);
        }
        true
    }

    pub fn check_errors(slice: &[i32]) -> bool {
        check_slice::<-3, -1>(slice) || check_slice::<1, 3>(slice)
    }
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

            if save::check_errors(&entries) {
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
