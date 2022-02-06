use lvgl_sys::{lv_coord_t, lv_obj_t};

use core::{
    ops::{Deref, DerefMut},
    ptr,
};

use crate::{
    style::{Align, Flag, State},
    core::Event,
    core::event::add_event_cb,
};

/// Base LVGL object. C is the application context that we provide to the
/// callbacks The lifetime of the object depends on the lifetime of its parent.
/// (in lvgl, deleting an object deletes all its children).
/// We have not implemented lifetimes correctly at this point.
pub struct Obj<C> {
    // pub so that the user can use lvgl_sys functions directly
    pub raw: &'static mut lv_obj_t,
    // We want a stable pointer.
    pub(crate) context: ptr::NonNull<Option<C>>, // Should be a refcell? Or a &mut? Lost patience trying to make things work.
}

unsafe impl<C> Send for Obj<C> {}

impl<C> Obj<C> {
    pub fn from_raw(raw: &'static mut lv_obj_t, context: ptr::NonNull<Option<C>>) -> Self {
        Self { raw, context }
    }
}

pub trait ObjExt<C: 'static>: Deref<Target = Obj<C>> + DerefMut + Sized {
    fn context(&mut self) -> &mut Option<C> {
        unsafe { self.context.as_mut() }
    }

    fn apply(mut self, f: impl FnOnce(&mut Self)) -> Self {
        // We don't care about the return value. It's typically &mut self
        f(&mut self);
        self
    }

    /// Register an event callback, for a specific event
    fn on_event(&mut self, event: Event, mut f: impl FnMut(&mut C) + 'static) -> &mut Self {
        let mut context = self.context;
        add_event_cb(self.raw, Some(event), move |_e, _current_target, _child| {
            let context = unsafe { context.as_mut().as_mut() };
            let context = context.expect("screen.context() must be set");
            f(context)
        });
        self
    }

    /// Register an event callback, receiving all events.
    fn on_any_event(&mut self, mut f: impl FnMut(&mut C, Event) + 'static) -> &mut Self {
        let mut context = self.context;
        add_event_cb(self.raw, None, move |e, _current_target, _child| {
            let context = unsafe { context.as_mut().as_mut() };
            let context = context.expect("screen.context() must be set");
            f(context, e)
        });
        self
    }

    fn align_to(&mut self, base: &impl ObjExt<C>, align: Align,
             x_mod: lv_coord_t, y_mod: lv_coord_t) -> &mut Self
    {
        unsafe { lvgl_sys::lv_obj_align_to(&mut *self.raw, base.raw, align.into(), x_mod, y_mod) };
        self
    }

    fn set_pos(&mut self, x: lv_coord_t, y: lv_coord_t) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_set_pos(&mut *self.raw, x, y) };
        self
    }

    fn set_size(&mut self, w: lv_coord_t, h: lv_coord_t) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_set_size(&mut *self.raw, w, h) };
        self
    }

    fn set_width(&mut self, w: lv_coord_t) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_set_width(&mut *self.raw, w) };
        self
    }

    fn set_height(&mut self, h: lv_coord_t) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_set_height(&mut *self.raw, h) };
        self
    }

    fn add_flag(&mut self, flag: Flag) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_add_flag(&mut *self.raw, flag.bits()) };
        self
    }

    fn clear_flag(&mut self, flag: Flag) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_clear_flag(&mut *self.raw, flag.bits()) };
        self
    }

    fn has_flag(&self, flag: Flag) -> bool {
        unsafe { lvgl_sys::lv_obj_has_flag(&*self.raw, flag.bits()) }
    }

    fn add_state(&mut self, state: State) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_add_state(&mut *self.raw, state.bits()) };
        self
    }

    fn clear_state(&mut self, state: State) -> &mut Self {
        unsafe { lvgl_sys::lv_obj_clear_state(&mut *self.raw, state.bits()) };
        self
    }

    fn get_state(&self) -> State {
        let state = unsafe { lvgl_sys::lv_obj_get_state(&*self.raw) };
        State::from_bits(state).unwrap()
    }

    fn has_state(&self, state: State) -> bool {
        unsafe { lvgl_sys::lv_obj_has_state(&*self.raw, state.bits()) }
    }
}

impl<C: 'static, T: Deref<Target = Obj<C>> + DerefMut + Sized> ObjExt<C> for T {}

macro_rules! define_object {
    ($item:ident) => {
        pub struct $item<C> {
            pub(crate) obj: Obj<C>
        }

        impl<C> core::ops::Deref for $item<C> {
            type Target = Obj<C>;

            fn deref(&self) -> &Self::Target {
                &self.obj
            }
        }

        impl<C> core::ops::DerefMut for $item<C> {
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
