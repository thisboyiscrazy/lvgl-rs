use core::ptr;
use alloc::boxed::Box;

use crate::style::{Style, Align, Part, State};

/// Represents a native LVGL object
pub trait NativeObject {
    /// Provide common way to access to the underlying native object pointer.
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t>;
}

/// Generic LVGL object.
///
/// This is the parent object of all widget types. It stores the native LVGL raw pointer.
pub struct Obj {
    raw: ptr::NonNull<lvgl_sys::lv_obj_t>,
}

impl NativeObject for Obj {
    fn raw(&self) -> ptr::NonNull<lvgl_sys::lv_obj_t> {
        self.raw
    }
}

/// A wrapper for all LVGL common operations on generic objects.
pub trait Widget: NativeObject {
    /// Construct an instance of the object from a raw pointer.
    ///
    /// # Safety
    /// Provided the LVGL library can allocate memory this should be safe.
    ///
    unsafe fn from_raw(raw_pointer: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self;

    fn add_style(&self, style: Style, part: Part, state: State) {
        let part: lvgl_sys::lv_part_t = part.into();
        let selector = part | state.bits() as u32;

        unsafe {
            lvgl_sys::lv_obj_add_style(
                self.raw().as_mut(),
                Box::into_raw(style.raw),
                selector
            );
        };
    }

    fn set_pos(&mut self, x: i16, y: i16) {
        unsafe {
            lvgl_sys::lv_obj_set_pos(
                self.raw().as_mut(),
                x as lvgl_sys::lv_coord_t,
                y as lvgl_sys::lv_coord_t,
            );
        }
    }

    fn set_size(&mut self, w: i16, h: i16) {
        unsafe {
            lvgl_sys::lv_obj_set_size(
                self.raw().as_mut(),
                w as lvgl_sys::lv_coord_t,
                h as lvgl_sys::lv_coord_t,
            );
        }
    }

    fn set_width(&mut self, w: u32) {
        unsafe {
            lvgl_sys::lv_obj_set_width(
                self.raw().as_mut(),
                w as lvgl_sys::lv_coord_t
            );
        }
    }

    fn set_height(&mut self, h: u32) {
        unsafe {
            lvgl_sys::lv_obj_set_height(
                self.raw().as_mut(),
                h as lvgl_sys::lv_coord_t
            );
        }
    }

    fn align_to<C>(&mut self, base: &C, align: Align, x_mod: i32, y_mod: i32)
    where
        C: NativeObject,
    {
        unsafe {
            lvgl_sys::lv_obj_align_to(
                self.raw().as_mut(),
                base.raw().as_ptr(),
                align.into(),
                x_mod as lvgl_sys::lv_coord_t,
                y_mod as lvgl_sys::lv_coord_t,
            );
        }
    }
}

impl Widget for Obj {
    unsafe fn from_raw(raw: ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
        Self { raw }
    }
}

impl Default for Obj {
    fn default() -> Self {
        let raw = unsafe { lvgl_sys::lv_obj_create(ptr::null_mut()) };
        let raw = ptr::NonNull::new(raw).expect("OOM");
        Self { raw }
    }
}

macro_rules! define_object {
    ($item:ident) => {
        define_object!($item, event = (), part = $crate::Part);
    };
    ($item:ident, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $crate::Part);
    };
    ($item:ident, part = $part_type:ty) => {
        define_object!($item, event = (), part = $part_type);
    };
    ($item:ident, part = $part_type:ty, event = $event_type:ty) => {
        define_object!($item, event = $event_type, part = $part_type);
    };
    ($item:ident, event = $event_type:ty, part = $part_type:ty) => {
        pub struct $item {
            core: $crate::core::Obj,
        }

        unsafe impl Send for $item {}

        impl $item {
            pub fn on_event<F>(&mut self, event: $crate::core::Event, f: F)
            where
                F: FnMut(Self, $crate::core::Event, Option<$crate::core::Obj>),
            {
                $crate::core::add_event_cb(self, Some(event), f);
            }

            pub fn on_any_event<F>(&mut self, f: F)
            where
                F: FnMut(Self, $crate::core::Event, Option<$crate::core::Obj>),
            {
                $crate::core::add_event_cb(self, None, f);
            }
        }

        impl $crate::core::NativeObject for $item {
            fn raw(&self) -> core::ptr::NonNull<lvgl_sys::lv_obj_t> {
                self.core.raw()
            }
        }

        impl $crate::core::Widget for $item {
            unsafe fn from_raw(raw: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
                Self {
                    core: $crate::core::Obj::from_raw(raw),
                }
            }
        }
    };
}

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
