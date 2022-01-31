pub enum Part {
    /// A background like rectangle
    Main,
    /// The scrollbar(s)
    Scrollbar,
    /// Indicator, e.g. for slider, bar, switch, or the tick box of the checkbox
    Indicator,
    /// Like handle to grab to adjust the value
    Knob,
    /// Indicate the currently selected option or section,
    Selected,
    /// Used if the widget has multiple similar elements (e.g. table cells)
    Items,
    /// Ticks on scale e.g. for a chart or meter
    Ticks,
    /// Mark a specific place e.g. for text area's cursor or on a chart
    Cursor,
    /// Extension point for custom widgets
    CustomFirst,
    /// Special value can be used in some functions to target all parts
    Any,
}

impl Default for Part {
    fn default() -> Self {
        Self::Main
    }
}

impl Into<lvgl_sys::lv_part_t> for Part {
    fn into(self) -> lvgl_sys::lv_part_t {
        match self {
            Part::Main => lvgl_sys::LV_PART_MAIN,
            Part::Scrollbar => lvgl_sys::LV_PART_SCROLLBAR,
            Part::Indicator => lvgl_sys::LV_PART_INDICATOR,
            Part::Knob => lvgl_sys::LV_PART_KNOB,
            Part::Selected => lvgl_sys::LV_PART_SELECTED,
            Part::Items => lvgl_sys::LV_PART_ITEMS,
            Part::Ticks => lvgl_sys::LV_PART_TICKS,
            Part::Cursor => lvgl_sys::LV_PART_CURSOR,
            Part::CustomFirst => lvgl_sys::LV_PART_CUSTOM_FIRST,
            Part::Any => lvgl_sys::LV_PART_ANY,
        }
    }
}
