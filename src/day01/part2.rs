#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("parse :: expected number, found newline")]
    ExpectedNumber,
    #[error("parse :: expected newline")]
    ExpectedNewline,
    #[error("parse :: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
}

pub fn find_amount(n: u32, sorted: &[u32]) -> u32 {
    match sorted.binary_search(&n) {
        Ok(center) => {
            // search partition begin
            let beg = sorted[..center]
                .iter()
                .rposition(|v| *v != n)
                .map(|v| v + 1)
                .unwrap_or(0);
            // search partition end
            let end = sorted[center..]
                .iter()
                .position(|v| *v != n)
                .unwrap_or(sorted.len() - center)
                + center;

            (end - beg) as u32
        }
        Err(..) => 0,
    }
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
    let (left, mut right) = result?;
    right.sort_unstable();

    let mut sum = 0;
    let mut mul = 0;
    let mut prev = 0;

    for left in left {
        if left == prev {
            mul += 1;
            continue;
        }
        sum += mul * prev * find_amount(prev, &right);

        prev = left;
        mul = 1;
    }

    if mul != 0 {
        sum += mul * prev * find_amount(prev, &right);
    }

    Ok(sum.to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        process("3   4\n4   3\n2   5\n1   3\n3   9\n3   3",),
        Ok("31".to_string())
    )
}
