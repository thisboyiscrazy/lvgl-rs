use embedded_graphics_core::{
    prelude::*,
    draw_target::DrawTarget,
};

use super::display::{PixelColor, Display};

use core::{
    sync::atomic::{AtomicBool, Ordering},
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr,
};

pub struct Lvgl<S> {
    // The phantom is used for two things:
    // 1) Prevent the user from building that struct
    // 2) Remove the Send and Sync trait with the pointer
    _phantom: PhantomData<(S, *mut cty::c_void)>,
}

// We can let another thread use the tick_inc/timer_handler functions, but one
// must be careful to not register buttons from another thread.
unsafe impl<S> Send for Lvgl<S> {}

impl<S> Lvgl<S> {
    pub fn ensure_init() {
        static HAS_INIT: AtomicBool = AtomicBool::new(false);
        if HAS_INIT.fetch_or(true, Ordering::Relaxed) {
            unsafe { lvgl_sys::lv_init(); }
        }
    }

    pub fn new() -> Self {
        Self::ensure_init();
        Self { _phantom: PhantomData }
    }

    pub fn register_logger<F>(&self, mut f: F)
    where
        F: FnMut(&str) -> () + 'static
    {
        static mut LOGGER: *mut () = ptr::null_mut();
        unsafe {
            LOGGER = mem::transmute(&mut f);
            lvgl_sys::lv_log_register_print_cb(Some(print_cb::<F>));
        }

        unsafe extern "C" fn print_cb<F>(str: *const cty::c_char)
        where
            F: FnMut(&str) -> () + 'static
        {
            let logger: *mut F = mem::transmute(LOGGER);
            let logger = logger.as_mut().unwrap();
            let str = cstr_core::CStr::from_ptr(str);
            logger(str.to_string_lossy().as_ref());
        }
    }

    /// Pass in the drawing buffer. See https://docs.lvgl.io/master/porting/display.html
    /// PixelColor is aliased to the color type configured by lv_conf.h
    /// 1/10th of the screen size is recommended for the size.
    // Note that we take references, because we want to be able to take special
    // addresses (like DMA regions), or static buffers, or stack allocated buffers.
    pub fn register_display<T: DrawTarget<Color = PixelColor> + OriginDimensions>(
        &self,
        draw_buffer: &'static mut [MaybeUninit<PixelColor>],
        display: T
    ) -> Display<T, S>
    {
        Display::new(draw_buffer, display)
    }

    pub fn irq_tick_updater(&self) -> IrqTickUpdater {
        IrqTickUpdater::new()
    }

    /// Call this with good accuracy to inform LVGL about time
    pub fn tick_inc(&mut self, millis_since_last_tick: u32) {
        unsafe {
            lvgl_sys::lv_tick_inc(millis_since_last_tick)
        }
    }

    /// Call this at least every few milliseconds to run LVGL tasks
    /// `app_state` will be provided to registered callbacks.
    pub fn timer_handler(&mut self, app_state: &mut S) {
        unsafe {
            assert!(APP_STATE.is_null(), "timer_handler() called recursively");
            APP_STATE = mem::transmute(app_state);

            lvgl_sys::lv_timer_handler();

            APP_STATE = core::ptr::null_mut();
        }
    }
}

// The type here doesn't really matter. We don't know it in advance.  We use a
// global variable as opposed to something in a struct, because we would
// otherwise have to save an extra reference for each callback that we register.
// This cost memory for no good reason as we _have_ to operate with a singleton anyways.
// This is because the lvgl timer_handler() doesn't take any argument.
static mut APP_STATE: *mut () = ptr::null_mut();

// This gives a lifetime to the app state reference.
// it shouldn't not be kept for longer than the duration of the callback
pub(crate) struct AppState<S> {
    // No Send/Sync because the lifetime is bounded by timer_handler().
    _phantom: PhantomData<*mut S>,
}

impl<S> AppState<S> {
    pub(crate) fn global() -> Self {
        Self { _phantom: PhantomData }
    }

    pub(crate) fn as_mut(&mut self) -> &mut S {
        unsafe {
            let app_state: *mut S = mem::transmute(APP_STATE);
            app_state.as_mut().expect("APP_STATE accessed outside of timer_handler")
        }
    }
}

pub struct IrqTickUpdater {
    _phantom: PhantomData<()>,
}

impl IrqTickUpdater {
    fn new() -> Self {
        Self { _phantom: PhantomData }
    }

    /// This function is safe to call while lvgl.timer_handler() is running
    pub fn inc(&mut self, millis_since_last_tick: u32) {
        unsafe {
            lvgl_sys::lv_tick_inc(millis_since_last_tick)
        }
    }
}
