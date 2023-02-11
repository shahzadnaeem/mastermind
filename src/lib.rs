#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod scores;

#[cfg(test)]
#[path = "./scores/tests.rs"]
mod tests;
