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

    let btn_move_up = Btn::new(screen).apply(|obj| {
        Label::new(obj)
            .set_text(&CStr::from_bytes_with_nul(b"Move Up\0").unwrap());
        obj
        .align_to(screen, Align::TopMid, 0, 2*spacing)
    });

    //const BTN_MAP: &[&'static str] = &[
    //    "0.1mm", "1mm", "10mm","\n",
    //    "UP","STOP","DOWN","\n",
    //    "HOME","",
    //];
//
    //let btnm = Btnmatrix::new(&mut screen).apply(|obj| {
    //    .align_to(screen, Align::TopMid, 0, 0)
    //});

    //btnm.set_align(&mut screen, Align::Center, 0, 10)?;
    
    
    
    // Create the bar object
    //let mut bar = Bar::new(&mut screen)?;

    //bar.set_size(175, 20)?;
    //bar.set_align(&mut screen, Align::Center, 0, 10)?;
    //bar.set_range(0, 100)?;
    //bar.on_event(|_b, _e| {
    //    println!("Completed!");
    //})?;
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
