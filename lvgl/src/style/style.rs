use alloc::boxed::Box;
use core::mem;

pub enum Themes {
    Pretty,
}

pub struct Style {
    pub(crate) _raw: Box<lvgl_sys::lv_style_t>,
}

/*
use crate::{Color, LvResult, State};
use cstr_core::CStr;
impl Style {
    pub fn set_value_str(&mut self, state: State, value: &CStr) -> LvResult<()> {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_ptr(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_STR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.as_ptr() as *mut cty::c_void,
            );
        }
        Ok(())
    }
}
*/

impl Default for Style {
    fn default() -> Self {
        let raw = unsafe {
            let mut style = mem::MaybeUninit::<lvgl_sys::lv_style_t>::uninit();
            lvgl_sys::lv_style_init(style.as_mut_ptr());
            Box::new(style.assume_init())
        };
        Self { _raw: raw }
    }
}

bitflags! {
    pub struct Opacity: u8 {
        const OPA_TRANSP = lvgl_sys::LV_OPA_TRANSP;
        const OPA_0 = lvgl_sys::LV_OPA_0;
        const OPA_10 = lvgl_sys::LV_OPA_10;
        const OPA_20 = lvgl_sys::LV_OPA_20;
        const OPA_30 = lvgl_sys::LV_OPA_30;
        const OPA_40 = lvgl_sys::LV_OPA_40;
        const OPA_50 = lvgl_sys::LV_OPA_50;
        const OPA_60 = lvgl_sys::LV_OPA_60;
        const OPA_70 = lvgl_sys::LV_OPA_70;
        const OPA_80 = lvgl_sys::LV_OPA_80;
        const OPA_90 = lvgl_sys::LV_OPA_90;
        const OPA_100 = lvgl_sys::LV_OPA_100;
        const OPA_COVER = lvgl_sys::LV_OPA_COVER;
    }
}

impl Into<u8> for Opacity {
    fn into(self) -> u8 {
        self.bits as u8
    }
}

bitflags! {
    pub struct StyleProp: u16 {
        const PROP_INV = lvgl_sys::lv_style_prop_t_LV_STYLE_PROP_INV;

        /*Group 0*/
        const WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_WIDTH;
        const MIN_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_MIN_WIDTH;
        const MAX_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_MAX_WIDTH;
        const HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_HEIGHT;
        const MIN_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_MIN_HEIGHT;
        const MAX_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_MAX_HEIGHT;
        const X = lvgl_sys::lv_style_prop_t_LV_STYLE_X;
        const Y = lvgl_sys::lv_style_prop_t_LV_STYLE_Y;
        const ALIGN = lvgl_sys::lv_style_prop_t_LV_STYLE_ALIGN;
        const TRANSFORM_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_WIDTH;
        const TRANSFORM_HEIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_HEIGHT;
        const TRANSLATE_X = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSLATE_X;
        const TRANSLATE_Y = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSLATE_Y;
        const TRANSFORM_ZOOM = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_ZOOM;
        const TRANSFORM_ANGLE = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSFORM_ANGLE;

        /*Group 1*/
        const PAD_TOP = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_TOP;
        const PAD_BOTTOM = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_BOTTOM;
        const PAD_LEFT = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_LEFT;
        const PAD_RIGHT = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_RIGHT;
        const PAD_ROW = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_ROW;
        const PAD_COLUMN = lvgl_sys::lv_style_prop_t_LV_STYLE_PAD_COLUMN;

        /*Group 2*/
        const BG_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_COLOR;
        const BG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_OPA;
        const BG_GRAD_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_COLOR;
        const BG_GRAD_DIR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_DIR;
        const BG_MAIN_STOP = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_MAIN_STOP;
        const BG_GRAD_STOP = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_GRAD_STOP;

        const BG_IMG_SRC = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_SRC;
        const BG_IMG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_OPA;
        const BG_IMG_RECOLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR;
        const BG_IMG_RECOLOR_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_RECOLOR_OPA;
        const BG_IMG_TILED = lvgl_sys::lv_style_prop_t_LV_STYLE_BG_IMG_TILED;

        /*Group 3*/
        const BORDER_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_COLOR;
        const BORDER_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_OPA;
        const BORDER_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_WIDTH;
        const BORDER_SIDE = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_SIDE;
        const BORDER_POST = lvgl_sys::lv_style_prop_t_LV_STYLE_BORDER_POST;

        const OUTLINE_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_WIDTH;
        const OUTLINE_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_COLOR;
        const OUTLINE_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_OPA;
        const OUTLINE_PAD = lvgl_sys::lv_style_prop_t_LV_STYLE_OUTLINE_PAD;

        /*Group 4*/
        const SHADOW_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_WIDTH;
        const SHADOW_OFS_X = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_X;
        const SHADOW_OFS_Y = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OFS_Y;
        const SHADOW_SPREAD = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_SPREAD;
        const SHADOW_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_COLOR;
        const SHADOW_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_SHADOW_OPA;

        const IMG_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_OPA;
        const IMG_RECOLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR;
        const IMG_RECOLOR_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_IMG_RECOLOR_OPA;

        const LINE_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_WIDTH;
        const LINE_DASH_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_DASH_WIDTH;
        const LINE_DASH_GAP = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_DASH_GAP;
        const LINE_ROUNDED = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_ROUNDED;
        const LINE_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_COLOR;
        const LINE_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_LINE_OPA;

        /*Group 5*/
        const ARC_WIDTH = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_WIDTH;
        const ARC_ROUNDED = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_ROUNDED;
        const ARC_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_COLOR;
        const ARC_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_OPA;
        const ARC_IMG_SRC = lvgl_sys::lv_style_prop_t_LV_STYLE_ARC_IMG_SRC;

        const TEXT_COLOR = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_COLOR;
        const TEXT_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_OPA;
        const TEXT_FONT = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_FONT;
        const TEXT_LETTER_SPACE = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_LETTER_SPACE;
        const TEXT_LINE_SPACE = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_LINE_SPACE;
        const TEXT_DECOR = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_DECOR;
        const TEXT_ALIGN = lvgl_sys::lv_style_prop_t_LV_STYLE_TEXT_ALIGN;

        /*Group 6*/
        const RADIUS = lvgl_sys::lv_style_prop_t_LV_STYLE_RADIUS;
        const CLIP_CORNER = lvgl_sys::lv_style_prop_t_LV_STYLE_CLIP_CORNER;
        const OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_OPA;
        const COLOR_FILTER_DSC = lvgl_sys::lv_style_prop_t_LV_STYLE_COLOR_FILTER_DSC;
        const COLOR_FILTER_OPA = lvgl_sys::lv_style_prop_t_LV_STYLE_COLOR_FILTER_OPA;
        const ANIM_TIME = lvgl_sys::lv_style_prop_t_LV_STYLE_ANIM_TIME;
        const ANIM_SPEED = lvgl_sys::lv_style_prop_t_LV_STYLE_ANIM_SPEED;
        const TRANSITION = lvgl_sys::lv_style_prop_t_LV_STYLE_TRANSITION;
        const BLEND_MODE = lvgl_sys::lv_style_prop_t_LV_STYLE_BLEND_MODE;
        const LAYOUT = lvgl_sys::lv_style_prop_t_LV_STYLE_LAYOUT;
        const BASE_DIR = lvgl_sys::lv_style_prop_t_LV_STYLE_BASE_DIR;

        const PROP_ANY = lvgl_sys::lv_style_prop_t_LV_STYLE_PROP_ANY;

    }
}

