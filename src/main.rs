extern crate winit;
extern crate cef_sys;

use winit::os::windows::WindowExt;

use std::process;

use cef::{Cef, InitErr};

fn main() {
    env_logger::init().unwrap();
    debug!("Starting...");
    let c = Cef::new();
    match c.init() {
        Ok(c) => c,
        Err(InitErr::ChildWithExitCode(exit_code)) => process::exit(exit_code),
        Err(InitErr::InitFail) => panic!("Unable to init")
    };

    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_title("winit")
        .with_dimensions(800, 600)
    .build(&events_loop).unwrap();

    let hwnd = window.get_hwnd();
    let (width, height) = window.get_inner_size_pixels().unwrap();
    let browser = c.create_browser(hwnd as winapi::HWND, width as _, height as _);

    events_loop.run_forever(|event| {
        c.tick();
        match event {
            winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => {
                winit::ControlFlow::Break
            },
            _ => winit::ControlFlow::Continue,
        }
    });

    c.shutdown();
}