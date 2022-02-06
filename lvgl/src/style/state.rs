bitflags! {
    pub struct State: lvgl_sys::lv_state_t {
        /// Normal, released
        const DEFAULT  = lvgl_sys::LV_STATE_DEFAULT;
        /// Toggled or checked
        const CHECKED  = lvgl_sys::LV_STATE_CHECKED;
        /// Focused via keypad or encoder or clicked via touchpad/mouse
        const FOCUSED  = lvgl_sys::LV_STATE_FOCUSED;
        /// Focused via a keypad
        const FOCUS_KEY = lvgl_sys::LV_STATE_FOCUS_KEY;
        /// Edit by an encoder
        const EDITED   = lvgl_sys::LV_STATE_EDITED;
        /// Hovered by mouse (not supported now)
        const HOVERED  = lvgl_sys::LV_STATE_HOVERED;
        /// Pressed
        const PRESSED  = lvgl_sys::LV_STATE_PRESSED;
        /// SCrolled
        const SCROLLED = lvgl_sys::LV_STATE_SCROLLED;
        /// Disabled or inactive
        const DISABLED = lvgl_sys::LV_STATE_DISABLED;

        const USER_1 = lvgl_sys::LV_STATE_USER_1;
        const USER_2 = lvgl_sys::LV_STATE_USER_2;
        const USER_3 = lvgl_sys::LV_STATE_USER_3;
        const USER_4 = lvgl_sys::LV_STATE_USER_4;

        const ANY = lvgl_sys::LV_STATE_ANY;
    }
}

impl Default for State {
    fn default() -> Self {
        Self::DEFAULT
    }
}
