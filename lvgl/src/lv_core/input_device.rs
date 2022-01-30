struct InputDevice {
    _indev_drv: Box<lvgl_sys::lv_indev_drv_t>,
    indev: *mut lvgl_sys::lv_indev_t,
}

impl InputDevice {

}


/*
ui.disp_drv_register(display).unwrap();
let mut screen = ui.scr_act().unwrap();

struct TouchGlobal {
    touch_screen: TouchScreen,
}

static mut TOUCH: Option<TouchGlobal> = None;
unsafe {
    //let delay = core::mem::transmute_copy(&delay);
    TOUCH = Some(TouchGlobal { touch_screen });
}


let indev_drv = unsafe {
    let mut indev_drv = MaybeUninit::<lvgl_sys::lv_indev_drv_t>::uninit();
    lvgl_sys::lv_indev_drv_init(indev_drv.as_mut_ptr());
    let mut indev_drv = indev_drv.assume_init();
    indev_drv.type_ = lvgl_sys::LV_INDEV_TYPE_POINTER as u8;
    indev_drv.read_cb = Some(input_read_cb);
    lvgl_sys::lv_indev_drv_register(&mut indev_drv as *mut lvgl_sys::lv_indev_drv_t);
    indev_drv
};


unsafe extern "C" fn input_read_cb(drv: *mut lvgl_sys::lv_indev_drv_t, data: *mut lvgl_sys::lv_indev_data_t) -> bool {
    let t = TOUCH.as_mut().unwrap();


    if let Some((x,y)) = t.touch_screen.read_x_y(&mut t.delay) {
        (*data).point.x = x as i16;
        (*data).point.y = y as i16;
        (*data).state = lvgl_sys::LV_INDEV_STATE_PR as u8;
    } else {
        (*data).state = lvgl_sys::LV_INDEV_STATE_REL as u8;
    }

    false
}

*/
