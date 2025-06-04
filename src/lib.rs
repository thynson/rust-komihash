#[cfg(test)]
mod tests;
mod utils;
pub mod v4;
pub mod v5;

pub use v5::{komihash, KomiHasher, Komirand, StreamedKomihash};
