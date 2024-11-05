use glium::{glutin::api::glx::display, winit, Surface};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    loop {
        let mut frame = display.draw();
        frame.draw();
        frame.finish();
    }
}
