#![crate_name = "histogram_minimizer"]
#![crate_type = "lib"]

//! One dimentional histogram minimizer.

pub mod histogram_minimizer;
pub use histogram_minimizer::minimize;
pub mod pc;
