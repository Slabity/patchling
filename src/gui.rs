use conrod;
use conrod::backend::glium::glium;

use result::*;

fn handle_event(event: glium::glutin::Event) -> glium::glutin::ControlFlow {
    println!("{:?}", event);
    match event {
        glium::glutin::Event::WindowEvent { event, .. } => match event {
            glium::glutin::WindowEvent::Closed => {
                glium::glutin::ControlFlow::Break
            },
            _ => glium::glutin::ControlFlow::Continue
        },
        _ => glium::glutin::ControlFlow::Continue
    }
}

pub fn run() -> Result<()> {
    let mut events_loop = glium::glutin::EventsLoop::new();

    let window = {
        glium::glutin::WindowBuilder::new()
            .with_title("Test")
    };

    let context = {
        glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4)
    };

    let display = {
        glium::Display::new(window, context, &events_loop)
    };

    let mut ui = conrod::UiBuilder::new([500 as f64, 500 as f64]).build();
    events_loop.run_forever(handle_event);

    Ok(())
}
