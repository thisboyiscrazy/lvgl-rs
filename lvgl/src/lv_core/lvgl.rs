use embedded_graphics_core::{
    prelude::*,
    draw_target::DrawTarget,
};

use super::display::{PixelColor, Display};

use core::sync::atomic::{AtomicBool, Ordering};

// Initialize LVGL only once.
static LVGL_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub(crate) fn lvgl_ensure_init() {
    if LVGL_INITIALIZED
        .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        unsafe {
            lvgl_sys::lv_init();
        }
    }
}

pub struct Lvgl {}

impl Lvgl {
    pub fn new() -> Self {
        lvgl_ensure_init();
        Self {}
    }

    /// Pass in the drawing buffer. See https://docs.lvgl.io/master/porting/display.html
    /// PixelColor is aliased to the color type configured by lv_conf.h
    /// 1/10th of the screen size is recommended for the size.
    // Note that we take references, because we want to be able to take special
    // addresses (like DMA regions), or static buffers, or stack allocated buffers.
    pub fn add_display<'a, T: DrawTarget<Color = PixelColor> + OriginDimensions>(
        draw_buffer: &'a mut [PixelColor],
        display: T
    ) -> Display<'a, T>
    {
        Display::new(draw_buffer, display)
    }

    /// Call this with good accuracy so LVGL knows about time passing
    pub fn tick_inc(&mut self, millis_since_last_tick: u32) {
        unsafe {
            lvgl_sys::lv_tick_inc(millis_since_last_tick)
        }
    }

    /// Call this every few milliseconds to handle LVGL tasks
    pub fn timer_handler(&mut self) {
        unsafe {
            lvgl_sys::lv_timer_handler();
        }
    }
}
