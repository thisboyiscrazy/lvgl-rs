use crate::lv_core::style::Style;
use crate::Align;
use core::ptr;
use alloc::boxed::Box;

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
            core: $crate::Obj,
        }

        unsafe impl Send for $item {}

        impl $item {
            pub fn on_event<F>(&mut self, f: F, event: $crate::Event)
            where
                F: FnMut(Self, $crate::Event, Option<$crate::Obj>),
            {
                $crate::add_event_cb(self, f, Some(event));
            }

            pub fn on_any_event<F>(&mut self, f: F)
            where
                F: FnMut(Self, $crate::Event, Option<$crate::Obj>),
            {
                $crate::add_event_cb(self, f, None);
            }
        }

        impl $crate::NativeObject for $item {
            fn raw(&self) -> core::ptr::NonNull<lvgl_sys::lv_obj_t> {
                self.core.raw()
            }
        }

        impl $crate::Widget for $item {
            unsafe fn from_raw(raw: core::ptr::NonNull<lvgl_sys::lv_obj_t>) -> Self {
                Self {
                    core: $crate::Obj::from_raw(raw),
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

bitflags! {
    pub struct State: u16 {
        /// Normal, released
        const DEFAULT  = lvgl_sys::LV_STATE_DEFAULT as u16;
        /// Toggled or checked
        const CHECKED  = lvgl_sys::LV_STATE_CHECKED as u16;
        /// Focused via keypad or encoder or clicked via touchpad/mouse
        const FOCUSED  = lvgl_sys::LV_STATE_FOCUSED as u16;
        /// Focused via a keypad
        const FOCUS_KEY = lvgl_sys::LV_STATE_FOCUS_KEY as u16;
        /// Edit by an encoder
        const EDITED   = lvgl_sys::LV_STATE_EDITED as u16;
        /// Hovered by mouse (not supported now)
        const HOVERED  = lvgl_sys::LV_STATE_HOVERED as u16;
        /// Pressed
        const PRESSED  = lvgl_sys::LV_STATE_PRESSED as u16;
        /// SCrolled
        const SCROLLED = lvgl_sys::LV_STATE_SCROLLED as u16;
        /// Disabled or inactive
        const DISABLED = lvgl_sys::LV_STATE_DISABLED as u16;

        const USER_1 = lvgl_sys::LV_STATE_USER_1 as u16;
        const USER_2 = lvgl_sys::LV_STATE_USER_2 as u16;
        const USER_3 = lvgl_sys::LV_STATE_USER_3 as u16;
        const USER_4 = lvgl_sys::LV_STATE_USER_4 as u16;

        const ANY = lvgl_sys::LV_STATE_ANY as u16;
    }
}

/*
impl State {
    pub(crate) fn get_bits(&self) -> u16 {
        self.bits
    }
}
*/

impl Default for State {
    fn default() -> Self {
        Self::DEFAULT
    }
}

pub enum Part {
    /// A background like rectangle
    Main,
    /// The scrollbar(s)
    Scrollbar,
    /// Indicator, e.g. for slider, bar, switch, or the tick box of the checkbox
    Indicator,
    /// Like handle to grab to adjust the value
    Knob,
    /// Indicate the currently selected option or section,
    Selected,
    /// Used if the widget has multiple similar elements (e.g. table cells)
    Items,
    /// Ticks on scale e.g. for a chart or meter
    Ticks,
    /// Mark a specific place e.g. for text area's cursor or on a chart
    Cursor,
    /// Extension point for custom widgets
    CustomFirst,
    /// Special value can be used in some functions to target all parts
    Any,
}

impl Default for Part {
    fn default() -> Self {
        Self::Main
    }
}

impl Into<lvgl_sys::lv_part_t> for Part {
    fn into(self) -> lvgl_sys::lv_part_t {
        match self {
            Part::Main => lvgl_sys::LV_PART_MAIN,
            Part::Scrollbar => lvgl_sys::LV_PART_SCROLLBAR,
            Part::Indicator => lvgl_sys::LV_PART_INDICATOR,
            Part::Knob => lvgl_sys::LV_PART_KNOB,
            Part::Selected => lvgl_sys::LV_PART_SELECTED,
            Part::Items => lvgl_sys::LV_PART_ITEMS,
            Part::Ticks => lvgl_sys::LV_PART_TICKS,
            Part::Cursor => lvgl_sys::LV_PART_CURSOR,
            Part::CustomFirst => lvgl_sys::LV_PART_CUSTOM_FIRST,
            Part::Any => lvgl_sys::LV_PART_ANY,
        }
    }
}
