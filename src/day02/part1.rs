use itertools::Itertools;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("syntax :: no report in line")]
    NoReportInLine,
    #[error("parse :: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

pub fn process(input: &str) -> Result<String, Error> {
    input
        .lines()
        .filter_map(|line| {
            let mut entries = line.split_whitespace().map(str::parse::<u32>);
            let first = match entries.next() {
                Some(Ok(n)) => n as i32,
                Some(Err(e)) => return Some(Err(Error::from(e))),
                None => return Some(Err(Error::NoReportInLine)),
            };
            let second = match entries.next() {
                Some(Ok(n)) => n as i32,
                Some(Err(e)) => return Some(Err(Error::from(e))),
                None => return Some(Err(Error::NoReportInLine)),
            };

            match second - first {
                1..=3 => entries
                    .try_fold(Ok(second), |prev, curr| {
                        match prev
                            .and_then(|prev| Ok((curr?, prev)))
                            .map(|(curr, prev)| (curr as i32, curr as i32 - prev))
                        {
                            Ok((curr, 1..=3)) => Some(Ok(curr)),
                            Ok(..) => None,
                            Err(e) => Some(Err(e)),
                        }
                    })
                    .map(|v| v.map(|_| ())),
                -3..=-1 => entries
                    .try_fold(Ok(second), |prev, curr| {
                        match prev
                            .and_then(|prev| Ok((curr?, prev)))
                            .map(|(curr, prev)| (curr as i32, curr as i32 - prev))
                        {
                            Ok((curr, -3..=-1)) => Some(Ok(curr)),
                            Ok(..) => None,
                            Err(e) => Some(Err(e)),
                        }
                    })
                    .map(|v| v.map(|_| ())),
                _ => None,
            }
        })
        .fold_ok(0u32, |acc, _| acc + 1)
        .map(|count| count.to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        process("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n"),
        Ok("2".to_string())
    );
}
