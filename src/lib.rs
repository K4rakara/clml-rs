#[macro_use] pub(crate) extern crate lazy_static;
#[cfg(feature = "advanced")] pub(crate) extern crate mlua;
pub(crate) extern crate rand;
pub(crate) extern crate regex;

#[cfg(feature = "advanced")] pub(crate) mod advanced;
pub(crate) mod basic;
pub(crate) mod emoji;
pub(crate) mod replacements;

pub use basic::{ clml };
#[cfg(feature = "advanced")] pub use advanced::*;