use crate::*;

#[inline]
pub unsafe fn lv_canvas_set_px(canvas: *mut lv_obj_t, x: lv_coord_t, y: lv_coord_t, c: lv_color_t)
{
    lv_canvas_set_px_color(canvas, x, y, c);
}

#[inline]
pub unsafe fn lv_slider_set_value(obj: *mut lv_obj_t, value: i32, anim: lv_anim_enable_t)
{
    lv_bar_set_value(obj, value, anim);
}

#[inline]
pub unsafe fn lv_slider_set_left_value(obj: *mut lv_obj_t, value: i32, anim: lv_anim_enable_t)
{
    lv_bar_set_start_value(obj, value, anim);
}

#[inline]
pub unsafe fn lv_slider_set_range(obj: *mut lv_obj_t, min: i32, max: i32)
{
    lv_bar_set_range(obj, min, max);
}

#[inline]
pub unsafe fn lv_slider_set_mode(obj: *mut lv_obj_t, mode: lv_slider_mode_t)
{
    lv_bar_set_mode(obj, mode as lv_bar_mode_t);
}

#[inline]
pub unsafe fn lv_slider_get_value(obj: *const lv_obj_t) -> i32
{
    lv_bar_get_value(obj)
}

#[inline]
pub unsafe fn lv_slider_get_left_value(obj: *const lv_obj_t) -> i32
{
    lv_bar_get_start_value(obj)
}

#[inline]
pub unsafe fn lv_slider_get_min_value(obj: *const lv_obj_t) -> i32
{
    lv_bar_get_min_value(obj)
}

#[inline]
pub unsafe fn lv_slider_get_max_value(obj: *const lv_obj_t) -> i32
{
    lv_bar_get_max_value(obj)
}
