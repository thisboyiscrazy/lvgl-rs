mod style;
pub use style::*;

mod align;
pub use align::*;

mod part;
pub use part::*;

mod state;
pub use state::*;

mod flag;
pub use flag::*;


// Adapted from https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
macro_rules! native_enum {
    ($native_type:ty,
        $(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy)]
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val as isize)?,)*
        }

        impl core::convert::TryFrom<$native_type> for $name {
            type Error = ();

            fn try_from(v: $native_type) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as $native_type => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }

        impl From<$name> for $native_type {
            fn from(v: $name) -> Self {
                v as $native_type
            }
        }
    }
}

pub(crate) use native_enum;
