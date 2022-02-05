use core::ops::{Deref, DerefMut};
use core::marker::PhantomData;
use lvgl_sys::{lv_coord_t, lv_obj_t};

use core::ptr;

use crate::style::Align;
//use alloc::boxed::Box;
use crate::{
    //style::{Style, Align, Part, State},
    core::Event,
    core::event::add_event_cb,
};

/// Base LVGL object. S is the AppState that we provide to the callbacks
/// The lifetime of the object depends on the lifetime of its parent.
/// (in lvgl, deleting an object deletes all its children)
/// We put the lifetime on the user_context, but it's slightly semantically wrong.
pub struct Obj<'parent, C> {
    // pub so that the user can use lvgl_sys functions directly
    pub raw: &'static mut lv_obj_t,
    // We want a stable pointer.
    pub(crate) context: ptr::NonNull<Option<C>>, // Should be a refcell? Or a &mut? Lost patience trying to make things work.
    _phantom: PhantomData<&'parent Self>
}

unsafe impl<'p, C> Send for Obj<'p, C> {}

impl<'p, C> Obj<'p, C> {
    pub fn from_raw(raw: &'static mut lv_obj_t, context: ptr::NonNull<Option<C>>) -> Self {
        Self { raw, context, _phantom: PhantomData }
    }
}

pub trait ObjExt<'p, C: 'static>: Deref<Target = Obj<'p, C>> + DerefMut + Sized {
    /// Register an event callback, for a specific event
    fn on_event(mut self, event: Event, mut f: impl FnMut(Option<&mut C>)) -> Self {
        let mut context = self.context;
        add_event_cb(self.raw, Some(event), move |_e, _current_target, _child| {
            let context = unsafe { context.as_mut().as_mut() };
            f(context)
        });
        self
    }

    /// Register an event callback, receiving all events.
    fn on_any_event(mut self, mut f: impl FnMut(Option<&mut C>, Event)) -> Self {
        let mut context = self.context;
        add_event_cb(self.raw, None, move |e, _current_target, _child| {
            let context = unsafe { context.as_mut().as_mut() };
            f(context, e)
        });
        self
    }

    /// Typically used to create a nested object.
    fn nest<'a, R>(mut self, f: impl FnOnce(&mut Self) -> R) -> Self
    where C: 'a
    {
        // we can't create the object and then use lv_obj_set_parent() to parent it.
        // We have to pass the correct parent upfront when creating a child.
        // This is why we use a closure. And that's also why we don't care about the return value
        f(&mut self);
        self
    }

    fn align<'a>(mut self, base: &impl ObjExt<'a, C>, align: Align,
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
        pub struct $item<'p, C> {
            pub(crate) obj: Obj<'p, C>
        }

        impl<'p, C> core::ops::Deref for $item<'p, C> {
            type Target = Obj<'p, C>;

            fn deref(&self) -> &Self::Target {
                &self.obj
            }
        }

        impl<'p, C> core::ops::DerefMut for $item<'p, C> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.obj
            }
        }
    };
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

