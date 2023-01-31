use cstr_core::CString;
use embedded_graphics_core::pixelcolor::Rgb565;
use embedded_graphics_core::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl::style::Style;
use lvgl::widgets::{Btnmatrix};
use lvgl::core::Screen::UI;
use lvgl::style::Align;
use lvgl_sys;
use std::time::Instant;
use lvgl::cstr_core::CStr;

fn main() -> Result<(),Err> {

    use lvgl::widgets::*;
    use lvgl::style::*;
    use lvgl::core::*;

    let display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(
        lvgl_sys::LV_HOR_RES_MAX,
        lvgl_sys::LV_VER_RES_MAX,
    ));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Btn matrix", &output_settings);

    let mut ui = UI::init()?;

    // Implement and register your display:
    ui.disp_drv_register(display).unwrap();

    // Create screen and widgets
    let mut screen = ui.scr_act()?;

    let spacing = 12;

    let spacing = 12;

    let mut m = [
        CStr::from_bytes_with_nul(b"0.1mm\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"1mm\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"10mm\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"\n\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"UP\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"HOME\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"DOWN\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"\n\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"STOP\0").unwrap().as_ptr(),
        CStr::from_bytes_with_nul(b"\0").unwrap().as_ptr(),
    ];
        
    let ptr = m.as_mut_ptr();

    let btnm = Btnmatrix::new(screen).apply(|obj| {
        obj
        .set_map(ptr)
        .align_to(screen, Align::TopLeft, 0, 0)
        .set_size(320, 200)
        .on_event(Event::ValueChanged, |context| {
            
        });
    });

    let mut loop_started = Instant::now();
    'running: loop {    
        ui.task_handler();
        window.update(ui.get_display_ref().unwrap());

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        ui.tick_inc(loop_started.elapsed());
    }

    Ok(())
}
