pub mod v4;
pub mod v5;
mod utils;
#[cfg(test)]
mod tests;

pub use v5::{komihash, Komirand, KomiHasher, StreamedKomihash};