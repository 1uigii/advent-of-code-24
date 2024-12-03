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
    let mut do_active = true;

    while let Some((junk, r)) = rest.split_once("mul") {
        let do_fn = junk.rfind("do()");
        let do_not_fn = junk.rfind("don't()");
        match (do_fn, do_not_fn) {
            (None, None) => {}
            (None, Some(..)) => do_active = false,
            (Some(..), None) => do_active = true,
            (Some(do_fn), Some(do_not_fn)) => do_active = do_fn > do_not_fn,
        }
        rest = r;
        if !do_active {
            continue;
        }
        if let Some(mul) = parse_two_pair_tuple(rest) {
            sum += mul;
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        process("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        Ok("48".to_string())
    )
}
