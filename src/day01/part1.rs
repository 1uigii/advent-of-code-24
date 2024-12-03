use std::iter;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("parse :: expected number, found newline")]
    ExpectedNumber,
    #[error("parse :: expected newline")]
    ExpectedNewline,
    #[error("parse :: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

pub fn process(input: &str) -> Result<String, Error> {
    let result: Result<(Vec<_>, Vec<_>), Error> = input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            let left = iter.next().ok_or(Error::ExpectedNumber)?.parse::<u32>()?;
            let right = iter.next().ok_or(Error::ExpectedNumber)?.parse::<u32>()?;
            match iter.next() {
                Some(..) => Err(Error::ExpectedNewline),
                None => Ok((left, right)),
            }
        })
        .collect();
    let (mut left, mut right) = result?;
    left.sort_unstable();
    right.sort_unstable();

    Ok(iter::zip(left, right)
        .map(|(l, r)| u32::abs_diff(l, r))
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        process("3   4\n4   3\n2   5\n1   3\n3   9\n3   3",),
        Ok("11".to_string())
    )
}
