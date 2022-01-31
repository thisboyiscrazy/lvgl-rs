mod arc;
mod bar;
mod gauge;
mod label;
use crate::core::NativeObject;

include!(concat!(env!("OUT_DIR"), "/generated-widgets.rs"));

pub use arc::*;
pub use bar::*;
pub use gauge::*;
pub use label::*;
