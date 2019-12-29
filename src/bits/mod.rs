//! A module containing structs and functions
//! that help the user avoid bit-fiddling.

mod bit_writer;
mod biterator;
mod helpers;
pub use bit_writer::BitWriter;
pub use biterator::Biterator;
pub use helpers::{get_bit, set_bit};
