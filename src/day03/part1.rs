use std::convert::Infallible;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error(transparent)]
pub struct Error(Infallible);

fn parse_two_pair_tuple(input: &str) -> Option<u32> {
    let rest = match input.split_once('(') {
        Some(("", rest)) => rest,
        _ => return None,
    };
    let (first, rest) = rest.split_once(',')?;
    let (second, _) = rest.split_once(')')?;

    Some(first.parse::<u32>().ok()? * second.parse::<u32>().ok()?)
}

pub fn process(input: &str) -> Result<String, Error> {
    let mut rest = input;
    let mut sum = 0;

    while let Some((_junk, r)) = rest.split_once("mul") {
        rest = r;
        if let Some(mul) = parse_two_pair_tuple(rest) {
            sum += mul;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(parse_two_pair_tuple("(12,32)"), Some(12 * 32));
    assert_eq!(parse_two_pair_tuple("(12, 32)"), None);
    assert_eq!(parse_two_pair_tuple("(12 32)"), None);
    assert_eq!(parse_two_pair_tuple("12, 32)"), None);

    assert_eq!(
        process("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        Ok("161".to_string())
    )
}
