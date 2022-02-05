use alloc::boxed::Box;
use crate::core::InputDeviceEvent;

use core::{
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ptr,
    ops::{Deref, DerefMut},
};

use embedded_graphics_core::{
    prelude::*,
    draw_target::DrawTarget,
    primitives::Rectangle,
};

use super::{Obj, Screen};

// This gives us "pub type PixelColor = embedded_graphics_core::pixel_color::Rgb565;" with the right color
include!(concat!(env!("OUT_DIR"), "/generated-color-settings.rs"));

/// `Display` represents a display for Lvgl
/// Limitations:
/// * No async drawing, no double buffering
/// * No color conversion. lv_conf.h specifies what embedded_graphics display you can use
/// * Resources are leaked when `Display` is dropped

pub struct Display<T: DrawTarget<Color = PixelColor> + OriginDimensions, S> {
    // We box because we need stable addresses
    display: Box<T>,
    disp: &'static mut lvgl_sys::lv_disp_t,
    _phantom: PhantomData<S>,
}

impl<T: DrawTarget<Color = PixelColor> + OriginDimensions, S> Display<T, S> {
    pub(crate) fn new(
        // We don't need 'static. We could just create a generic lifetime.
        draw_buffer: &'static mut [MaybeUninit<PixelColor>],
        display: T,
    ) -> Self {
        // We box the display to pin its address. This way, we can operate on it in the callback.
        let mut display = Box::new(display);

        let mut disp_draw_buf = unsafe {
            let mut disp_draw_buf = MaybeUninit::uninit();
            lvgl_sys::lv_disp_draw_buf_init(
                disp_draw_buf.as_mut_ptr(),
                draw_buffer.as_mut_ptr() as *mut cty::c_void,
                ptr::null_mut(),
                draw_buffer.len() as u32,
            );
            Box::new(disp_draw_buf.assume_init())
        };

        let mut disp_drv = unsafe {
            let mut disp_drv = MaybeUninit::uninit();
            lvgl_sys::lv_disp_drv_init(disp_drv.as_mut_ptr());
            let mut disp_drv = Box::new(disp_drv.assume_init());
            disp_drv.draw_buf = disp_draw_buf.as_mut();
            disp_drv.hor_res = display.size().width as lvgl_sys::lv_coord_t;
            disp_drv.ver_res = display.size().height as lvgl_sys::lv_coord_t;
            disp_drv.flush_cb = Some(display_flush_cb::<T>);
            disp_drv.user_data = mem::transmute(display.as_mut());
            disp_drv
        };

        let disp = unsafe {
            lvgl_sys::lv_disp_drv_register(disp_drv.as_mut())
                .as_mut().unwrap()
        };

        // If we wanted to cleanup resources on drop, these would have to be freed.
        Box::into_raw(disp_draw_buf);
        Box::into_raw(disp_drv);

        Self {
            disp,
            display,
            _phantom: PhantomData,
        }
    }

    pub fn screen<'a>(&'a mut self) -> Screen<'a, S> {
        unsafe {
            let obj_ptr = lvgl_sys::lv_disp_get_scr_act(self.disp);
            let obj = Obj::from_raw(obj_ptr.as_mut().unwrap());
            Screen { obj }
        }
    }

    pub fn register_input_device<F, I>(&mut self, event_generator: F)
    where
        F: Fn(&mut S) -> I + 'static,
        I: InputDeviceEvent,
    {
        crate::core::input_device::register_input_device(self.disp, event_generator);
    }
}

impl<T: DrawTarget<Color = PixelColor> + OriginDimensions, S> Deref for Display<T, S> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.display.deref()
    }
}

impl<T: DrawTarget<Color = PixelColor> + OriginDimensions, S> DerefMut for Display<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.display.deref_mut()
    }
}



unsafe extern "C" fn display_flush_cb<T>(
    disp_drv: *mut lvgl_sys::lv_disp_drv_t,
    area: *const lvgl_sys::lv_area_t,
    color_p: *mut lvgl_sys::lv_color_t,
) where
    T: DrawTarget<Color = PixelColor>,
{
    // In the `std` world we would make sure to capture panics here and make them not escape across
    // the FFI boundary. Since this library is focused on embedded platforms, we don't
    // have an standard unwinding mechanism to rely upon.
    let disp_drv = disp_drv.as_mut().unwrap();
    let display_ptr: *mut T = mem::transmute(disp_drv.user_data);
    let display = display_ptr.as_mut().unwrap();

    let area = Rectangle::with_corners(
        ((*area).x1 as i32, (*area).y1 as i32).into(),
        ((*area).x2 as i32, (*area).y2 as i32).into()
    );

    let num_pixels = (area.size.width * area.size.height) as usize;
    let colors = core::slice::from_raw_parts(color_p as *const PixelColor, num_pixels);
    let colors = colors
        .into_iter()
        .cloned();

    // Ignore errors
    let _ = display.fill_contiguous(&area, colors);

    // Indicate to LVGL that we are ready with the flushing
    // Note that we could do something async if we were to use something like DMA and two buffers.
    lvgl_sys::lv_disp_flush_ready(disp_drv);
}
