native_enum! {
    lvgl_sys::lv_align_t,
    pub enum Align {
        Default = lvgl_sys::LV_ALIGN_DEFAULT,
        TopLeft = lvgl_sys::LV_ALIGN_TOP_LEFT,
        TopMid = lvgl_sys::LV_ALIGN_TOP_MID,
        TopRight = lvgl_sys::LV_ALIGN_TOP_RIGHT,
        BottomLeft = lvgl_sys::LV_ALIGN_BOTTOM_LEFT,
        BottomMid = lvgl_sys::LV_ALIGN_BOTTOM_MID,
        BottomRight = lvgl_sys::LV_ALIGN_BOTTOM_RIGHT,
        LeftMid = lvgl_sys::LV_ALIGN_LEFT_MID,
        RightMid = lvgl_sys::LV_ALIGN_RIGHT_MID,
        Center = lvgl_sys::LV_ALIGN_CENTER,
        OutTopLeft = lvgl_sys::LV_ALIGN_OUT_TOP_LEFT,
        OutTopMid = lvgl_sys::LV_ALIGN_OUT_TOP_MID,
        OutTopRight = lvgl_sys::LV_ALIGN_OUT_TOP_RIGHT,
        OutBottomLeft = lvgl_sys::LV_ALIGN_OUT_BOTTOM_LEFT,
        OutBottomMid = lvgl_sys::LV_ALIGN_OUT_BOTTOM_MID,
        OutBottomRight = lvgl_sys::LV_ALIGN_OUT_BOTTOM_RIGHT,
        OutLeftTop = lvgl_sys::LV_ALIGN_OUT_LEFT_TOP,
        OutLeftMid = lvgl_sys::LV_ALIGN_OUT_LEFT_MID,
        OutLeftBottom = lvgl_sys::LV_ALIGN_OUT_LEFT_BOTTOM,
        OutRightTop = lvgl_sys::LV_ALIGN_OUT_RIGHT_TOP,
        OutRightMid = lvgl_sys::LV_ALIGN_OUT_RIGHT_MID,
        OutRightBottom = lvgl_sys::LV_ALIGN_OUT_RIGHT_BOTTOM,
    }
}

native_enum! {
    lvgl_sys::lv_anim_enable_t,
    pub enum Animation {
        On = lvgl_sys::lv_anim_enable_t_LV_ANIM_ON,
        Off = lvgl_sys::lv_anim_enable_t_LV_ANIM_OFF,
    }
}