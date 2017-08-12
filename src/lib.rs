#[cfg(feature = "unchecked_refcell")]
use std::{cmp, fmt, ops};

#[cfg(feature = "unchecked_refcell")]
pub mod cell;

#[cfg(not(feature = "unchecked_refcell"))]
pub use std::cell as cell;
