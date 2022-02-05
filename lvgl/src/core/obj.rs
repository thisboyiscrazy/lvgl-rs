use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use lvgl_sys::lv_coord_t;

use crate::style::Align;
//use alloc::boxed::Box;
use crate::{
    //style::{Style, Align, Part, State},
    core::Event,
    core::event::add_event_cb,
};

/// Base LVGL object. S is the AppState that we provide to the callbacks
/// The lifetime of the object depends on the lifetime of its parent.
pub struct Obj<'parent, S> {
    pub(crate) raw: &'static mut lvgl_sys::lv_obj_t,
    _phantom: PhantomData<(&'parent Self, S)>,
}

unsafe impl<'p, S> Send for Obj<'p,S> {}

impl<'p, S> Obj<'p, S> {
    pub fn from_raw(raw: &'static mut lvgl_sys::lv_obj_t) -> Self {
        Self { raw, _phantom: PhantomData }
    }
}

pub trait ObjExt<'p, S: 'static>: Deref<Target = Obj<'p,S>> + DerefMut + Sized {
    /// Register an event callback, for a specific event. The function will be
    /// called with a child object if the object triggering the event is not the
    /// current object.
    fn on_event<'a, F>(mut self, event: Event, mut f: F) -> Self
    where
        F: FnMut(&'a mut S, Option<Obj<'a, S>>) + 'static,
        S: 'static,
    {
        let f = move |s, _e, c| f(s, c);
        add_event_cb(&mut *self, Some(event), f);
        self
    }

    /// Register an event callback, receiving all events. The function will be
    /// called with a child object if the object triggering the event is not the
    /// current object.
    fn on_any_event<'a, F>(mut self, f: F) -> Self
    where
        F: FnMut(&mut S, Event, Option<Obj<'a, S>>) + 'static,
        S: 'static,
    {
        add_event_cb(&mut *self, None, f);
        self
    }

    /// Typically used to create a nested object.
    fn nest<'a, R>(mut self, f: impl FnOnce(&mut Self) -> R) -> Self
    where S: 'a
    {
        // we can't create the object and then use lv_obj_set_parent() to parent it.
        // We have to pass the correct parent upfront when creating a child.
        // This is why we use a closure. And that's also why we don't care about the return value
        f(&mut self);
        self
    }

    fn align<'a>(mut self, base: &impl ObjExt<'a, S>, align: Align,
                 x_mod: lv_coord_t, y_mod: lv_coord_t) -> Self
    {
        unsafe { lvgl_sys::lv_obj_align_to(&mut *self.raw, base.raw, align.into(), x_mod, y_mod) };
        self
    }

    fn pos(mut self, x: lv_coord_t, y: lv_coord_t) -> Self {
        unsafe { lvgl_sys::lv_obj_set_pos(&mut *self.raw, x, y) };
        self
    }

    fn size(mut self, w: lv_coord_t, h: lv_coord_t) -> Self {
        unsafe { lvgl_sys::lv_obj_set_size(&mut *self.raw, w, h) };
        self
    }

    fn width(mut self, w: lv_coord_t) -> Self {
        unsafe { lvgl_sys::lv_obj_set_width(&mut *self.raw, w) };
        self
    }

    fn height(mut self, h: lv_coord_t) -> Self {
        unsafe { lvgl_sys::lv_obj_set_height(&mut *self.raw, h) };
        self
    }

}
impl<'p, S: 'static, T: Deref<Target = Obj<'p,S>> + DerefMut + Sized> ObjExt<'p,S> for T {}

macro_rules! define_object {
    ($item:ident) => {
        pub struct $item<'parent, S> {
            pub(crate) obj: Obj<'parent, S>
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

define_object!(Screen);



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

    */


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