// Auto-gen code, please look into lvgl-codegen for any changes.
impl Style {
    /*
    pub fn set_radius(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_RADIUS | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32))
                    as u16,
                value,
            );
        }
    }

    pub fn set_clip_corner(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_CLIP_CORNER
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_size(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SIZE | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32))
                    as u16,
                value,
            );
        }
    }

    pub fn set_transform_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transform_height(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_HEIGHT
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transform_angle(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_ANGLE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transform_zoom(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSFORM_ZOOM
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_opa_scale(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OPA_SCALE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_pad_top(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_TOP | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32))
                    as u16,
                value,
            );
        }
    }

    pub fn set_pad_bottom(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_BOTTOM
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pad_left(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_LEFT
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pad_right(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_RIGHT
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pad_inner(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PAD_INNER
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_top(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_TOP
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_bottom(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_BOTTOM
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_left(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_LEFT
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_margin_right(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_MARGIN_RIGHT
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_main_stop(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_MAIN_STOP
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_grad_stop(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_GRAD_STOP
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_grad_dir(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_GRAD_DIR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_bg_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_bg_grad_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_GRAD_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_bg_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BG_OPA | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32))
                    as u16,
                value.into(),
            );
        }
    }

    pub fn set_border_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_side(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_SIDE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_post(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_POST
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_border_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_border_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_BORDER_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_outline_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_outline_pad(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_PAD
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_outline_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_outline_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_outline_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_OUTLINE_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_shadow_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_ofs_x(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_OFS_X
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_ofs_y(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_OFS_Y
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_spread(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_SPREAD
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_shadow_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_shadow_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SHADOW_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_pattern_repeat(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_REPEAT
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pattern_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_pattern_recolor(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_RECOLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_pattern_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_pattern_recolor_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_PATTERN_RECOLOR_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_value_letter_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_LETTER_SPACE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_line_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_LINE_SPACE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_ofs_x(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_OFS_X
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_ofs_y(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_OFS_Y
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_align(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_ALIGN
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_value_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_value_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_VALUE_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_text_letter_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_LETTER_SPACE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_line_space(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_LINE_SPACE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_decor(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_DECOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_text_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_text_sel_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_SEL_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_text_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TEXT_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_line_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_dash_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_DASH_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_dash_gap(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_DASH_GAP
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_rounded(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_ROUNDED
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_line_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_line_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_LINE_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_image_blend_mode(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_BLEND_MODE
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_image_recolor(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_RECOLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_image_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_image_recolor_opa(&mut self, state: State, value: Opacity) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_opa(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_IMAGE_RECOLOR_OPA
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.into(),
            );
        }
    }

    pub fn set_transition_time(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_TIME
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_delay(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_DELAY
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_1(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_1
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_2(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_2
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_3(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_3
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_4(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_4
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_5(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_5
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_transition_prop_6(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_TRANSITION_PROP_6
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_border_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_BORDER_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_end_border_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_END_BORDER_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_end_line_width(&mut self, state: State, value: i16) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_int(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_END_LINE_WIDTH
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value,
            );
        }
    }

    pub fn set_scale_grad_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_GRAD_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }

    pub fn set_scale_end_color(&mut self, state: State, value: Color) {
        let native_state: u32 = state.get_bits();
        unsafe {
            lvgl_sys::_lv_style_set_color(
                self.raw.as_mut(),
                (lvgl_sys::LV_STYLE_SCALE_END_COLOR
                    | (native_state << lvgl_sys::LV_STYLE_STATE_POS as u32)) as u16,
                value.raw,
            );
        }
    }
    */
}
