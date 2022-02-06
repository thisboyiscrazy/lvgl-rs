use super::Obj;
use core::ptr;
use core::ops::{Deref, DerefMut};
use alloc::boxed::Box;
use super::Display;

pub struct Screen<C: 'static> {
    pub(crate) obj: Obj<C>,
    // This gets passed to callbacks.
    // The outer Box is so that we know the address immedately.
    // The option is because the user will most likely want to create an initial
    // application state with widgets that need the screen to be instantiated,
    // and so it's a chicken and egg problem.
    _context: Box<Option<C>>,
}

impl<C: 'static> Screen<C> {
    pub fn new<D>(_display: &Display<D>) -> Self {
        unsafe {
            // This gets passed to callbacks.
            // The outer Box is so that we know the address immediately, and it shouldn't change.
            // The option is because the user will most likely want to create an initial
            // application state with widgets that need the screen to be instantiated,
            // and so it's a chicken and egg problem.
            let mut context = Box::new(None);

            let context_ptr = ptr::NonNull::new_unchecked(context.as_mut() as *mut _);

            let obj = lvgl_sys::lv_obj_create(core::ptr::null_mut());
            let obj = Obj::from_raw(obj.as_mut().expect("OOM"), context_ptr);
            Self { obj, _context: context }
        }
    }
}

impl<S> Deref for Screen<S> {
    type Target = Obj<S>;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}

impl<S> DerefMut for Screen<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.obj
    }
}
