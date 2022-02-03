use core::marker::PhantomData;
//use alloc::boxed::Box;
use crate::{
    //style::{Style, Align, Part, State},
    core::Event,
    core::event::add_event_cb,
};

/// Base LVGL object. S is the AppState that we provide to the callbacks
pub struct Obj<'parent, S> {
    pub(crate) raw: &'static mut lvgl_sys::lv_obj_t,
    _phantom: PhantomData<(&'parent Self, S)>,
}

unsafe impl<'a, S> Send for Obj<'a,S> {}

/// A wrapper for all LVGL common operations on generic objects.
impl<'a, S> Obj<'a, S> {
    pub fn from_raw(raw: &'static mut lvgl_sys::lv_obj_t) -> Self {
        Self { raw, _phantom: PhantomData }
    }

    /// Register an event callback, for a specific event. The function will be
    /// called with a child object if the object triggering the event is not the
    /// current object.
    pub fn on_event<'b, F>(&mut self, event: Event, mut f: F)
    where
        F: FnMut(&'b mut S, Option<Obj<'b, S>>) + 'static,
        S: 'static,
    {
        let f = move |s, _e, c| f(s, c);
        add_event_cb(self, Some(event), f);
    }

    /// Register an event callback, receiving all events. The function will be
    /// called with a child object if the object triggering the event is not the
    /// current object.
    pub fn on_any_event<'b, F>(&mut self, f: F)
    where
        F: FnMut(&mut S, Event, Option<Obj<'b, S>>) + 'static,
        S: 'static,
    {
        add_event_cb(self, None, f);
    }


/*
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
    */
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

macro_rules! define_object {
    ($item:ident) => {
        pub struct $item<'parent, S> {
            obj: Obj<'parent, S>
        }

        impl<'a,S> core::ops::Deref for $item<'a,S> {
            type Target = Obj<'a, S>;

            fn deref(&self) -> &Self::Target {
                &self.obj
            }
        }

        impl<'a,S> core::ops::DerefMut for $item<'a,S> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.obj
            }
        }
    };
}
