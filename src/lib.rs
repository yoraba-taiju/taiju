// https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs

mod scenes;
mod donut;

pub use crate::scenes::stage::prelude::*;
