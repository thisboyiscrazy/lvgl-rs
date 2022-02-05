use core::mem::{self, MaybeUninit};
use alloc::boxed::Box;
use super::Display;

//////////////////
// Generic trait
//////////////////

pub trait InputDeviceState {
    fn populate_lv_indev_data(&self, data: &mut lvgl_sys::lv_indev_data_t);
    fn input_device_type() -> lvgl_sys::lv_indev_type_t;
}

pub struct InputDevice<S> {
    state: Box<S>,
}

impl<S: InputDeviceState + Default> InputDevice<S> {
    pub fn new<D>(display: &mut Display<D>) -> Self {
        let mut state = Box::new(S::default());

        unsafe {
            let mut indev_drv = {
                let mut indev_drv = MaybeUninit::<lvgl_sys::lv_indev_drv_t>::uninit();
                lvgl_sys::lv_indev_drv_init(indev_drv.as_mut_ptr());
                let mut indev_drv = Box::new(indev_drv.assume_init());
                indev_drv.type_ = <S as InputDeviceState>::input_device_type();
                indev_drv.read_cb = Some(Self::indev_read_cb);
                indev_drv.disp = display.disp;
                indev_drv.user_data = mem::transmute(state.as_mut());
                indev_drv
            };
            let _indev = lvgl_sys::lv_indev_drv_register(indev_drv.as_mut());

            // lvgl needs indev_drv to stick around
            Box::into_raw(indev_drv);
        }

        Self { state }
    }

    // We could add a feature to run a user-provided closure when lvgl polls the
    // device so we don't lose events, but not a priority at this point.

    unsafe extern "C" fn indev_read_cb(
        drv: *mut lvgl_sys::lv_indev_drv_t,
        data: *mut lvgl_sys::lv_indev_data_t,
    ) {
        let drv = drv.as_mut().unwrap();
        let data = data.as_mut().unwrap();

        let state_ptr: *mut S = mem::transmute(drv.user_data);
        let state = state_ptr.as_mut().unwrap();

        state.populate_lv_indev_data(data);
    }

    pub fn state(&mut self) -> &mut S {
        self.state.as_mut()
    }
}

impl<S> Drop for InputDevice<S> {
    fn drop(&mut self) {
        panic!("InputDevice can't be dropped");
    }
}


//////////////////
// Touchpad
//////////////////

// Users can add other types (e.g., keyboard) like this one.

#[derive(Debug)]
pub enum TouchPad {
    Released,
    Pressed {
        x: lvgl_sys::lv_coord_t,
        y: lvgl_sys::lv_coord_t,
    },
}

impl Default for TouchPad {
    fn default() -> Self {
        Self::Released
    }
}

impl InputDeviceState for TouchPad {
    fn populate_lv_indev_data(&self, data: &mut lvgl_sys::lv_indev_data_t) {
        match &self {
            TouchPad::Pressed { x, y } => {
                data.point.x = *x;
                data.point.y = *y;
                data.state = lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_PRESSED;
            }
            TouchPad::Released => {
                data.state = lvgl_sys::lv_indev_state_t_LV_INDEV_STATE_RELEASED;
            }
        }
    }

    fn input_device_type() -> lvgl_sys::lv_indev_type_t {
        lvgl_sys::lv_indev_type_t_LV_INDEV_TYPE_POINTER
    }
}
