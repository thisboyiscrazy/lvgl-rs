bitflags! {
    pub struct Flag: lvgl_sys::lv_obj_flag_t {
        const HIDDEN = lvgl_sys::LV_OBJ_FLAG_HIDDEN;
        const CLICKABLE = lvgl_sys::LV_OBJ_FLAG_CLICKABLE;
        const CLICK_FOCUSABLE = lvgl_sys::LV_OBJ_FLAG_CLICK_FOCUSABLE;
        const CHECKABLE = lvgl_sys::LV_OBJ_FLAG_CHECKABLE;
        const SCROLLABLE = lvgl_sys::LV_OBJ_FLAG_SCROLLABLE;
        const SCROLL_ELASTIC = lvgl_sys::LV_OBJ_FLAG_SCROLL_ELASTIC;
        const SCROLL_MOMENTUM = lvgl_sys::LV_OBJ_FLAG_SCROLL_MOMENTUM;
        const SCROLL_ONE = lvgl_sys::LV_OBJ_FLAG_SCROLL_ONE;
        const SCROLL_CHAIN_HOR = lvgl_sys::LV_OBJ_FLAG_SCROLL_CHAIN_HOR;
        const SCROLL_CHAIN_VER = lvgl_sys::LV_OBJ_FLAG_SCROLL_CHAIN_VER;
        const SCROLL_CHAIN = lvgl_sys::LV_OBJ_FLAG_SCROLL_CHAIN;
        const SCROLL_ON_FOCUS = lvgl_sys::LV_OBJ_FLAG_SCROLL_ON_FOCUS;
        const SCROLL_WITH_ARROW = lvgl_sys::LV_OBJ_FLAG_SCROLL_WITH_ARROW;
        const SNAPPABLE = lvgl_sys::LV_OBJ_FLAG_SNAPPABLE;
        const PRESS_LOCK = lvgl_sys::LV_OBJ_FLAG_PRESS_LOCK;
        const EVENT_BUBBLE = lvgl_sys::LV_OBJ_FLAG_EVENT_BUBBLE;
        const GESTURE_BUBBLE = lvgl_sys::LV_OBJ_FLAG_GESTURE_BUBBLE;
        const ADV_HITTEST = lvgl_sys::LV_OBJ_FLAG_ADV_HITTEST;
        const IGNORE_LAYOUT = lvgl_sys::LV_OBJ_FLAG_IGNORE_LAYOUT;
        const FLOATING = lvgl_sys::LV_OBJ_FLAG_FLOATING;
        const OVERFLOW_VISIBLE = lvgl_sys::LV_OBJ_FLAG_OVERFLOW_VISIBLE;
        const LAYOUT_1 = lvgl_sys::LV_OBJ_FLAG_LAYOUT_1;
        const LAYOUT_2 = lvgl_sys::LV_OBJ_FLAG_LAYOUT_2;
        const WIDGET_1 = lvgl_sys::LV_OBJ_FLAG_WIDGET_1;
        const WIDGET_2 = lvgl_sys::LV_OBJ_FLAG_WIDGET_2;
        const USER_1 = lvgl_sys::LV_OBJ_FLAG_USER_1;
        const USER_2 = lvgl_sys::LV_OBJ_FLAG_USER_2;
        const USER_3 = lvgl_sys::LV_OBJ_FLAG_USER_3;
        const USER_4 = lvgl_sys::LV_OBJ_FLAG_USER_4;
    }
}
