mod arc;
mod bar;
mod gauge;
mod label;

include!(concat!(env!("OUT_DIR"), "/generated-widgets.rs"));

use crate::NativeObject;
pub use arc::*;
pub use bar::*;
pub use gauge::*;
pub use label::*;
