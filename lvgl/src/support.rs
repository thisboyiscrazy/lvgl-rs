use crate::{Obj, Widget};
use core::convert::TryInto;
use core::ptr::NonNull;
use embedded_graphics_core::pixelcolor::{Rgb565, Rgb888};

pub type LvResult<T> = Result<T, LvError>;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum LvError {
    InvalidReference,
    Uninitialized,
    LvOOMemory,
    AlreadyInUse,
}

#[derive(Clone)]
pub struct Color {
    pub(crate) raw: lvgl_sys::lv_color_t,
}

impl Color {
    // Normally, LV_COLOR_MAKE is a C macro. Calling our shim can make things slow.
    pub fn from_rgb((r, g, b): (u8, u8, u8)) -> Self {
        let raw = unsafe { lvgl_sys::_LV_COLOR_MAKE(r, g, b) };
        Self { raw }
    }

    pub fn from_raw(raw: lvgl_sys::lv_color_t) -> Self {
        Self { raw }
    }

    pub fn r(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_R(self.raw) as u8 }
    }

    pub fn g(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_G(self.raw) as u8 }
    }

    pub fn b(&self) -> u8 {
        unsafe { lvgl_sys::_LV_COLOR_GET_B(self.raw) as u8 }
    }
}

impl From<Color> for Rgb888 {
    fn from(color: Color) -> Self {
        unsafe {
            Rgb888::new(
                lvgl_sys::_LV_COLOR_GET_R(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_G(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_B(color.raw) as u8,
            )
        }
    }
}

impl From<Color> for Rgb565 {
    fn from(color: Color) -> Self {
        unsafe {
            Rgb565::new(
                lvgl_sys::_LV_COLOR_GET_R(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_G(color.raw) as u8,
                lvgl_sys::_LV_COLOR_GET_B(color.raw) as u8,
            )
        }
    }
}


// Adapted from https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer
macro_rules! native_enum {
    ($native_type:ty,
        $(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val as isize)?,)*
        }

        impl core::convert::TryFrom<$native_type> for $name {
            type Error = ();

            fn try_from(v: $native_type) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as $native_type => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }

        impl From<$name> for $native_type {
            fn from(v: $name) -> Self {
                v as $native_type
            }
        }
    }
}

native_enum! {
    lvgl_sys::lv_event_code_t,
    /// Events are triggered in LVGL when something happens which might be interesting to
    /// the user, e.g. if an object:
    ///  - is clicked
    ///  - is dragged
    ///  - its value has changed, etc.
    ///
    /// All objects (such as Buttons/Labels/Sliders etc.) receive these generic events
    /// regardless of their type.
    pub enum Event {
        /** Input device events*/
        /// The object has been pressed
        Pressed = lvgl_sys::lv_event_code_t_LV_EVENT_PRESSED,
        /// The object is being pressed (called continuously while pressing)
        Pressing = lvgl_sys::lv_event_code_t_LV_EVENT_PRESSING,
        /// The object is still being pressed but slid cursor/finger off of the object
        PressLost = lvgl_sys::lv_event_code_t_LV_EVENT_PRESS_LOST,
        /// The object was pressed for a short period of time, then released it. Not called if scrolled.
        ShortClicked = lvgl_sys::lv_event_code_t_LV_EVENT_SHORT_CLICKED,
        /// Object has been pressed for at least `long_press_time`.  Not called if scrolled.
        LongPressed = lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED,
        /// Called after `long_press_time` in every `long_press_repeat_time` ms.  Not called if scrolled.
        LongPressedRepeat = lvgl_sys::lv_event_code_t_LV_EVENT_LONG_PRESSED_REPEAT,
        /// Called on release if not scrolled (regardless to long press)
        Clicked = lvgl_sys::lv_event_code_t_LV_EVENT_CLICKED,
        /// Called in every cases when the object has been released
        Released = lvgl_sys::lv_event_code_t_LV_EVENT_RELEASED,
        /// Scrolling begins
        ScrollBegin = lvgl_sys::lv_event_code_t_LV_EVENT_SCROLL_BEGIN,
        /// Scrolling ends
        ScrollEnd = lvgl_sys::lv_event_code_t_LV_EVENT_SCROLL_END,
        /// Scrolling
        Scroll = lvgl_sys::lv_event_code_t_LV_EVENT_SCROLL,
        /// A gesture is detected. Get the gesture with `lv_indev_get_gesture_dir(lv_indev_get_act());`
        Gesture = lvgl_sys::lv_event_code_t_LV_EVENT_GESTURE,
        /// A key is sent to the object. Get the key with `lv_indev_get_key(lv_indev_get_act());`
        Key = lvgl_sys::lv_event_code_t_LV_EVENT_KEY,
        /// The object is focused
        Focused = lvgl_sys::lv_event_code_t_LV_EVENT_FOCUSED,
        /// The object is defocused
        Defocused = lvgl_sys::lv_event_code_t_LV_EVENT_DEFOCUSED,
        /// The object is defocused but still selected
        Leave = lvgl_sys::lv_event_code_t_LV_EVENT_LEAVE,
        /// Perform advanced hit-testing
        HitTest = lvgl_sys::lv_event_code_t_LV_EVENT_HIT_TEST,

        /** Drawing events*/
        /// Check if the object fully covers an area. The event parameter is `lv_cover_check_info_t *`.
        CoverCheck = lvgl_sys::lv_event_code_t_LV_EVENT_COVER_CHECK,
        /// Get the required extra draw area around the object (e.g. for shadow). The event parameter is `lv_coord_t *` to store the size.
        RefrExtDrawSize = lvgl_sys::lv_event_code_t_LV_EVENT_REFR_EXT_DRAW_SIZE,
        /// Starting the main drawing phase
        DrawMainBegin = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN,
        /// Perform the main drawing
        DrawMain = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN,
        /// Finishing the main drawing phase
        DrawMainEnd = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_MAIN_END,
        /// Starting the post draw phase (when all children are drawn)
        DrawPostBegin = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_BEGIN,
        /// Perform the post draw phase (when all children are drawn)
        DrawPost = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST,
        /// Finishing the post draw phase (when all children are drawn)
        DrawPostEnd = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_POST_END,
        /// Starting to draw a part. The event parameter is `lv_obj_draw_dsc_t *`.
        DrawPartBegin = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_PART_BEGIN,
        /// Finishing to draw a part. The event parameter is `lv_obj_draw_dsc_t *`.
        DrawPartEnd = lvgl_sys::lv_event_code_t_LV_EVENT_DRAW_PART_END,

        /** Special events*/
        /// The object's value has changed (i.e. slider moved)
        ValueChanged = lvgl_sys::lv_event_code_t_LV_EVENT_VALUE_CHANGED,
        /// A text is inserted to the object. The event data is `char *` being inserted.
        Insert = lvgl_sys::lv_event_code_t_LV_EVENT_INSERT,
        /// Notify the object to refresh something on it (for the user)
        Refresh = lvgl_sys::lv_event_code_t_LV_EVENT_REFRESH,
        /// A process has finished
        Ready = lvgl_sys::lv_event_code_t_LV_EVENT_READY,
        /// A process has been cancelled
        Cancel = lvgl_sys::lv_event_code_t_LV_EVENT_CANCEL,

        /** Other events*/
        /// Object is being deleted
        Delete = lvgl_sys::lv_event_code_t_LV_EVENT_DELETE,
        /// Child was removed, added, or its size, position were changed
        ChildChanged = lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CHANGED,
        /// Child was created, always bubbles up to all parents
        ChildCreated = lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_CREATED,
        /// Child was deleted, always bubbles up to all parents
        ChildDeleted = lvgl_sys::lv_event_code_t_LV_EVENT_CHILD_DELETED,
        /// A screen unload started, fired immediately when scr_load is called
        ScreenUnloadStart = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOAD_START,
        /// A screen load started, fired when the screen change delay is expired
        ScreenLoadStart = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOAD_START,
        /// A screen was loaded
        ScreenLoaded = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_LOADED,
        /// A screen was unloaded
        ScreenUnloaded = lvgl_sys::lv_event_code_t_LV_EVENT_SCREEN_UNLOADED,
        /// Object coordinates/size have changed
        SizeChanged = lvgl_sys::lv_event_code_t_LV_EVENT_SIZE_CHANGED,
        /// Object's style has changed
        StyleChanged = lvgl_sys::lv_event_code_t_LV_EVENT_STYLE_CHANGED,
        /// The children position has changed due to a layout recalculation
        LayoutChanged = lvgl_sys::lv_event_code_t_LV_EVENT_LAYOUT_CHANGED,
        /// Get the internal size of a widget
        GetSelfSize = lvgl_sys::lv_event_code_t_LV_EVENT_GET_SELF_SIZE,
    }
}

pub(crate) unsafe extern "C" fn event_callback<T, F>(
    event: *mut lvgl_sys::lv_event_t,
) where
    T: Widget + Sized,
    F: FnMut(T, Event, Option<Obj>),
{
    // Seems a bit silly to use functions to access fields, but that's what the
    // libary example show.

    let event_code = lvgl_sys::lv_event_get_code(event);
    let current_target = lvgl_sys::lv_event_get_current_target(event);
    let target = lvgl_sys::lv_event_get_target(event);
    let user_data = lvgl_sys::lv_event_get_user_data(event);

    if let Ok(event_code) = event_code.try_into() {
        if let Some(current_target) = NonNull::new(current_target) {
            if let Some(target) = NonNull::new(target) {
                // current_target is always the object on which .on_event() was called.
                // (So it can be casted to T)
                // target can either be the same object, or a child object,
                // when LV_OBJ_FLAG_EVENT_BUBBLE is set on the child.
                let child = if current_target != target {
                    Some(Obj::from_raw(target))
                } else {
                    None
                };

                let current_target = T::from_raw(current_target);
                let user_closure = &mut *(user_data as *mut F);
                user_closure(current_target, event_code, child);
            }
        }
    }
}

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

#[cfg(test)]
mod test {
    use super::*;
    use lvgl_sys;

    #[test]
    fn color_properties_accessible() {
        let color = Color::from_rgb((206, 51, 255));

        if lvgl_sys::LV_COLOR_DEPTH == 32 {
            assert_eq!(color.r(), 206);
            assert_eq!(color.g(), 51);
            assert_eq!(color.b(), 255);
        } else if lvgl_sys::LV_COLOR_DEPTH == 16 {
            assert_eq!(color.r(), 25);
            assert_eq!(color.g(), 12);
            assert_eq!(color.b(), 31);
        }
    }
}
