use embedded_graphics_core::{
    prelude::*,
    draw_target::DrawTarget,
};

use super::display::{PixelColor, Display};

use core::{
    sync::atomic::{AtomicBool, Ordering},
    marker::PhantomData,
    mem::MaybeUninit,
};

pub struct Lvgl {
    // The phantom is used for two things:
    // 1) Prevent the user from building that struct
    // 2) Remove the Send and Sync trait with the pointer
    _phantom: PhantomData<*mut cty::c_void>,
}

// We can let another thread use the tick_inc/timer_handler functions, but one
// must be careful to not register buttons from another thread.
unsafe impl Send for Lvgl {}

impl Lvgl {
    pub(crate) fn ensure_init() {
        static HAS_INIT: AtomicBool = AtomicBool::new(false);
        if HAS_INIT.fetch_or(true, Ordering::Relaxed) {
            unsafe { lvgl_sys::lv_init(); }
        }
    }

    pub fn new() -> Self {
        Self::ensure_init();
        Self { _phantom: PhantomData }
    }

    /// Pass in the drawing buffer. See https://docs.lvgl.io/master/porting/display.html
    /// PixelColor is aliased to the color type configured by lv_conf.h
    /// 1/10th of the screen size is recommended for the size.
    // Note that we take references, because we want to be able to take special
    // addresses (like DMA regions), or static buffers, or stack allocated buffers.
    pub fn register_display<'a, T: DrawTarget<Color = PixelColor> + OriginDimensions>(
        draw_buffer: &'a mut [MaybeUninit<PixelColor>],
        display: T
    ) -> Display<'a, T>
    {
        Display::new(draw_buffer, display)
    }

    /// Call this with good accuracy to inform LVGL about time
    pub fn tick_inc(&mut self, millis_since_last_tick: u32) {
        unsafe {
            lvgl_sys::lv_tick_inc(millis_since_last_tick)
        }
    }

    /// Call this at least every few milliseconds to run LVGL tasks
    pub fn timer_handler(&mut self) {
        unsafe {
            lvgl_sys::lv_timer_handler();
        }
    }
}

