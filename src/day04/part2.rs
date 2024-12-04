#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("parse :: empty input")]
    EmptyInput,
    #[error("parse :: non-uniform lines")]
    NonUniformLines,
}

#[derive(Clone, Copy, Debug)]
struct IVec2(isize, isize);

impl std::ops::Add for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: Self) -> Self::Output {
        IVec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        IVec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl IVec2 {
    fn into_index(self, width: usize, height: usize) -> Option<usize> {
        if (0..width as isize).contains(&self.0) && (0..height as isize).contains(&self.1) {
            Some(self.0 as usize + self.1 as usize * width)
        } else {
            None
        }
    }
}

const PATTERN: [IVec2; 4] = [IVec2(-1, -1), IVec2(1, -1), IVec2(-1, 1), IVec2(1, 1)];

const IDENT: [[char; 4]; 4] = [
    ['M', 'M', 'S', 'S'],
    ['S', 'M', 'S', 'M'],
    ['S', 'S', 'M', 'M'],
    ['M', 'S', 'M', 'S'],
];

pub fn process(input: &str) -> Result<String, Error> {
    let lines = input.lines();

    let width = lines
        .clone()
        .map(str::len)
        .map(Some)
        .reduce(|acc, line| match acc {
            Some(len) if len == line.unwrap() => acc,
            _ => None,
        })
        .ok_or(Error::EmptyInput)?
        .ok_or(Error::NonUniformLines)?;
    if width == 0 {
        return Err(Error::EmptyInput);
    }
    let height = lines.clone().count();

    let buf: Vec<char> = lines.flat_map(str::chars).collect();

    let mut sum = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let pos = IVec2(x as isize, y as isize);
            if buf[pos
                .into_index(width, height)
                .expect("should always be in bounds")]
                != 'A'
            {
                continue;
            }
            for ident in IDENT {
                sum += std::iter::zip(PATTERN, ident).all(|(offset, character)| {
                    match (pos + offset).into_index(width, height) {
                        Some(idx) => buf[idx] == character,
                        None => false,
                    }
                }) as u32;
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(process("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"), Ok("9".to_string()));
}
