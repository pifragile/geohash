#![doc(html_root_url = "https://docs.rs/geohash/")]

//! # Geohash
//!
//! Geohash algorithm implementation in Rust. It encodes/decodes a
//! longitude-latitude tuple into/from a hashed string. You can find
//! more about geohash algorithm on [Wikipedia](https://en.wikipedia.org/wiki/Geohash)
//!
//! ## Usage
//! ```rust
//! extern crate geohash;
//!
//! use geohash::{encode, decode, neighbor, Direction};
//! use fixed::types::I64F64;
//!
//! fn main() -> Result<(), Box<geohash::GeohashError>> {
//!   let lon = I64F64::from_num(112.5584);
//!   let lat = I64F64::from_num(37.8324f64);
//!
//!   // decode a geohash
//!   let (lon, lat, _, _) = decode("ww8p1r4t8")?;
//!
//!   // find a neighboring hash
//!   let sw = neighbor("ww8p1r4t8", Direction::SW)?;
//!
//!   Ok(())
//! }
//! ```
//!

mod core;
mod error;
mod neighbors;

pub use crate::core::{decode, encode, neighbor, neighbors};
pub use crate::error::GeohashError;
pub use crate::neighbors::{Direction, Neighbors};
