#[macro_export]
macro_rules! day {
    () => {};

    ($day:literal) => {
        paste::paste! {
            pub mod [<day $day>] {
                pub mod part1;
                pub mod part2;

                #[derive(Default, Clone, Copy, Debug)]
                pub struct Part1<'s> (Option<&'s str>);
                #[derive(Default, Clone, Copy, Debug)]
                pub struct Part2<'s> (Option<&'s str>);

                impl $crate::Process for Part1<'_> {
                    fn process(&self) -> Result<String, $crate::Error> {
                        part1::process(self.0.unwrap_or(include_str!(concat!("day", stringify!($day), "/input"))))
                            .map_err(Error::from)
                            .map_err($crate::Error::from)
                    }
                }

                impl $crate::Process for Part2<'_> {
                    fn process(&self) -> Result<String, $crate::Error> {
                        part2::process(self.0.unwrap_or(include_str!(concat!("day", stringify!($day), "/input"))))
                            .map_err(Error::from)
                            .map_err($crate::Error::from)
                    }
                }

                #[derive(thiserror::Error, Debug)]
                pub enum Error {
                    #[error("part 1 :: {0}")]
                    Part1(#[from] part1::Error),
                    #[error("part 2 :: {0}")]
                    Part2(#[from] part2::Error),
                }
            }
        }
    };

    ($day:literal, $($other:literal),+) => {
        day!($day); // Process the first day
        day!($($other),+); // Process the rest of the days
    };
}

pub use day;
