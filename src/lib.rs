#[cfg(feature = "unsafe_refcell_as_safe")]
use std::{cmp, fmt, ops};

#[cfg(feature = "unsafe_refcell_as_safe")]
pub mod cell;

#[cfg(not(feature = "unsafe_refcell_as_safe"))]
pub use std::cell as cell;
