use conrod;
use self::conrod::backend::glium as gliumbackend;
use self::gliumbackend::glium;
use self::glium::glutin;
use self::glutin::Event;
use self::glutin::ControlFlow;

use failure::Error;

widget_ids! {
    struct Ids {
        text,
    }
}

fn handle_event(event: Event) -> ControlFlow {
    use self::glutin::WindowEvent;

    match event {
        Event::WindowEvent { event: WindowEvent::Closed, .. } => ControlFlow::Break,
        _ => ControlFlow::Continue
    }
}

pub fn run() -> Result<(), Error> {
    use self::glutin::EventsLoop;
    use self::glutin::WindowBuilder;
    use self::glutin::ContextBuilder;
    use self::glium::Display;
    use self::gliumbackend::Renderer;
    use self::conrod::UiBuilder;
    use self::conrod::backend::winit;

    let mut events_loop = EventsLoop::new();

    let window  = WindowBuilder::new().with_title("Test");
    let context = ContextBuilder::new().with_vsync(true).with_multisampling(4);
    let display = Display::new(window, context, &events_loop).unwrap();

    let mut renderer = Renderer::new(&display)?;
    let mut ui = UiBuilder::new([500.0 as f64, 500.0 as f64]).build();
    let ids = &mut Ids::new(ui.widget_id_generator());
    events_loop.run_forever(| event | {
        match winit::convert_event(event.clone(), &display) {
            Some(input) => ui.handle_event(input),
            _ => ()
        };

        handle_event(event)
    });

    Ok(())
}
