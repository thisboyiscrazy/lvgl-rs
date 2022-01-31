bitflags! {
    pub struct State: u16 {
        /// Normal, released
        const DEFAULT  = lvgl_sys::LV_STATE_DEFAULT as u16;
        /// Toggled or checked
        const CHECKED  = lvgl_sys::LV_STATE_CHECKED as u16;
        /// Focused via keypad or encoder or clicked via touchpad/mouse
        const FOCUSED  = lvgl_sys::LV_STATE_FOCUSED as u16;
        /// Focused via a keypad
        const FOCUS_KEY = lvgl_sys::LV_STATE_FOCUS_KEY as u16;
        /// Edit by an encoder
        const EDITED   = lvgl_sys::LV_STATE_EDITED as u16;
        /// Hovered by mouse (not supported now)
        const HOVERED  = lvgl_sys::LV_STATE_HOVERED as u16;
        /// Pressed
        const PRESSED  = lvgl_sys::LV_STATE_PRESSED as u16;
        /// SCrolled
        const SCROLLED = lvgl_sys::LV_STATE_SCROLLED as u16;
        /// Disabled or inactive
        const DISABLED = lvgl_sys::LV_STATE_DISABLED as u16;

        const USER_1 = lvgl_sys::LV_STATE_USER_1 as u16;
        const USER_2 = lvgl_sys::LV_STATE_USER_2 as u16;
        const USER_3 = lvgl_sys::LV_STATE_USER_3 as u16;
        const USER_4 = lvgl_sys::LV_STATE_USER_4 as u16;

        const ANY = lvgl_sys::LV_STATE_ANY as u16;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::DEFAULT
    }
}
