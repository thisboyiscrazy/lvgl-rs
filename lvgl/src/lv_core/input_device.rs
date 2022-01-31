use core::mem::{self, MaybeUninit};
use alloc::boxed::Box;
use crate::AppStateInCallbacks;

//////////////////
// Generic trait
//////////////////

pub trait InputDeviceEvent {
    fn populate_lv_indev_data(&self, data: &mut lvgl_sys::lv_indev_data_t);
    fn input_device_type() -> lvgl_sys::lv_indev_type_t;
}

pub fn register_input_device<F, I, S>(
    disp: &mut lvgl_sys::lv_disp_t,
    mut event_generator: F,
) where
    // We require static because we don't want any references in the closure to disappear
    F: Fn(&mut S) -> I + 'static,
    I: InputDeviceEvent,
{
    unsafe {
        let mut indev_drv = {
            let mut indev_drv = MaybeUninit::<lvgl_sys::lv_indev_drv_t>::uninit();
            lvgl_sys::lv_indev_drv_init(indev_drv.as_mut_ptr());
            let mut indev_drv = Box::new(indev_drv.assume_init());
            indev_drv.type_ = <I as InputDeviceEvent>::input_device_type();
            indev_drv.read_cb = Some(indev_read_cb::<F, I, S>);
            indev_drv.disp = disp;
            indev_drv.user_data = mem::transmute(&mut event_generator);
            indev_drv
        };
        lvgl_sys::lv_indev_drv_register(indev_drv.as_mut());

        // This will leak memory on drop
        Box::into_raw(indev_drv);
    }
}


unsafe extern "C" fn indev_read_cb<F, I, S>(
    drv: *mut lvgl_sys::lv_indev_drv_t,
    data: *mut lvgl_sys::lv_indev_data_t,
) where
    F: Fn(&mut S) -> I + 'static,
    I: InputDeviceEvent,
{
    let drv = drv.as_mut().unwrap();
    let data = data.as_mut().unwrap();

    let event_generator_ptr: *mut F = mem::transmute(drv.user_data);
    let event_generator = event_generator_ptr.as_mut().unwrap();

    let event = event_generator(AppStateInCallbacks::global().as_mut());
    event.populate_lv_indev_data(data);
}


//////////////////
// Touchpad
//////////////////

// Users can add other types (e.g., keyboard) like this one.

pub enum TouchPad {
    Pressed {
        x: lvgl_sys::lv_coord_t,
        y: lvgl_sys::lv_coord_t,
    },
    Released,
}

impl InputDeviceEvent for TouchPad {
    fn populate_lv_indev_data(&self, data: &mut lvgl_sys::lv_indev_data_t) {
        match &self {
            TouchPad::Pressed { x, y } => {
                data.point.x = *x;
                data.point.y = *y;
                data.state = lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED;
            }
            TouchPad::Released => {
                data.state = lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED;
            }
        }
    }

    fn input_device_type() -> lvgl_sys::lv_indev_type_t {
        lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_POINTER
    }
}
