//! # Tracked Memory
//! This crate provides a safe API for several things that are difficult
//! or unsafe to do in normal rust. It adds some overhead in order to
//! make things like uninitialzed arrays safe.
//!
//! This library is a work in progress. There are probably a lot of things
//! that would fit in this library that would be helpful. Any contributions
//! or suggestions are welcome.

#[cfg(test)]
mod tests;

pub mod uninitialized;
pub use uninitialized::SafeUninitializedVec;

pub mod might_own;
pub use might_own::MightOwn;
