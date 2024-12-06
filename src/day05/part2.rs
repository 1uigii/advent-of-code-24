use std::{array, ops::Index};

use itertools::Itertools;
use smallvec::SmallVec;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("parse :: expected number, found newline")]
    ExpectedNumber,
    #[error("parse :: expected divider")]
    ExpectedDivider,
    #[error("parse :: {0}")]
    NumberParseError(#[from] std::num::ParseIntError),
    #[error("parse :: expected update definition")]
    ExpectedUpdateDefinition,
}

#[derive(Debug)]
struct OrderingMap([SmallVec<[u8; 20]>; u8::MAX as usize]);

impl Index<u8> for OrderingMap {
    type Output = [u8];

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl OrderingMap {
    fn new() -> OrderingMap {
        OrderingMap(array::from_fn(|_| SmallVec::new()))
    }

    fn insert(&mut self, before: u8, after: u8) {
        self.0[after as usize].push(before);
    }
}

struct OrderingMask<'o> {
    ord_map: &'o OrderingMap,
    allowed: [u8; u8::MAX as usize],
}

impl<'o> OrderingMask<'o> {
    fn new(ord_map: &'o OrderingMap) -> OrderingMask<'o> {
        OrderingMask {
            ord_map,
            allowed: [0; u8::MAX as usize],
        }
    }

    fn push(&mut self, num: u8) -> bool {
        self.ord_map[num]
            .iter()
            .for_each(|&idx| self.allowed[idx as usize] += 1);

        self.allowed[num as usize] == 0
    }

    fn pop(&mut self, num: u8) {
        self.ord_map[num]
            .iter()
            .for_each(|&idx| self.allowed[idx as usize] -= 1);
    }

    fn is_allowed(&self, num: u8) -> bool {
        self.allowed[num as usize] == 0
    }
}

fn parse_order_expression(input: &str) -> Result<(u8, u8), Error> {
    let (before, after) = input.split_once('|').ok_or(Error::ExpectedDivider)?;
    Ok((before.parse()?, after.parse()?))
}

fn parse_ordering<'s, I: Iterator<Item = &'s str>>(input: I) -> Result<OrderingMap, Error> {
    let mut order = OrderingMap::new();
    for expr in input {
        let (before, after) = parse_order_expression(expr)?;
        order.insert(before, after);
    }
    Ok(order)
}

pub fn process(input: &str) -> Result<String, Error> {
    let (order_def, update_def) = input
        .split_once("\n\n")
        .or(input.split_once("\r\n\r\n"))
        .ok_or(Error::ExpectedUpdateDefinition)?;
    let order = parse_ordering(order_def.lines())?;

    let mut sum = 0;
    for line in update_def.lines() {
        let nums: SmallVec<[u8; 10]> = line.split(',').map(str::parse).try_collect()?;

        let mut mask = OrderingMask::new(&order);
        // .all short-circuits, but every element should be propperly added
        if nums
            .iter()
            .fold(true, |allowed, &num| mask.push(num) && allowed)
        {
            continue;
        }

        let mut ord_nums: SmallVec<[u8; 10]> = SmallVec::with_capacity(nums.len());
        let mut used: SmallVec<[bool; 10]> = smallvec::smallvec![false; nums.len()];

        let mut idx = 0;
        while !used.iter().all(|&b| b) {
            if !used[idx % nums.len()] && mask.is_allowed(nums[idx % nums.len()]) {
                used[idx % nums.len()] = true;
                mask.pop(nums[idx % nums.len()]);
                ord_nums.push(nums[idx % nums.len()]);
            }
            idx += 1;
        }

        sum += ord_nums[ord_nums.len() / 2] as u32;
    }

    Ok(sum.to_string())
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(
        process(
            "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
        ),
        Ok("123".to_string())
    )
}
